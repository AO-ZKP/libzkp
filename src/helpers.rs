
use common::{G1Point, G2Point};
use serde_json::{Value};
use lambdaworks_groth16::*;
use lambdaworks_math::{elliptic_curve::{short_weierstrass::{curves::{bls12_381::field_extension::Degree2ExtensionField, bls12_381::field_extension::BLS12381FieldModulus, bn_254::{curve::BN254Curve, field_extension::BN254PrimeField, pairing::BN254AtePairing}}, point::ShortWeierstrassProjectivePoint}, traits::IsEllipticCurve}, field::{element::FieldElement, fields::montgomery_backed_prime_fields::MontgomeryBackendPrimeField}};

use lambdaworks_math::unsigned_integer::element::UnsignedInteger;



#[inline]
pub fn circom_str_to_proof(value: Value) -> Proof {

    let pi1_vars: [FieldElement<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>; 3] = [
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("20366786490384464781523300859137110254534749757663007344537202359512744839921".to_string().as_str()).unwrap()),
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("1632730028121698871666862237109338339706950307296935188587531596249774171296".to_string().as_str()).unwrap()),
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("1".to_string().as_str()).unwrap()),
    ];    

    let pi2_vars: [FieldElement<Degree2ExtensionField>; 3] = [
        // X coordinate
        FieldElement::<Degree2ExtensionField>::from_raw([
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("5731908340196060836509326490101225874756105985788386761526608363374270330739".to_string().as_str()).unwrap()),
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("9760732879439821787253904407520150637722941622377745007073784232715502367563".to_string().as_str()).unwrap())
        ]),
        // Y coordinate
        FieldElement::<Degree2ExtensionField>::from_raw([
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("18453554185372640907808242492298116716350954610694204097931396933117337026664".to_string().as_str()).unwrap()),
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("15959269512884994911167213520796420464715988208373113073472115505055666150183".to_string().as_str()).unwrap())
        ]),        
        // Z coordinate
        FieldElement::<Degree2ExtensionField>::from_raw([
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("1".to_string().as_str()).unwrap()),
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("0".to_string().as_str()).unwrap())
        ]),    ];

    let pi3_vars: [FieldElement<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>; 3] = [
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("8807217090969397126669485058027111936466112062878930365674055859535394878983".to_string().as_str()).unwrap()),
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("5160513827476884578365183067515278263589892326512489196891066243863851435345".to_string().as_str()).unwrap()),
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus,6>>::from_raw(UnsignedInteger::<6>::from_dec_str("1".to_string().as_str()).unwrap()),
    ]; 

    // let g1_vars = [
    //     FrElement::from(&UnsignedInteger::<4>::from_dec_str(value["pi_a"][0].to_string().as_str()).unwrap()),
    //     FrElement::from(&UnsignedInteger::<4>::from_dec_str(value["pi_a"][1].to_string().as_str()).unwrap()),
    //     FrElement::from(&UnsignedInteger::<4>::from_dec_str(value["pi_a"][2].to_string().as_str()).unwrap()),
    // ]   

    let pi1 = G1Point::new(pi1_vars);
    let pi2 = G2Point::new(pi2_vars);
    let pi3 = G1Point::new(pi3_vars);

    let proof = Proof {
        pi1,
        pi2,
        pi3, 
    };

    return proof;

}
