use common::{G1Point, G2Point, PairingOutput};
use lambdaworks_groth16::*;
use lambdaworks_math::{
    elliptic_curve::{
        short_weierstrass::{
            curves::{
                bls12_381::field_extension::BLS12381FieldModulus,
                bls12_381::field_extension::Degree2ExtensionField,
                bn_254::{
                    curve::BN254Curve, field_extension::BN254PrimeField, pairing::BN254AtePairing,
                },
            },
            point::ShortWeierstrassProjectivePoint,
        },
        traits::IsEllipticCurve,
    },
    field::{
        element::FieldElement, fields::montgomery_backed_prime_fields::MontgomeryBackendPrimeField,
    },
};
use serde_json::{json, value, Value};

use lambdaworks_math::unsigned_integer::element::UnsignedInteger;

#[inline]
pub fn circom_str_to_proof(value: Value) -> Proof {
    let pi1_vars: [FieldElement<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>; 3] = [
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
            UnsignedInteger::<6>::from_dec_str(
                "20366786490384464781523300859137110254534749757663007344537202359512744839921"
                    .to_string()
                    .as_str(),
            )
            .unwrap(),
        ),
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
            UnsignedInteger::<6>::from_dec_str(
                "1632730028121698871666862237109338339706950307296935188587531596249774171296"
                    .to_string()
                    .as_str(),
            )
            .unwrap(),
        ),
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
            UnsignedInteger::<6>::from_dec_str("1".to_string().as_str()).unwrap(),
        ),
    ];

    let pi2_vars: [FieldElement<Degree2ExtensionField>; 3] = [
        // X coordinate
        FieldElement::<Degree2ExtensionField>::from_raw([
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                UnsignedInteger::<6>::from_dec_str(
                    "5731908340196060836509326490101225874756105985788386761526608363374270330739"
                        .to_string()
                        .as_str(),
                )
                .unwrap(),
            ),
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                UnsignedInteger::<6>::from_dec_str(
                    "9760732879439821787253904407520150637722941622377745007073784232715502367563"
                        .to_string()
                        .as_str(),
                )
                .unwrap(),
            ),
        ]),
        // Y coordinate
        FieldElement::<Degree2ExtensionField>::from_raw([
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                UnsignedInteger::<6>::from_dec_str(
                    "18453554185372640907808242492298116716350954610694204097931396933117337026664"
                        .to_string()
                        .as_str(),
                )
                .unwrap(),
            ),
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                UnsignedInteger::<6>::from_dec_str(
                    "15959269512884994911167213520796420464715988208373113073472115505055666150183"
                        .to_string()
                        .as_str(),
                )
                .unwrap(),
            ),
        ]),
        // Z coordinate
        FieldElement::<Degree2ExtensionField>::from_raw([
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                UnsignedInteger::<6>::from_dec_str("1".to_string().as_str()).unwrap(),
            ),
            FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                UnsignedInteger::<6>::from_dec_str("0".to_string().as_str()).unwrap(),
            ),
        ]),
    ];

    let pi3_vars: [FieldElement<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>; 3] = [
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
            UnsignedInteger::<6>::from_dec_str(
                "8807217090969397126669485058027111936466112062878930365674055859535394878983"
                    .to_string()
                    .as_str(),
            )
            .unwrap(),
        ),
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
            UnsignedInteger::<6>::from_dec_str(
                "5160513827476884578365183067515278263589892326512489196891066243863851435345"
                    .to_string()
                    .as_str(),
            )
            .unwrap(),
        ),
        FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
            UnsignedInteger::<6>::from_dec_str("1".to_string().as_str()).unwrap(),
        ),
    ];

    // let g1_vars = [
    //     FrElement::from(&UnsignedInteger::<4>::from_dec_str(value["pi_a"][0].to_string().as_str()).unwrap()),
    //     FrElement::from(&UnsignedInteger::<4>::from_dec_str(value["pi_a"][1].to_string().as_str()).unwrap()),
    //     FrElement::from(&UnsignedInteger::<4>::from_dec_str(value["pi_a"][2].to_string().as_str()).unwrap()),
    // ]

    let pi1 = G1Point::new(pi1_vars);
    let pi2 = G2Point::new(pi2_vars);
    let pi3 = G1Point::new(pi3_vars);

    let proof = Proof { pi1: pi1.clone(), pi2: pi2.clone(), pi3 };
        



    return proof;
}

