use super::{ProofStr, VkeyStr};
use crate::verifier_bls::{Proof, VerifyingKey};
use alloc::vec::Vec;
use pairing_ce::bls12_381::{G1Affine, G1Uncompressed, G2Affine, G2Uncompressed};
use pairing_ce::{CurveAffine, EncodedPoint, Engine};

pub fn parse_bls_proof<E>(proof: &ProofStr) -> Proof<E>
where
    E: Engine<G1Affine = G1Affine, G2Affine = G2Affine>,
{
    let pi_a = &proof.pi_a;
    let pi_b = &proof.pi_b;
    let pi_c = &proof.pi_c;

    let mut a_arr: [u8; 96] = [0; 96];
    let mut b_arr: [u8; 192] = [0; 192];
    let mut c_arr: [u8; 96] = [0; 96];

    a_arr[..pi_a.len()].copy_from_slice(&pi_a[..]);
    b_arr[..pi_b.len()].copy_from_slice(&pi_b[..]);
    c_arr[..pi_c.len()].copy_from_slice(&pi_c[..]);

    let mut pia_uncomp = G1Uncompressed::empty();
    let mut pib_uncomp = G2Uncompressed::empty();
    let mut pic_uncomp = G1Uncompressed::empty();

    pia_uncomp.as_mut().copy_from_slice(&a_arr);
    pib_uncomp.as_mut().copy_from_slice(&b_arr);
    pic_uncomp.as_mut().copy_from_slice(&c_arr);

    let pia_affine = pia_uncomp.into_affine().expect("Invalid G1 point for pi_a");
    let pib_affine = pib_uncomp.into_affine().expect("Invalid G2 point for pi_b");
    let pic_affine = pic_uncomp.into_affine().expect("Invalid G1 point for pi_c");

    Proof {
        a: pia_affine,
        b: pib_affine,
        c: pic_affine,
    }
}

pub fn parse_bls_vkey<E>(vkey: &VkeyStr) -> VerifyingKey<E>
where
    E: Engine<G1Affine = G1Affine, G2Affine = G2Affine>,
{
    let vk_alpha_1 = &vkey.alpha_1;
    let vk_beta_2 = &vkey.beta_2;
    let vk_gamma_2 = &vkey.gamma_2;
    let vk_delta_2 = &vkey.delta_2;
    let vk_ic = &vkey.ic;

    let mut alpha1: [u8; 96] = [0; 96];
    let mut beta2: [u8; 192] = [0; 192];
    let mut gamma2: [u8; 192] = [0; 192];
    let mut delta2: [u8; 192] = [0; 192];
    let mut ic_0: [u8; 96] = [0; 96];
    let mut ic_1: [u8; 96] = [0; 96];

    alpha1[..vk_alpha_1.len()].copy_from_slice(&vk_alpha_1[..]);
    beta2[..vk_beta_2.len()].copy_from_slice(&vk_beta_2[..]);
    gamma2[..vk_gamma_2.len()].copy_from_slice(&vk_gamma_2[..]);
    delta2[..vk_delta_2.len()].copy_from_slice(&vk_delta_2[..]);
    ic_0[..vk_ic[0].len()].copy_from_slice(&vk_ic[0][..]);
    ic_1[..vk_ic[1].len()].copy_from_slice(&vk_ic[1][..]);

    let mut alpha1_uncomp = G1Uncompressed::empty();
    let mut beta2_uncomp = G2Uncompressed::empty();
    let mut gamma2_uncomp = G2Uncompressed::empty();
    let mut delta2_uncomp = G2Uncompressed::empty();
    let mut ic0_uncomp = G1Uncompressed::empty();
    let mut ic1_uncomp = G1Uncompressed::empty();

    alpha1_uncomp.as_mut().copy_from_slice(&alpha1);
    beta2_uncomp.as_mut().copy_from_slice(&beta2);
    gamma2_uncomp.as_mut().copy_from_slice(&gamma2);
    delta2_uncomp.as_mut().copy_from_slice(&delta2);
    ic0_uncomp.as_mut().copy_from_slice(&ic_0);
    ic1_uncomp.as_mut().copy_from_slice(&ic_1);

    let alpha1_affine = alpha1_uncomp.into_affine().expect("Invalid G1 point for alpha1");
    let beta2_affine = beta2_uncomp.into_affine().expect("Invalid G2 point for beta2");
    let gamma2_affine = gamma2_uncomp.into_affine().expect("Invalid G2 point for gamma2");
    let delta2_affine = delta2_uncomp.into_affine().expect("Invalid G2 point for delta2");
    let ic0_affine = ic0_uncomp.into_affine().expect("Invalid G1 point for ic0");
    let ic1_affine = ic1_uncomp.into_affine().expect("Invalid G1 point for ic1");

    let mut ic = Vec::new();
    ic.push(ic0_affine);
    ic.push(ic1_affine);

    VerifyingKey {
        alpha_g1: alpha1_affine,
        beta_g1: G1Affine::zero(),
        beta_g2: beta2_affine,
        gamma_g2: gamma2_affine,
        delta_g1: G1Affine::zero(),
        delta_g2: delta2_affine,
        ic,
    }
}