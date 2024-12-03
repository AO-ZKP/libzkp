pub mod adapter;
pub mod verifier_bn;
pub mod verifier_bls;

use crate::adapter::types::{ProofStr, VkeyStr};
use serde_json::Value;
use std::ffi::{c_char, CStr};

fn verify_internal(
    proof: ProofStr,
    vkey: VkeyStr,
    public_inputs: &[String],
    curve_type: &str,
) -> bool {
    match curve_type {
        "bls" => {
            use bellman::groth16::{prepare_verifying_key, verify_proof};
            use bls12_381::Bls12;

            let pof = adapter::parser_bls::parse_bls_proof::<Bls12>(&proof);
            let verificationkey = adapter::parser_bls::parse_bls_vkey::<Bls12>(&vkey);
            let pvk = prepare_verifying_key(&verificationkey);

            let inputs: Vec<_> = public_inputs
                .iter()
                .map(|input| ff::PrimeField::from_str_vartime(input).unwrap())
                .collect();

            verify_proof(&pvk, &pof, &inputs).is_ok()
        }
        "bn" => {
            use crate::verifier_bn::{prepare_verifying_key, verify_proof};
            use pairing_ce::ff::PrimeField as Frce;
            use pairing_ce::bn256::Bn256;

            let pof = adapter::parser_bn::parse_bn_proof::<Bn256>(&proof);
            let verificationkey = adapter::parser_bn::parse_bn_vkey::<Bn256>(&vkey);
            let pvk = prepare_verifying_key(&verificationkey);

            let inputs: Vec<_> = public_inputs
                .iter()
                .map(|input| Frce::from_str(input).unwrap())
                .collect();

            verify_proof(&pvk, &pof, &inputs).unwrap()
        }
        _ => panic!("Invalid curve type. Use 'bn' or 'bls'"),
    }
}

#[no_mangle]
pub extern "C" fn verify(input_ptr: *const c_char) -> i32 {
    if input_ptr.is_null() {
        return -1;
    }

    let c_str = unsafe {
        match CStr::from_ptr(input_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return -2,
        }
    };

    let json_value: Value = match serde_json::from_str(c_str) {
        Ok(v) => v,
        Err(_) => return -3,
    };

    let proof = match extract_proof(&json_value) {
        Ok(p) => p,
        Err(_) => return -4,
    };

    let vkey = match extract_vkey(&json_value) {
        Ok(v) => v,
        Err(_) => return -5,
    };

    let public_inputs: Vec<String> = match json_value.get("public_inputs").and_then(Value::as_array) {
        Some(inputs) => inputs
            .iter()
            .map(|v| v.as_str().unwrap().to_string())
            .collect(),
        None => return -6,
    };

    let curve_type = match json_value.get("curve_type").and_then(Value::as_str) {
        Some(curve) if curve == "bn" || curve == "bls" => curve,
        _ => return -7,
    };

    match verify_internal(proof, vkey, &public_inputs, curve_type) {
        true => 1,
        false => 0,
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
    use serde_json::json;
    use std::fs;

    fn create_verification_input(
        proof_path: &str,
        vkey_path: &str,
        public_path: &str,
        curve_type: &str,
    ) -> String {
        // Read the files
        let proof_str = fs::read_to_string(proof_path).expect("Failed to read proof file");
        let vkey_str = fs::read_to_string(vkey_path).expect("Failed to read vkey file");
        let public_str = fs::read_to_string(public_path).expect("Failed to read public inputs file");

        // Parse JSON
        let proof_json: Value = serde_json::from_str(&proof_str).expect("Failed to parse proof JSON");
        let vkey_json: Value = serde_json::from_str(&vkey_str).expect("Failed to parse vkey JSON");
        let public_inputs: Vec<String> = serde_json::from_str(&public_str).expect("Failed to parse public inputs JSON");

        // Create the complete input JSON
        let input = json!({
            "proof": proof_json,
            "vkey": vkey_json,
            "public_inputs": public_inputs,
            "curve_type": curve_type
        });

        input.to_string()
    }

    #[test]
    fn test_valid_bn_proof() {
        let input = create_verification_input(
            "circuit/Multiplication/proof_uncompressed.json",
            "circuit/Multiplication/vkey_uncompressed.json",
            "circuit/Multiplication/public.json",
            "bn",
        );

        let c_input = std::ffi::CString::new(input).unwrap();
        let result = verify(c_input.as_ptr());
        assert_eq!(result, 1, "Verification should succeed with valid proof");
    }

    #[test]
    fn test_invalid_bn_proof() {
        // Create a modified version of public.json with wrong input
        let invalid_input = json!(["999"]).to_string();
        let invalid_path = "circuit/Multiplication/invalid_public.json";
        std::fs::write(invalid_path, invalid_input).unwrap();

        let input = create_verification_input(
            "circuit/Multiplication/proof_uncompressed.json",
            "circuit/Multiplication/vkey_uncompressed.json",
            invalid_path,
            "bn",
        );

        let c_input = std::ffi::CString::new(input).unwrap();
        let result = verify(c_input.as_ptr());
        assert_eq!(
            result, 0,
            "Verification should fail with invalid public input"
        );
        // Cleanup
        std::fs::remove_file(invalid_path).unwrap();
    }
}