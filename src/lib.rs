use lambdaworks_groth16::{
    common::FrElement,
    setup, verify, Prover,
    QuadraticArithmeticProgram as QAP,
};

// Define the Vitalik QAP (x^3 + x + 5 = 35)
fn vitalik_qap() -> QAP {
    let num_of_public_inputs = 1;
    let [l, r, o] = [
        [
            ["0", "0", "0", "5"], // 1
            ["1", "0", "1", "0"], // x
            ["0", "0", "0", "0"], // ~out
            ["0", "1", "0", "0"], // sym_1
            ["0", "0", "1", "0"], // y
            ["0", "0", "0", "1"], // sym_2
        ],
        [
            ["0", "0", "1", "1"],
            ["1", "1", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
        ],
        [
            ["0", "0", "0", "0"],
            ["0", "0", "0", "0"],
            ["0", "0", "0", "1"],
            ["1", "0", "0", "0"],
            ["0", "1", "0", "0"],
            ["0", "0", "1", "0"],
        ],
    ]
    .map(|matrix| matrix.map(|row| row.map(FrElement::from_hex_unchecked).to_vec()));
    QAP::from_variable_matrices(num_of_public_inputs, &l, &r, &o)
}

#[no_mangle]
pub extern "C" fn rust_test() -> i32 {
    // Set up the QAP
    let qap = vitalik_qap();
    println!("QAP is set up");
    // Generate proving and verifying keys
    let (pk, vk) = setup(&qap);

    // Create a witness (x = 3)
    let w = ["0x1", "0x3", "0x23", "0x9", "0x1b", "0x1e"]
        .map(FrElement::from_hex_unchecked)
        .to_vec();

    // Generate the proof
    let proof = Prover::prove(&w, &qap, &pk);

    // Verify the proof
    let public_inputs = &w[..qap.num_of_public_inputs];
    let is_valid = verify(&vk, &proof, public_inputs);
    
    // Return 1 if valid, 0 otherwise
    if is_valid {
        1
    } else {
        0
    }
    
    
}