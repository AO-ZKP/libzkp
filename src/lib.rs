pub mod adapter;
pub mod circuit;

use std::ffi::{CStr, c_char};
use crate::adapter::types::{ProofStr, VkeyStr};
use serde_json::Value;

/// Internal verify function used by the library
fn verify_internal(proof: ProofStr, vkey: VkeyStr, public_input: &str, curve_type: &str) -> bool {
    match curve_type {
        "bls" => {
            use bellman::groth16::{prepare_verifying_key, verify_proof};
            use bls12_381::Bls12;

            let pof = adapter::parser_bls::parse_bls_proof::<Bls12>(&proof);
            let verificationkey = adapter::parser_bls::parse_bls_vkey::<Bls12>(&vkey);
            let pvk = prepare_verifying_key(&verificationkey);

            verify_proof(
                &pvk,
                &pof, 
                &[ff::PrimeField::from_str_vartime(public_input).unwrap()]
            ).is_ok()
        },
        "bn" => {
            use bellman_ce::groth16::{prepare_verifying_key, verify_proof};
            use pairing_ce::bn256::Bn256;
            use ff_ce::PrimeField as Frce;

            let pof = adapter::parser_bn::parse_bn_proof::<Bn256>(&proof); 
            let verificationkey = adapter::parser_bn::parse_bn_vkey::<Bn256>(&vkey);
            let pvk = prepare_verifying_key(&verificationkey);

            verify_proof(
                &pvk,
                &pof,
                &[Frce::from_str(public_input).unwrap()]
            ).unwrap()
        },
        _ => panic!("Invalid curve type. Use 'bn' or 'bls'")
    }
}

/// C-compatible verify function that takes a JSON string input
#[no_mangle]
pub extern "C" fn verify(input_ptr: *const c_char) -> i32 {
    // Safety checks
    if input_ptr.is_null() {
        return -1; // Null pointer error
    }

    // Convert C string to Rust string safely
    let c_str = unsafe {
        match CStr::from_ptr(input_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -2, // Invalid UTF-8 error
        }
    };

    // Parse JSON input
    let json_value: Value = match serde_json::from_str(c_str) {
        Ok(v) => v,
        Err(_) => return -3, // JSON parsing error
    };

    // Extract required fields
    let proof = match extract_proof(&json_value) {
        Ok(p) => p,
        Err(_) => return -4, // Proof extraction error
    };

    let vkey = match extract_vkey(&json_value) {
        Ok(v) => v,
        Err(_) => return -5, // Vkey extraction error
    };

    let public_input = match json_value.get("public_input").and_then(Value::as_str) {
        Some(input) => input,
        None => return -6, // Missing public input error
    };

    let curve_type = match json_value.get("curve_type").and_then(Value::as_str) {
        Some(curve) if curve == "bn" || curve == "bls" => curve,
        _ => return -7, // Invalid curve type error
    };

    // Call the internal verify function
    match verify_internal(proof, vkey, public_input, curve_type) {
        true => 0,  // Success
        false => 1, // Verification failed
    }
}

fn extract_proof(json: &Value) -> Result<ProofStr, &'static str> {
    let proof_obj = json.get("proof").ok_or("Missing proof object")?;
    
    Ok(ProofStr {
        pi_a: extract_bytes_array(proof_obj, "pi_a")?,
        pi_b: extract_bytes_array(proof_obj, "pi_b")?,
        pi_c: extract_bytes_array(proof_obj, "pi_c")?,
    })
}

fn extract_vkey(json: &Value) -> Result<VkeyStr, &'static str> {
    let vkey_obj = json.get("vkey").ok_or("Missing vkey object")?;
    
    Ok(VkeyStr {
        alpha_1: extract_bytes_array(vkey_obj, "alpha_1")?,
        beta_2: extract_bytes_array(vkey_obj, "beta_2")?,
        gamma_2: extract_bytes_array(vkey_obj, "gamma_2")?,
        delta_2: extract_bytes_array(vkey_obj, "delta_2")?,
        ic: extract_ic_array(vkey_obj)?,
    })
}

fn extract_bytes_array(obj: &Value, field: &str) -> Result<Vec<u8>, &'static str> {
    obj.get(field)
        .and_then(Value::as_array)
        .ok_or("Missing or invalid field")?
        .iter()
        .map(|v| v.as_u64().map(|n| n as u8))
        .collect::<Option<Vec<u8>>>()
        .ok_or("Invalid byte array")
}

fn extract_ic_array(obj: &Value) -> Result<Vec<Vec<u8>>, &'static str> {
    obj.get("ic")
        .and_then(Value::as_array)
        .ok_or("Missing ic array")?
        .iter()
        .map(|arr| {
            arr.as_array()
                .and_then(|nums| {
                    nums.iter()
                        .map(|v| v.as_u64().map(|n| n as u8))
                        .collect::<Option<Vec<u8>>>()
                })
                .ok_or("Invalid ic array")
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use serde_json::json;

    /// Test helper function to create a JSON input string for verification
    fn create_verification_input(proof_path: &str, vkey_path: &str, public_input: &str, curve_type: &str) -> String {
        let proof_json: Value = serde_json::from_str(
            &fs::read_to_string(proof_path).expect("Failed to read proof file")
        ).expect("Failed to parse proof JSON");
        
        let vkey_json: Value = serde_json::from_str(
            &fs::read_to_string(vkey_path).expect("Failed to parse vkey file")
        ).expect("Failed to parse vkey JSON");

        // Create the complete input JSON structure
        let input = json!({
            "proof": proof_json,
            "vkey": vkey_json,
            "public_input": public_input,
            "curve_type": curve_type
        });

        input.to_string()
    }

    #[test]
    fn test_valid_bn_proof() {
        let input = create_verification_input(
            "circuit/Multiplication/proof_uncompressed.json",
            "circuit/Multiplication/vkey_uncompressed.json",
            "15",  // Correct public input
            "bn"
        );

        let c_input = std::ffi::CString::new(input).unwrap();
        let result = verify(c_input.as_ptr());
        assert_eq!(result, 0, "Verification should succeed with valid proof");
    }

    #[test]
    fn test_invalid_bn_proof() {
        let input = create_verification_input(
            "circuit/Multiplication/proof_uncompressed.json",
            "circuit/Multiplication/vkey_uncompressed.json",
            "999",  // Incorrect public input
            "bn"
        );

        let c_input = std::ffi::CString::new(input).unwrap();
        let result = verify(c_input.as_ptr());
        assert_eq!(result, 1, "Verification should fail with invalid public input");
    }
}