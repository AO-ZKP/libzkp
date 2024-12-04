use alloy_sol_types::{sol, SolValue};
use alloy_primitives::{keccak256, U256, hex, B256, aliases::U240};
use risc0_steel::Commitment;
use risc0_zkvm::{Receipt, sha::Digest};
use serde::{Serialize, Deserialize};
use std::ffi::{c_char, CStr, CString};
use std::convert::AsRef;

sol! {
    struct Journal {
        Commitment commitment;
        address from;
        uint256 amount;
        uint256 timestamp;
        uint256 nullifier;
    }

        // Define a new struct for the fields we want to hash
    struct JournalHashFields {
        address from;
        uint256 amount;
        uint256 timestamp;
        uint256 nullifier;
    }
}

#[derive(Serialize, Deserialize)]
struct Input {
    receipt: Receipt,
    withdraw: String,
    imageid: String,
}

#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Output {
    error: Option<String>,
    proof: bool,
    withdraw: bool,
    amount: String,
    timestamp: String,
    blocknumber: String,
    blockhash: String,
}

impl Output {
    fn failure(error_msg: &str) -> Self {
        Output {
            error: Some(error_msg.to_string()),
            proof: false,
            withdraw: false,
            amount: String::new(),
            timestamp: String::new(),
            blocknumber: String::new(),
            blockhash: String::new(),
        }
    }
}

