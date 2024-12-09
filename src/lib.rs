//! ZKP Library for RISC Zero Verification
//! 
//! This library provides functionality for verifying zero-knowledge proofs
//! and handling blockchain commitments in a no_std environment.
//! It includes capabilities for keccak hashing and proof verification
//! while remaining compatible with embedded systems.

#![no_std]
extern crate alloc;

use alloc::{string::{String, ToString}, vec::Vec};
use alloc::ffi::CString;
use core::{ffi::{c_char, CStr}, convert::AsRef};
use alloy_sol_types::{sol, SolValue};
use alloy_primitives::{keccak256, U256, hex, B256, aliases::U240};
use risc0_steel::Commitment;
use risc0_zkvm::{Receipt, sha::Digest};
use serde::{Serialize, Deserialize};

// Define Solidity-compatible structures using the sol! macro
sol! {
    /// Represents the full journal structure containing commitment and transaction details
    struct Journal {
        Commitment commitment;
        address from;
        uint256 amount;
        uint256 timestamp;
        uint256 nullifier;
    }

    /// Structure containing only the fields that need to be hashed
    /// Used to generate consistent hashes for verification
    struct JournalHashFields {
        address from;
        uint256 amount;
        uint256 timestamp;
        uint256 nullifier;
    }
}

/// Input structure for verification functions
/// Contains the ZK proof receipt and associated data
#[derive(Serialize, Deserialize)]
struct Input {
    /// The ZK proof receipt containing the verification data
    receipt: Receipt,
    /// Withdrawal identifier string
    withdraw: String,
    /// Image ID for proof verification
    imageid: String,
}

/// Output structure containing verification results
/// Used to return the status and details of verification
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
struct Output {
    /// Optional error message if verification fails
    error: Option<String>,
    /// Indicates if the proof verification was successful
    proof: bool,
    /// Indicates if the withdrawal verification was successful
    withdraw: bool,
    /// The transaction amount
    amount: String,
    /// Transaction timestamp
    timestamp: String,
    /// Block number
    blocknumber: String,
    /// Block hash
    blockhash: String,
}

