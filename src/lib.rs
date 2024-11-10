use bls12_381::{Bls12, Scalar};
use std::ffi::CStr;
use std::os::raw::c_char;
use serde::{Deserialize, Serialize};
use groth16::{prepare_verifying_key, verify_proof, Proof, VerifyingKey};

#[derive(Serialize, Deserialize)]
struct VerifierInput {
    vk: String,           // base64 encoded verifying key
    proof: String,        // base64 encoded proof
    public_inputs: String // base64 encoded array of public inputs
}

/// Returns true if proof is valid, false otherwise
#[no_mangle]
pub extern "C" fn verify(input_ptr: *const c_char) -> bool {
    // Convert C string to Rust string
    let input_str = unsafe {
        match CStr::from_ptr(input_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return false
        }
    };

    // Parse input JSON
    let input: VerifierInput = match serde_json::from_str(input_str) {
        Ok(i) => i,
        Err(_) => return false
    };

    // Decode base64 inputs
    let vk_bytes = match base64::decode(&input.vk) {
        Ok(bytes) => bytes,
        Err(_) => return false
    };

    let proof_bytes = match base64::decode(&input.proof) {
        Ok(bytes) => bytes,
        Err(_) => return false 
    };

    let inputs_bytes = match base64::decode(&input.public_inputs) {
        Ok(bytes) => bytes,
        Err(_) => return false
    };

    // Deserialize verifying key
    let verifying_key = match VerifyingKey::<Bls12>::read(&vk_bytes[..]) {
        Ok(vk) => vk,
        Err(_) => return false
    };

    // Deserialize proof
    let proof = match Proof::<Bls12>::read(&proof_bytes[..]) {
        Ok(p) => p,
        Err(_) => return false
    };

    // Deserialize public inputs
    let public_inputs: Vec<Scalar> = match deserialize_public_inputs(&inputs_bytes) {
        Ok(inputs) => inputs,
        Err(_) => return false
    };

    // Prepare verifying key
    let pvk = prepare_verifying_key(&verifying_key);

    // Verify the proof
    match verify_proof(&pvk, &proof, &public_inputs[..]) {
        Ok(()) => true,
        Err(_) => false
    }
}

fn deserialize_public_inputs(bytes: &[u8]) -> Result<Vec<Scalar>, &'static str> {
    if bytes.len() % 32 != 0 {
        return Err("Invalid public inputs length");
    }

    let mut result = Vec::with_capacity(bytes.len() / 32);
    let mut buf = [0u8; 32];

    for chunk in bytes.chunks(32) {
        buf.copy_from_slice(chunk);
        if let Some(fr) = Scalar::from_bytes(&buf).into() {
            result.push(fr);
        } else {
            return Err("Invalid public input value");
        }
    }

    Ok(result)
}

// Note: Removed alloc_string and free_string as they weren't necessary 
// since we're just returning a bool