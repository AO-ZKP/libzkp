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

/// Error codes returned by the verifier
#[repr(i32)]
pub enum VerifierError {
    Success = 0,
    InvalidInputString = -1,
    InvalidJson = -2,
    InvalidBase64VerifyingKey = -3, 
    InvalidBase64Proof = -4,
    InvalidBase64PublicInputs = -5,
    InvalidVerifyingKey = -6,
    InvalidProof = -7,
    InvalidPublicInputs = -8,
    ProofVerificationFailed = -9
}

/// Verifies a Groth16 proof
/// Returns 0 for success, negative numbers for specific error conditions
#[no_mangle]
pub extern "C" fn verify(input_ptr: *const c_char) -> i32 {
    // Convert C string to Rust string
    let input_str = unsafe {
        match CStr::from_ptr(input_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return VerifierError::InvalidInputString as i32
        }
    };

    // Parse input JSON
    let input: VerifierInput = match serde_json::from_str(input_str) {
        Ok(i) => i,
        Err(_) => return VerifierError::InvalidJson as i32
    };

    // Decode base64 verifying key
    let vk_bytes = match base64::decode(&input.vk) {
        Ok(bytes) => bytes,
        Err(_) => return VerifierError::InvalidBase64VerifyingKey as i32
    };

    // Decode base64 proof
    let proof_bytes = match base64::decode(&input.proof) {
        Ok(bytes) => bytes,
        Err(_) => return VerifierError::InvalidBase64Proof as i32
    };

    // Decode base64 public inputs
    let inputs_bytes = match base64::decode(&input.public_inputs) {
        Ok(bytes) => bytes,
        Err(_) => return VerifierError::InvalidBase64PublicInputs as i32
    };

    // Deserialize verifying key
    let verifying_key = match VerifyingKey::<Bls12>::read(&vk_bytes[..]) {
        Ok(vk) => vk,
        Err(_) => return VerifierError::InvalidVerifyingKey as i32
    };

    // Deserialize proof
    let proof = match Proof::<Bls12>::read(&proof_bytes[..]) {
        Ok(p) => p,
        Err(_) => return VerifierError::InvalidProof as i32
    };

    // Deserialize public inputs
    let public_inputs = match deserialize_public_inputs(&inputs_bytes) {
        Ok(inputs) => inputs,
        Err(_) => return VerifierError::InvalidPublicInputs as i32
    };

    // Prepare verifying key
    let pvk = prepare_verifying_key(&verifying_key);

    // Verify the proof
    match verify_proof(&pvk, &proof, &public_inputs[..]) {
        Ok(()) => VerifierError::Success as i32,
        Err(_) => VerifierError::ProofVerificationFailed as i32
    }
}

/// Deserializes public inputs from bytes
/// Returns Vec<Scalar> on success, error message on failure
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_invalid_input_string() {
        let invalid_ptr = std::ptr::null();
        assert_eq!(verify(invalid_ptr), VerifierError::InvalidInputString as i32);
    }

    #[test]
    fn test_invalid_json() {
        let invalid_json = CString::new("{invalid}").unwrap();
        assert_eq!(verify(invalid_json.as_ptr()), VerifierError::InvalidJson as i32);
    }

    #[test]
    fn test_invalid_base64() {
        let input = VerifierInput {
            vk: "invalid base64".to_string(),
            proof: "".to_string(),
            public_inputs: "".to_string()
        };
        let json = serde_json::to_string(&input).unwrap();
        let c_json = CString::new(json).unwrap();
        assert_eq!(verify(c_json.as_ptr()), VerifierError::InvalidBase64VerifyingKey as i32);
    }
}