#[no_mangle]
pub extern "C" fn keccak(input_ptr: *const c_char) -> *const c_char {
    // Validate input pointer
    if input_ptr.is_null() {
        return create_error_response("Invalid null input pointer");
    }

    // Convert C string to Rust string
    let c_str = unsafe {
        match CStr::from_ptr(input_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return create_error_response("Failed to decode input as UTF-8 string"),
        }
    };

    // Parse input JSON
    let input: Input = match serde_json::from_str(c_str) {
        Ok(v) => v,
        Err(_) => return create_error_response("Failed to parse input JSON"),
    };

    // Get the journal bytes from the receipt
    let journal_bytes: &Vec<u8> = &input.receipt.journal.bytes;
    
    // Decode the full journal first
    let journal: Journal = match Journal::abi_decode(journal_bytes, true) {
        Ok(j) => j,
        Err(_) => return create_error_response("Invalid journal data: Failed to decode journal bytes"),
    };

    // Create a new JournalHashFields struct with only the fields we want to hash
    let hash_fields = JournalHashFields {
        from: journal.from,
        amount: journal.amount,
        timestamp: journal.timestamp,
        nullifier: journal.nullifier,
    };

    // Encode only these fields
    let encoded_fields = hash_fields.abi_encode();
    
    // Calculate keccak hash of only the selected fields
    let hash: B256 = keccak256(&encoded_fields);
    
    // Convert hash to hex string with 0x prefix
    let hash_hex = format!("0x{}", hex::encode(hash));
    
    // Create response JSON
    let response = serde_json::json!({
        "error": String::new(),
        "hash": hash_hex
    });

    // Convert to C string and return
    CString::new(response.to_string())
        .unwrap_or_else(|_| CString::new(r#"{"error":"Failed to create response"}"#).unwrap())
        .into_raw()
}

fn create_error_response(msg: &str) -> *const c_char {
    let error_json = serde_json::json!({
        "error": msg,
        "hash": String::new()
    });
    CString::new(error_json.to_string())
        .unwrap_or_else(|_| CString::new(r#"{"error":"Failed to create error response"}"#).unwrap())
        .into_raw()
}

#[no_mangle]
pub extern "C" fn verify(input_ptr: *const c_char) -> *const c_char {
    // Validate input pointer
    if input_ptr.is_null() {
        return create_response(&Output::failure("Invalid null input pointer"));
    }

    // Convert C string to Rust string
    let c_str = unsafe {
        match CStr::from_ptr(input_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return create_response(&Output::failure("Failed to decode input as UTF-8 string")),
        }
    };

    // Parse input JSON
    let input: Input = match serde_json::from_str(c_str) {
        Ok(v) => v,
        Err(_) => return create_response(&Output::failure("Failed to parse input JSON")),
    };

    // Extract components
    let receipt: Receipt = input.receipt;
    let withdraw: String = input.withdraw;
    let image_id: Digest = match hex::decode(&input.imageid[2..]) {
        Ok(bytes) => match Digest::try_from(bytes.as_slice()) {
            Ok(digest) => digest,
            Err(_) => return create_response(&Output::failure("Failed to create digest from image ID")),
        },
        Err(_) => return create_response(&Output::failure("Failed to decode hex image ID")),
    };

    // Decode journal
    let journal: &Vec<u8> = &receipt.journal.bytes;
    let journal: Journal = match Journal::abi_decode(journal, true) {
        Ok(j) => j,
        Err(_) => return create_response(&Output::failure("Failed to decode journal data")),
    };

    // Verify nullifier
    let nullifier: alloy_primitives::Uint<256, 4> = journal.nullifier;
    let withdraw_hash: U256 = keccak256(withdraw.as_bytes()).into();
    let does_hash_match: bool = nullifier == withdraw_hash;

    if !does_hash_match {
        return create_response(&Output::failure("Nullifier verification failed: Hash mismatch"));
    }

    // Verify proof
    if let Err(_) = receipt.verify(image_id) {
        return create_response(&Output::failure("Receipt verification failed"));
    }

    // Extract commitment details
    let commitment_id: alloy_primitives::Uint<256, 4> = journal.commitment.id;

    // Create masks matching the Solidity implementation
    // 0x0000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff for ID
    let id_mask: alloy_primitives::Uint<256, 4> = U256::from_str_radix("0000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", 16).unwrap();

    // Extract version (top 16 bits, shifted right by 240)
    let version: u16 = ((commitment_id >> 240u32).as_limbs()[0] & 0xFFFF) as u16;

    // Extract block number using the bottom 240 bits
    let masked_value = commitment_id & id_mask;
    let block_number = U240::from(masked_value);

    if version != 0 {
        return create_response(&Output::failure("Invalid version number"));
    }

    // Get digest bytes and encode as hex
    let digest_bytes: &[u8] = journal.commitment.digest.as_ref();

    // Create successful output
    let output = Output {
        error: "".parse().ok(),
        proof: true,
        withdraw: true,
        amount: journal.amount.to_string(),
        timestamp: journal.timestamp.to_string(),
        blocknumber: block_number.to_string(),
        blockhash: format!("0x{}", hex::encode(digest_bytes)),
    };

    create_response(&output)
}

fn create_response(output: &Output) -> *const c_char {
    let json = serde_json::to_string(output).unwrap();
    CString::new(json).unwrap().into_raw()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_verify_function() {
        // Read the input JSON file
        let input_json = fs::read_to_string("input_fixed.json")
            .expect("Failed to read input.json");
        

        // Convert the input to a C string
        let c_input: CString = CString::new(input_json)
            .expect("Failed to create CString from input");
        
        // Call the verify function
        let result_ptr: *const i8 = verify(c_input.as_ptr());
        
        // Convert the result back to a Rust string
        let result = unsafe {
            CStr::from_ptr(result_ptr)
                .to_string_lossy()
                .into_owned()
        };

        println!("\nOutput JSON:");
        println!("{}", result);
        
        // Parse the output to make it pretty (optional)
        if let Ok(output_json) = serde_json::from_str::<serde_json::Value>(&result) {
            println!("\nFormatted Output:");
            println!("{}", serde_json::to_string_pretty(&output_json).unwrap());
        }
    }

    #[test]
    fn test_keccak_function() {
        // Read the test input file
        let input_json = fs::read_to_string("input_fixed.json")
            .expect("Failed to read input_fixed.json");

        // Convert input to C string
        let c_input = CString::new(input_json)
            .expect("Failed to create CString from input");
        
        // Call the keccak function
        let result_ptr = keccak(c_input.as_ptr());
        
        // Convert result back to Rust string
        let result = unsafe {
            CStr::from_ptr(result_ptr)
                .to_string_lossy()
                .into_owned()
        };

        // Parse the result
        let result_json: serde_json::Value = serde_json::from_str(&result)
            .expect("Failed to parse result JSON");

        // Verify the hash exists and is correctly formatted
        let hash = result_json["hash"].as_str().expect("Hash not found in result");
        assert!(hash.starts_with("0x"), "Hash should start with 0x");
        assert_eq!(hash.len(), 66, "Hash should be 32 bytes (64 hex chars) plus 0x prefix");
        
        println!("Keccak hash of journal bytes: {}", hash);
    }
}