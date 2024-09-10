package groth16wrapper

import (
	"fmt"

	"github.com/consensys/gnark-crypto/ecc"
	"github.com/consensys/gnark/backend/groth16"
	"github.com/consensys/gnark/frontend"
	"github.com/consensys/gnark/frontend/cs/r1cs"
)

// SimpleCircuit defines a basic circuit for testing
type SimpleCircuit struct {
	X frontend.Variable `gnark:",public"`
	Y frontend.Variable `gnark:",public"`
}

// Define implements the circuit logic
func (circuit *SimpleCircuit) Define(api frontend.API) error {
	api.AssertIsEqual(api.Mul(circuit.X, circuit.X), circuit.Y)
	return nil
}

// RunGroth16Test performs a complete Groth16 test
func RunGroth16Test() (int, error) {
	// Compile the circuit
	var circuit SimpleCircuit
	ccs, err := frontend.Compile(ecc.BN254.ScalarField(), r1cs.NewBuilder, &circuit)
	if err != nil {
		return 0, fmt.Errorf("compile circuit: %w", err)
	}

	// Setup
	pk, vk, err := groth16.Setup(ccs)
	if err != nil {
		return 0, fmt.Errorf("setup: %w", err)
	}

	// Generate a valid witness
	assignment := &SimpleCircuit{
		X: 3,
		Y: 9,
	}
	witness, err := frontend.NewWitness(assignment, ecc.BN254.ScalarField())
	if err != nil {
		return 0, fmt.Errorf("new witness: %w", err)
	}

	// Prove
	proof, err := groth16.Prove(ccs, pk, witness)
	if err != nil {
		return 0, fmt.Errorf("prove: %w", err)
	}

	// Extract public witness
	publicWitness, err := witness.Public()
	if err != nil {
		return 0, fmt.Errorf("public witness: %w", err)
	}

	// Verify
	err = groth16.Verify(proof, vk, publicWitness)
	if err != nil {
		return 0, fmt.Errorf("verify: %w", err)
	}

	return 1, nil
}
