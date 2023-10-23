const ARRAY_SIZE: usize = 16;
static mut ARRAY: [i32; ARRAY_SIZE] = [0; ARRAY_SIZE];
static mut SECRET: i32 = 42;

fn victim_function(x: usize) {
    unsafe {
        if x < ARRAY_SIZE {
            let _value = ARRAY[x];
        }
    }
}

fn main() {
    let x: usize = 5; // The attacker's goal is to leak the 'SECRET' variable

    // Training the branch predictor
    for i in 0..ARRAY_SIZE {
        victim_function(i);
    }

    // The attacker's code
    let value: i32;
    unsafe {
        value = ARRAY[x]; // Out-of-bounds access, but speculative execution occurs
    }

    // Some code that depends on 'value' to make it observable
    if value < 100 {
        // This is the side channel; you could measure the time it takes to execute.
    }
}
