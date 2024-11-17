// In lib.rs - Add this at the root of the verify folder
pub mod adapter;
pub mod circuit;

use adapter::types::{ProofStr, VkeyStr};

use ff::PrimeField as Fr;

/// Verifies a zero-knowledge proof against a verification key and public input
/// 
/// # Arguments
/// * `proof` - The proof in uncompressed format
/// * `vkey` - The verification key in uncompressed format
/// * `public_input` - The public input as a string
/// * `curve_type` - The curve type: "bn" for BN254 or "bls" for BLS12-381
/// 
/// # Returns
/// * `bool` - True if verification succeeds, false otherwise
pub fn verify(proof: ProofStr, vkey: VkeyStr, public_input: &str, curve_type: &str) -> bool {
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
                &[Fr::from_str_vartime(public_input).unwrap()]
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

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::from_str;
    use std::fs;

    #[test]
    fn test_verify_multiplication_circuit() {
        // Read the proof and verification key from the test circuit
        let proof_str = fs::read_to_string("circuit/Multiplication/proof_uncompressed.json")
            .expect("Failed to read proof file");
        let vkey_str = fs::read_to_string("circuit/Multiplication/vkey_uncompressed.json")
            .expect("Failed to read vkey file");

        let proof: ProofStr = from_str(&proof_str).expect("Failed to parse proof");
        let vkey: VkeyStr = from_str(&vkey_str).expect("Failed to parse vkey");

        // The public input is 33 for the multiplication circuit (as shown in the example)
        assert!(verify(proof, vkey, "15", "bn"));
    }

    #[test]
    fn test_verify_multiplication_circuit_invalid_input() {
        // Same setup but with wrong public input
        let proof_str = fs::read_to_string("circuit/Multiplication/proof_uncompressed.json")
            .expect("Failed to read proof file");
        let vkey_str = fs::read_to_string("circuit/Multiplication/vkey_uncompressed.json")
            .expect("Failed to read vkey file");

        let proof: ProofStr = from_str(&proof_str).expect("Failed to parse proof");
        let vkey: VkeyStr = from_str(&vkey_str).expect("Failed to parse vkey");

        // This should fail since 34 is not the correct public input
        assert!(!verify(proof, vkey, "34", "bn"));
    }
}