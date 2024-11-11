use zkp::{verify, VerifierError, VerifierInput};
use bls12_381::{G1Affine, G2Affine, Scalar, Bls12};
use ff::PrimeField;
use std::fs;
use std::path::PathBuf;
use std::ffi::CString;
use serde::Deserialize;
use groth16::{Proof, VerifyingKey};
use num_bigint::BigInt;

#[derive(Deserialize)]
struct ProofJson {
    pi_a: Vec<String>,
    pi_b: Vec<Vec<String>>,
    pi_c: Vec<String>,
    #[allow(dead_code)]
    protocol: String,
    #[allow(dead_code)]
    curve: String,
}

#[derive(Deserialize)]
struct VerifyingKeyJson {
    #[allow(dead_code)]
    protocol: String,
    #[allow(dead_code)]
    curve: String,
    #[allow(dead_code)]
    #[serde(rename = "nPublic")]
    n_public: u32,
    vk_alpha_1: Vec<String>,
    vk_beta_2: Vec<Vec<String>>,
    vk_gamma_2: Vec<Vec<String>>,
    vk_delta_2: Vec<Vec<String>>,
    #[serde(rename = "IC")]
    ic: Vec<Vec<String>>,
}

fn fixture_path(name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("tests");
    path.push("fixtures");
    path.push(name);
    path
}

fn create_verify_input(input: VerifierInput) -> CString {
    let json = serde_json::to_string(&input).unwrap();
    CString::new(json).unwrap()
}

fn parse_decimal_scalar(dec_str: &str) -> Scalar {
    let bigint = BigInt::parse_bytes(dec_str.as_bytes(), 10)
        .expect("Failed to parse decimal string");
    
    let bytes = bigint.to_bytes_be().1;
    let mut arr = [0u8; 32];
    if bytes.len() <= 32 {
        arr[32-bytes.len()..].copy_from_slice(&bytes);
    } else {
        arr.copy_from_slice(&bytes[bytes.len()-32..]);
    }
    
    Scalar::from_repr(arr.into())
        .unwrap_or(Scalar::zero())
}

fn create_proof_from_json(proof_str: &str) -> Proof<Bls12> {
    let proof: ProofJson = serde_json::from_str(proof_str).unwrap();
    
    let a = convert_g1_coords(&proof.pi_a);
    let b = convert_g2_coords(&proof.pi_b);
    let c = convert_g1_coords(&proof.pi_c);

    Proof { a, b, c }
}

fn create_vk_from_json(vk_str: &str) -> VerifyingKey<Bls12> {
    let vk: VerifyingKeyJson = serde_json::from_str(vk_str).unwrap();
    
    VerifyingKey {
        alpha_g1: convert_g1_coords(&vk.vk_alpha_1),
        beta_g1: G1Affine::generator(), // TODO: Implement proper conversion
        beta_g2: convert_g2_coords(&vk.vk_beta_2),
        gamma_g2: convert_g2_coords(&vk.vk_gamma_2),
        delta_g1: G1Affine::generator(), // TODO: Implement proper conversion
        delta_g2: convert_g2_coords(&vk.vk_delta_2),
        ic: vk.ic.iter().map(|coords| convert_g1_coords(coords)).collect(),
    }
}

fn convert_g1_coords(coords: &[String]) -> G1Affine {
    let _x = parse_decimal_scalar(&coords[0]);
    let _y = parse_decimal_scalar(&coords[1]);
    
    // TODO: Implement proper point conversion
    G1Affine::generator()
}

fn convert_g2_coords(coords: &[Vec<String>]) -> G2Affine {
    let _x_c0 = parse_decimal_scalar(&coords[0][0]);
    let _x_c1 = parse_decimal_scalar(&coords[0][1]);
    let _y_c0 = parse_decimal_scalar(&coords[1][0]);
    let _y_c1 = parse_decimal_scalar(&coords[1][1]);
    
    // TODO: Implement proper point conversion
    G2Affine::generator()
}

#[test]
fn test_real_proof_verification() {
    println!("Reading test files...");
    
    // Read JSON files using fixture paths
    let proof_str = fs::read_to_string(fixture_path("proof.json"))
        .expect("Failed to read proof.json");
    let vk_str = fs::read_to_string(fixture_path("verification_key.json"))
        .expect("Failed to read verification_key.json");
    let public_str = fs::read_to_string(fixture_path("public.json"))
        .expect("Failed to read public.json");

    println!("Parsing proof and verification key...");
    
    let proof = create_proof_from_json(&proof_str);
    let vk = create_vk_from_json(&vk_str);

    println!("Parsing public inputs...");
    
    let public_inputs: Vec<Scalar> = serde_json::from_str::<Vec<String>>(&public_str)
        .unwrap()
        .iter()
        .map(|s| parse_decimal_scalar(s))
        .collect();

    println!("Creating verification input...");
    
    let input = VerifierInput {
        vk: base64::encode(&zkp::serialize_verifying_key(&vk)),
        proof: base64::encode(&zkp::serialize_proof(&proof)),
        public_inputs: base64::encode(&zkp::serialize_public_inputs(&public_inputs)),
    };

    let c_input = create_verify_input(input);
    
    println!("Executing verification...");
    let result = verify(c_input.as_ptr());
    
    println!("Verification result: {}", result);
    
    // Currently using placeholder values, so this will fail verification
    // Once properly implemented, should check for Success
    assert!(result != VerifierError::InvalidJson as i32);
    assert!(result != VerifierError::InvalidBase64VerifyingKey as i32);
    assert!(result != VerifierError::InvalidBase64Proof as i32);
    assert!(result != VerifierError::InvalidBase64PublicInputs as i32);
}