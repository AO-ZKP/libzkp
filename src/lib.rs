// lib.rs

use bellman::{groth16, Circuit, ConstraintSystem, SynthesisError};
use bls12_381::Bls12;
use ff::PrimeField;
use rand_chacha::ChaCha20Rng;
use rand::SeedableRng;


#[derive(Clone)]
struct SimpleCircuit<F: PrimeField> {
    a: Option<F>,
    b: Option<F>,
}

impl<F: PrimeField> Circuit<F> for SimpleCircuit<F> {
    fn synthesize<CS: ConstraintSystem<F>>(
        self,
        cs: &mut CS,
    ) -> Result<(), SynthesisError> {
        let a = cs.alloc(|| "a", || self.a.ok_or(SynthesisError::AssignmentMissing))?;
        let b = cs.alloc(|| "b", || self.b.ok_or(SynthesisError::AssignmentMissing))?;
        let c = cs.alloc_input(
            || "c",
            || {
                let mut a = self.a.ok_or(SynthesisError::AssignmentMissing)?;
                let b = self.b.ok_or(SynthesisError::AssignmentMissing)?;
                a.mul_assign(&b);
                Ok(a)
            },
        )?;

        cs.enforce(
            || "mult",
            |lc| lc + a,
            |lc| lc + b,
            |lc| lc + c,
        );

        Ok(())
    }
}

#[no_mangle]
pub extern "C" fn wasm_test() -> i32 {
    let mut rng = ChaCha20Rng::seed_from_u64(42);

    let params = {
        let c = SimpleCircuit::<bls12_381::Scalar> {
            a: None,
            b: None,
        };
        groth16::generate_random_parameters::<Bls12, _, _>(c, &mut rng).unwrap()
    };

    let pvk = groth16::prepare_verifying_key(&params.vk);

    let a = bls12_381::Scalar::from(3u64);
    let b = bls12_381::Scalar::from(4u64);
    let c = a * b;

    let circuit = SimpleCircuit {
        a: Some(a),
        b: Some(b),
    };

    let proof = groth16::create_random_proof(circuit, &params, &mut rng).unwrap();

    let result = groth16::verify_proof(&pvk, &proof, &[c]).is_ok();

    if result { 1 } else { 0 }
}