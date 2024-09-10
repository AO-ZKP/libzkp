#include <stdio.h>

// Declare the function from the Go code
extern int runGroth16Test();

int main() {
    printf("Running Groth16 test...\n");
    int result = runGroth16Test();
    if (result == 1) {
        printf("Groth16 test successful!\n");
    } else {
        printf("Groth16 test failed.\n");
    }
    return 0;
}