#![cfg_attr(not(feature = "std"), no_std)]
extern crate alloc;

use pairing_ce::{CurveAffine, Engine, GenericCurveProjective};
use pairing_ce::ff::PrimeField;
use codec::{Encode, Decode};
use alloc::vec::Vec;
use core::prelude::v1::*;

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone, Encode, Decode, Default, Eq)]
pub struct Proof<E: Engine> {
    pub a: E::G1Affine,
    pub b: E::G2Affine,
    pub c: E::G1Affine,
}

impl<E: Engine> PartialEq for Proof<E> {
    fn eq(&self, other: &Self) -> bool {
        self.a == other.a && self.b == other.b && self.c == other.c
    }
}

#[cfg_attr(feature = "std", derive(Debug))]
#[derive(Clone)]
pub struct VerifyingKey<E: Engine> {
    pub alpha_g1: E::G1Affine,
    pub beta_g1: E::G1Affine,
    pub beta_g2: E::G2Affine,
    pub gamma_g2: E::G2Affine,
    pub delta_g1: E::G1Affine,
    pub delta_g2: E::G2Affine,
    pub ic: Vec<E::G1Affine>,
}

impl<E: Engine> PartialEq for VerifyingKey<E> {
    fn eq(&self, other: &Self) -> bool {
        self.alpha_g1 == other.alpha_g1 &&
        self.beta_g1 == other.beta_g1 &&
        self.beta_g2 == other.beta_g2 &&
        self.gamma_g2 == other.gamma_g2 &&
        self.delta_g1 == other.delta_g1 &&
        self.delta_g2 == other.delta_g2 &&
        self.ic == other.ic
    }
}

pub struct PreparedVerifyingKey<E: Engine> {
    /// Pairing result of alpha*beta
    alpha_g1_beta_g2: E::Fqk,
    /// -gamma in G2
    neg_gamma_g2: <E::G2Affine as CurveAffine>::Prepared,
    /// -delta in G2
    neg_delta_g2: <E::G2Affine as CurveAffine>::Prepared,
    /// Copy of IC from `VerifyingKey`
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
    UnconstrainedVariable,
}

#[derive(Debug, Clone)]
pub enum VerificationError {
    InvalidVerifyingKey,
    InvalidProof,
}

pub fn prepare_verifying_key<E: Engine>(
    vk: &VerifyingKey<E>
) -> PreparedVerifyingKey<E>
{
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

pub fn verify_proof<E: Engine>(
    pvk: &PreparedVerifyingKey<E>,
    proof: &Proof<E>,
    public_inputs: &[E::Fr]
) -> Result<(), VerificationError>
{
    if (public_inputs.len() + 1) != pvk.ic.len() {
        return Err(VerificationError::InvalidVerifyingKey);
    }

    let mut acc = pvk.ic[0].into_projective();

    for (i, b) in public_inputs.iter().zip(pvk.ic.iter().skip(1)) {
        let mut term = b.into_projective();
        term.mul_assign(i.into_raw_repr());
        acc.add_assign(&term);
    }

    // The original verification equation is:
    // A * B = alpha * beta + inputs * gamma + C * delta
    // ... however, we rearrange it so that it is:
    // A * B - inputs * gamma - C * delta = alpha * beta
    // or equivalently:
    // A * B + inputs * (-gamma) + C * (-delta) = alpha * beta
    // which allows us to do a single final exponentiation.

    let acc_affine = acc.into_affine();

    if E::final_exponentiation(
        &E::miller_loop([
            (&proof.a.prepare(), &proof.b.prepare()),
            (&acc_affine.prepare(), &pvk.neg_gamma_g2),
            (&proof.c.prepare(), &pvk.neg_delta_g2)
        ].iter())
    ).ok_or(VerificationError::InvalidProof)? == pvk.alpha_g1_beta_g2 {
        Ok(())
    } else {
        Err(VerificationError::InvalidProof)
    }
}