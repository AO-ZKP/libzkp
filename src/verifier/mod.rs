// src/verifier/mod.rs

use pairing_ce::{
    Engine,
    CurveProjective,
    CurveAffine,
};
use pairing_ce::ff::PrimeField;

#[derive(Clone)]
pub struct Proof<E: Engine> {
    pub a: E::G1Affine,
    pub b: E::G2Affine,
    pub c: E::G1Affine
}

#[derive(Clone)]
pub struct VerifyingKey<E: Engine> {
    pub alpha_g1: E::G1Affine,
    pub beta_g1: E::G1Affine,
    pub beta_g2: E::G2Affine,
    pub gamma_g2: E::G2Affine,
    pub delta_g1: E::G1Affine,
    pub delta_g2: E::G2Affine,
    pub ic: Vec<E::G1Affine>
}

pub struct PreparedVerifyingKey<E: Engine> {
    alpha_g1_beta_g2: E::Fqk,
    neg_gamma_g2: <E::G2Affine as CurveAffine>::Prepared,
    neg_delta_g2: <E::G2Affine as CurveAffine>::Prepared,
    ic: Vec<E::G1Affine>
}

#[derive(Debug)]
pub enum SynthesisError {
    AssignmentMissing,
    DivisionByZero,
    Unsatisfiable,
    PolynomialDegreeTooLarge,
    UnexpectedIdentity,
    IoError,
    MalformedVerifyingKey,
    UnconstrainedVariable
}

pub fn prepare_verifying_key<E: Engine>(
    vk: &VerifyingKey<E>
) -> PreparedVerifyingKey<E> {
    let mut gamma = vk.gamma_g2;
    gamma.negate();
    let mut delta = vk.delta_g2;
    delta.negate();

    PreparedVerifyingKey {
        alpha_g1_beta_g2: E::pairing(vk.alpha_g1, vk.beta_g2),
        neg_gamma_g2: gamma.prepare(),
        neg_delta_g2: delta.prepare(),
        ic: vk.ic.clone()
    }
}

pub fn verify_proof<'a, E: Engine>(
    pvk: &'a PreparedVerifyingKey<E>,
    proof: &Proof<E>,
    public_inputs: &[E::Fr]
) -> Result<bool, SynthesisError> {
    if (public_inputs.len() + 1) != pvk.ic.len() {
        return Err(SynthesisError::MalformedVerifyingKey);
    }

    let mut acc = pvk.ic[0].into_projective();

    for (i, b) in public_inputs.iter().zip(pvk.ic.iter().skip(1)) {
        acc.add_assign(&b.mul(i.into_repr()));
    }

    Ok(E::final_exponentiation(
        &E::miller_loop([
            (&proof.a.prepare(), &proof.b.prepare()),
            (&acc.into_affine().prepare(), &pvk.neg_gamma_g2),
            (&proof.c.prepare(), &pvk.neg_delta_g2)
        ].iter())
    ).unwrap() == pvk.alpha_g1_beta_g2)
}