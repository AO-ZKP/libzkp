use bls12_381::{Bls12, Scalar};
use std::ffi::CStr;
use std::os::raw::c_char;
use serde::{Deserialize, Serialize};
use groth16::{prepare_verifying_key, verify_proof, Proof, VerifyingKey};

#[derive(Serialize, Deserialize, Debug)]
pub struct VerifierInput {
    pub vk: String,           // base64 encoded verifying key
    pub proof: String,        // base64 encoded proof
    pub public_inputs: String // base64 encoded array of public inputs
}

/// Error codes returned by the verifier
#[repr(i32)]
#[derive(Debug, PartialEq)]
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

/// Validates the length of verifying key bytes
fn verify_vk_bytes_length(bytes: &[u8]) -> Result<(), &'static str> {
    const G1_SIZE: usize = 96;
    const G2_SIZE: usize = 192;
    const U32_SIZE: usize = 4;
    
    let min_size = G1_SIZE * 3 + G2_SIZE * 3 + U32_SIZE;
    if bytes.len() < min_size {
        return Err("Verifying key bytes too short");
    }
    Ok(())
}

/// Verifies a Groth16 proof
/// Returns 0 for success, negative numbers for specific error conditions
#[no_mangle]
pub extern "C" fn verify(input_ptr: *const c_char) -> i32 {
    let input_str = unsafe {
        match CStr::from_ptr(input_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return VerifierError::InvalidInputString as i32
        }
    };

    let input: VerifierInput = match serde_json::from_str(input_str) {
        Ok(i) => i,
        Err(_) => return VerifierError::InvalidJson as i32
    };

    let vk_bytes = match base64::decode(&input.vk) {
        Ok(bytes) => bytes,
        Err(_) => return VerifierError::InvalidBase64VerifyingKey as i32
    };

    if verify_vk_bytes_length(&vk_bytes).is_err() {
        return VerifierError::InvalidVerifyingKey as i32;
    }

    let verifying_key = match VerifyingKey::<Bls12>::read(&vk_bytes[..]) {
        Ok(vk) => vk,
        Err(_) => return VerifierError::InvalidVerifyingKey as i32
    };

    let proof_bytes = match base64::decode(&input.proof) {
        Ok(bytes) => bytes,
        Err(_) => return VerifierError::InvalidBase64Proof as i32
    };

    let proof = match Proof::<Bls12>::read(&proof_bytes[..]) {
        Ok(p) => p,
        Err(_) => return VerifierError::InvalidProof as i32
    };

    let inputs_bytes = match base64::decode(&input.public_inputs) {
        Ok(bytes) => bytes,
        Err(_) => return VerifierError::InvalidBase64PublicInputs as i32
    };

    let public_inputs = match deserialize_public_inputs(&inputs_bytes) {
        Ok(inputs) => inputs,
        Err(_) => return VerifierError::InvalidPublicInputs as i32
    };

    let pvk = prepare_verifying_key(&verifying_key);

    match verify_proof(&pvk, &proof, &public_inputs[..]) {
        Ok(()) => VerifierError::Success as i32,
        Err(_) => VerifierError::ProofVerificationFailed as i32
    }
}

/// Helper function to serialize a verifying key
pub fn serialize_verifying_key(vk: &VerifyingKey<Bls12>) -> Vec<u8> {
    let mut bytes = Vec::new();
    vk.write(&mut bytes).expect("Failed to serialize verifying key");
    bytes
}

/// Helper function to serialize a proof
pub fn serialize_proof(proof: &Proof<Bls12>) -> Vec<u8> {
    let mut bytes = Vec::new();
    proof.write(&mut bytes).expect("Failed to serialize proof");
    bytes
}

/// Helper function to serialize public inputs
pub fn serialize_public_inputs(inputs: &[Scalar]) -> Vec<u8> {
    let mut bytes = Vec::new();
    for input in inputs {
        bytes.extend_from_slice(input.to_bytes().as_ref());
    }
    bytes
}

/// Deserializes public inputs from bytes
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