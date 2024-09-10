package main

import (
	"fmt"
	"groth16example/groth16wrapper"
	"syscall/js"
)

func main() {
	fmt.Println("WebAssembly module loaded")
	js.Global().Set("runGroth16Test", js.FuncOf(runGroth16Test))
	<-make(chan bool)
}

func runGroth16Test(this js.Value, args []js.Value) interface{} {
	result, err := groth16wrapper.RunGroth16Test()
	if err != nil {
		fmt.Printf("Error in Groth16 test: %v\n", err)
		return 0
	}
	return result
}