pub fn circom_vk_to_vk() {

    let pairing_ab = json!([
        [
         [
          "5218699279749164827991347544662799904271573280134032133211789535652147036054",
          "17663964469993182029520551577864080022294837722332758385511916352119870524218"
         ],
         [
          "2407432915808372415619911789771905141159889362245586991275411364704616534869",
          "5798629906625362827486134996064072065806941514176498872838886465588659536372"
         ],
         [
          "14745473602596267030841352365024779739116862471759311924822898463140348858682",
          "8507477649830826017313947021647276785259462062813515324034895072797097460271"
         ]
        ],
        [
         [
          "18614486260417423853241793642413121767507493558349718279094260780304488717997",
          "1032334053609539643034290495443234760828005515256485354874645078240388956384"
         ],
         [
          "18688809649886288127880974121303299407366654192824871098021579494708205988717",
          "9283793592805488327758544864015302139584913369277884412402055718653400657869"
         ],
         [
          "5145116425825844036544625539134557098264149222251988051267487309305584385965",
          "18580122997736999027117155184038371495838099789425611451910230154542147194745"
         ]
        ]
    ]).as_array().unwrap();

    
    let vk = VerifyingKey {
        alpha_g1_times_beta_g2: PairingOutput::,
        
        delta_g2: ShortWeierstrassProjectivePoint::new([
            FieldElement::<Degree2ExtensionField>::from_raw([
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "10857046999023057135944570762232829481370756359578518086990519993285655852781"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "11559732032986387107991004021392285783925812861821192530917403151452391805634"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
            ]),
            FieldElement::<Degree2ExtensionField>::from_raw([
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "8495653923123431417604973247489272438418190587263600148770280649306958101930"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "4082367875863433681332203403145435568316851327593401208105741076214120093531"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
            ]),
            FieldElement::<Degree2ExtensionField>::from_raw([
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "1"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "0"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
            ]),
        ]),
        
        gamma_g2: ShortWeierstrassProjectivePoint::new([
            FieldElement::<Degree2ExtensionField>::from_raw([
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "10857046999023057135944570762232829481370756359578518086990519993285655852781"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "11559732032986387107991004021392285783925812861821192530917403151452391805634"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
            ]),
            FieldElement::<Degree2ExtensionField>::from_raw([
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "8495653923123431417604973247489272438418190587263600148770280649306958101930"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "4082367875863433681332203403145435568316851327593401208105741076214120093531"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
            ]),
            FieldElement::<Degree2ExtensionField>::from_raw([
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "1"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
                FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                    UnsignedInteger::<6>::from_dec_str(
                        "0"
                            .to_string()
                            .as_str(),
                    )
                    .unwrap(),
                ),
            ]),
        ]),
        verifier_k_tau_g1: vec![
            ShortWeierstrassProjectivePoint::new(
                [
                    FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                        UnsignedInteger::<6>::from_dec_str(
                            "1792693912222574621929804668277963490387773786020974279683800606140814326737"
                                .to_string()
                                .as_str(),
                        )
                        .unwrap(),
                    ),
                    FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                        UnsignedInteger::<6>::from_dec_str(
                            "15665603553563380371129631705066648596625079869016881864842346246744039023347"
                                .to_string()
                                .as_str(),
                        )
                        .unwrap(),
                    ),
                    FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                        UnsignedInteger::<6>::from_dec_str(
                            "1"
                                .to_string()
                                .as_str(),
                        )
                        .unwrap(),
                    )
                ]
            ),
            ShortWeierstrassProjectivePoint::new(
                [
                    FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                        UnsignedInteger::<6>::from_dec_str(
                            "11148478839883439331585272089516203892855957333759614498148015389555788355477"
                                .to_string()
                                .as_str(),
                        )
                        .unwrap(),
                    ),
                    FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                        UnsignedInteger::<6>::from_dec_str(
                            "2816257723750387661166313635693873182375006053010715363636453491495481986715"
                                .to_string()
                                .as_str(),
                        )
                        .unwrap(),
                    ),
                    FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                        UnsignedInteger::<6>::from_dec_str(
                            "1"
                                .to_string()
                                .as_str(),
                        )
                        .unwrap(),
                    )
                ]
            ),
            ShortWeierstrassProjectivePoint::new(
                [
                    FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                        UnsignedInteger::<6>::from_dec_str(
                            "18260769349849311377733163703468128888160887642312432328963512083019762066903"
                                .to_string()
                                .as_str(),
                        )
                        .unwrap(),
                    ),
                    FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                        UnsignedInteger::<6>::from_dec_str(
                            "15316680033775092865305869241260503106643340863838303346037303633153138014277"
                                .to_string()
                                .as_str(),
                        )
                        .unwrap(),
                    ),
                    FieldElement::<MontgomeryBackendPrimeField<BLS12381FieldModulus, 6>>::from_raw(
                        UnsignedInteger::<6>::from_dec_str(
                            "1"
                                .to_string()
                                .as_str(),
                        )
                        .unwrap(),
                    )
                ]
            ),
        
        ]
    };

    

}
