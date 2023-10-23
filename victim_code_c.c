#include <stdio.h>

#define ARRAY_SIZE 16
int array[ARRAY_SIZE];
int secret = 42;

void victim_function(size_t x) {
    if (x < ARRAY_SIZE) {
        int value = array[x];
    }
}

int main() {
    size_t x = 5; // The attacker's goal is to leak the 'secret' variable

    // Training the branch predictor
    for (size_t i = 0; i < ARRAY_SIZE; i++) {
        victim_function(i);
    }

    // The attacker's code
    int value = array[x]; // Out-of-bounds access, but speculative execution occurs

    // Some code that depends on 'value' to make it observable
    if (value < 100) {
        // This is the side channel; you could measure the time it takes to execute.
    }

    return 0;
}
