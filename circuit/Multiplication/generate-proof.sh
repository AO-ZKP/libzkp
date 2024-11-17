#!/bin/bash
set -e

# Download Powers of Tau file if it doesn't exist
if [ ! -f "pot12_final.ptau" ]; then
    echo "Downloading Powers of Tau file..."
    wget https://hermez.s3-eu-west-1.amazonaws.com/powersOfTau28_hez_final_12.ptau -O pot12_final.ptau
fi

# Compile the circuit
echo "Compiling circuit..."
circom multiplier.circom --r1cs --wasm --sym

# Create the input
echo "Creating input file..."
echo '{"a": "3", "b": "5"}' > input.json

# Generate the witness
echo "Generating witness..."
node multiplier_js/generate_witness.js multiplier_js/multiplier.wasm input.json witness.wtns

# Generate proving key
echo "Generating proving key..."
snarkjs groth16 setup multiplier.r1cs pot12_final.ptau multiplier_0000.zkey

# Generate proof
echo "Generating proof..."
snarkjs groth16 prove multiplier_0000.zkey witness.wtns proof.json public.json

# Export verification key
echo "Exporting verification key..."
snarkjs zkey export verificationkey multiplier_0000.zkey verification_key.json

echo "Done! Check proof.json and verification_key.json"