impl Output {
    /// Creates a failed output with the specified error message
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

/// Calculates the keccak256 hash of journal fields
/// 
/// # Safety
/// 
/// This function handles raw pointers and assumes the input_ptr is valid and points to properly formatted JSON data.
/// Caller must ensure the pointer is valid and the memory it points to remains valid for the duration of the call.
/// 
/// # Arguments
/// 
/// * `input_ptr` - Pointer to null-terminated string containing JSON input
/// 
/// # Returns
/// 
/// Returns a pointer to a null-terminated string containing JSON with the calculated hash or error message
#[no_mangle]
pub extern "C" fn keccak(input_ptr: *const c_char) -> *const c_char {
    // Validate input pointer
    if input_ptr.is_null() {
        return create_error_response("Invalid null input pointer");
    }

    // Convert C string to Rust string safely
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

    // Get journal bytes from receipt
    let journal_bytes: &Vec<u8> = &input.receipt.journal.bytes;
    
    // Decode journal data
    let journal: Journal = match Journal::abi_decode(journal_bytes, true) {
        Ok(j) => j,
        Err(_) => return create_error_response("Invalid journal data: Failed to decode journal bytes"),
    };

    // Create hash fields structure with selected fields
    let hash_fields = JournalHashFields {
        from: journal.from,
        amount: journal.amount,
        timestamp: journal.timestamp,
        nullifier: journal.nullifier,
    };

    // Encode fields and calculate hash
    let encoded_fields = hash_fields.abi_encode();
    let hash: B256 = keccak256(&encoded_fields);
    let hash_hex = alloc::format!("0x{}", hex::encode(hash));
    
    // Create response JSON
    let response = serde_json::json!({
        "error": String::new(),
        "hash": hash_hex
    });

    // Convert response to C string
    CString::new(response.to_string())
        .unwrap_or_else(|_| CString::new(r#"{"error":"Failed to create response"}"#).unwrap())
        .into_raw()
}

/// Creates an error response JSON string as a C string
/// 
/// # Arguments
/// 
/// * `msg` - The error message to include in the response
/// 
/// # Returns
/// 
/// Returns a pointer to a null-terminated string containing JSON error response
fn create_error_response(msg: &str) -> *const c_char {
    let error_json = serde_json::json!({
        "error": msg,
        "hash": String::new()
    });
    CString::new(error_json.to_string())
        .unwrap_or_else(|_| CString::new(r#"{"error":"Failed to create error response"}"#).unwrap())
        .into_raw()
}

/// Verifies a zero-knowledge proof and associated data
/// 
/// # Safety
/// 
/// This function handles raw pointers and assumes the input_ptr is valid and points to properly formatted JSON data.
/// Caller must ensure the pointer is valid and the memory it points to remains valid for the duration of the call.
/// 
/// # Arguments
/// 
/// * `input_ptr` - Pointer to null-terminated string containing JSON input
/// 
/// # Returns
/// 
/// Returns a pointer to a null-terminated string containing JSON with verification results
#[no_mangle]
pub extern "C" fn verify(input_ptr: *const c_char) -> *const c_char {
    // Validate input pointer
    if input_ptr.is_null() {
        return create_response(&Output::failure("Invalid null input pointer"));
    }

    // Convert C string to Rust string safely
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

    // Extract components from input
    let receipt: Receipt = input.receipt;
    let withdraw: String = input.withdraw;
    let image_id: Digest = match hex::decode(&input.imageid[2..]) {
        Ok(bytes) => match Digest::try_from(bytes.as_slice()) {
            Ok(digest) => digest,
            Err(_) => return create_response(&Output::failure("Failed to create digest from image ID")),
        },
        Err(_) => return create_response(&Output::failure("Failed to decode hex image ID")),
    };

    // Decode journal data
    let journal: &Vec<u8> = &receipt.journal.bytes;
    let journal: Journal = match Journal::abi_decode(journal, true) {
        Ok(j) => j,
        Err(_) => return create_response(&Output::failure("Failed to decode journal data")),
    };

    // Verify nullifier by comparing hashes
    let nullifier: alloy_primitives::Uint<256, 4> = journal.nullifier;
    let withdraw_hash: U256 = keccak256(withdraw.as_bytes()).into();
    let does_hash_match: bool = nullifier == withdraw_hash;

    if !does_hash_match {
        return create_response(&Output::failure("Nullifier verification failed: Hash mismatch"));
    }

    // Verify the ZK proof
    if let Err(_) = receipt.verify(image_id) {
        return create_response(&Output::failure("Receipt verification failed"));
    }

    // Extract and verify commitment details
    let commitment_id: alloy_primitives::Uint<256, 4> = journal.commitment.id;
    let id_mask: alloy_primitives::Uint<256, 4> = U256::from_str_radix(
        "0000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff", 
        16
    ).unwrap();

    // Extract version and block number
    let version: u16 = ((commitment_id >> 240u32).as_limbs()[0] & 0xFFFF) as u16;
    let masked_value = commitment_id & id_mask;
    let block_number = U240::from(masked_value);

    if version != 0 {
        return create_response(&Output::failure("Invalid version number"));
    }

    // Get digest bytes
    let digest_bytes: &[u8] = journal.commitment.digest.as_ref();

    // Create successful output
    let output = Output {
        error: None,
        proof: true,
        withdraw: true,
        amount: journal.amount.to_string(),
        timestamp: journal.timestamp.to_string(),
        blocknumber: block_number.to_string(),
        blockhash: alloc::format!("0x{}", hex::encode(digest_bytes)),
    };

    create_response(&output)
}

/// Creates a JSON response string as a C string from an Output struct
/// 
/// # Arguments
/// 
/// * `output` - The Output struct to convert to JSON
/// 
/// # Returns
/// 
/// Returns a pointer to a null-terminated string containing JSON response
fn create_response(output: &Output) -> *const c_char {
    let json = serde_json::to_string(output).unwrap();
    CString::new(json).unwrap().into_raw()
}

/// Test module that uses std features
/// These tests verify the core functionality of the library
#[cfg(test)]
mod tests {
    use super::*;
    
    // Enable std features only for tests
    extern crate std;
    use std::fs;
    use std::println;

    /// Tests the verify function with sample input
    #[test]
    fn test_verify_function() {
        let input_json = fs::read_to_string("input_fixed.json")
            .expect("Failed to read input.json");
        
        let c_input: CString = CString::new(input_json)
            .expect("Failed to create CString from input");
        
        let result_ptr: *const i8 = verify(c_input.as_ptr());
        
        let result = unsafe {
            CStr::from_ptr(result_ptr)
                .to_string_lossy()
                .into_owned()
        };

        if let Ok(output_json) = serde_json::from_str::<serde_json::Value>(&result) {
            println!("\nFormatted Output:");
            println!("{}", serde_json::to_string_pretty(&output_json).unwrap());
        }
    }

    /// Tests the keccak function with sample input
    #[test]
    fn test_keccak_function() {
        let input_json = fs::read_to_string("input_fixed.json")
            .expect("Failed to read input_fixed.json");

        let c_input = CString::new(input_json)
            .expect("Failed to create CString from input");
        
        let result_ptr = keccak(c_input.as_ptr());
        
        let result = unsafe {
            CStr::from_ptr(result_ptr)
                .to_string_lossy()
                .into_owned()
        };

        let result_json: serde_json::Value = serde_json::from_str(&result)
            .expect("Failed to parse result JSON");

        let hash = result_json["hash"].as_str().expect("Hash not found in result");
        assert!(hash.starts_with("0x"), "Hash should start with 0x");
        assert_eq!(hash.len(), 66, "Hash should be 32 bytes (64 hex chars) plus 0x prefix");
    }
}