extern crate core_affinity;

use std::arch::asm;
use std::mem;
use std::ptr;
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};

const CACHE_HIT_THRESHOLD: u64 = 80;
const NUM_TRIES: u64 = 1000;
const TRAINING_LOOPS: u64 = 100;
const ATTACK_LEAP: u64 = 10;
const INBETWEEN_DELAY: u64 = 100;
const LIKELY_THRESHOLD: u64 = (0.7 * NUM_TRIES as f64) as u64;
const SECRET: &str = "EECS 573";

unsafe fn clflush(addr: *const u8) {
    asm!("clflush [$0]" :: "r"(addr) :: "volatile");
}

fn init_attack() -> (Vec<bool>, Vec<u8>) {
    let mut is_attack = vec![false; TRAINING_LOOPS as usize];
    let mut attack_pattern = (0..256).collect::<Vec<u8>>();
    let seed = Instant::now().elapsed().as_nanos() as u64;

    attack_pattern.shuffle(&mut rand::thread_rng());

    for (i, is_attack_loop) in is_attack.iter_mut().enumerate() {
        *is_attack_loop = i % ATTACK_LEAP as usize == 0;
    }

    (is_attack, attack_pattern)
}

fn read_memory_byte(target_idx: usize, arr1_size: usize, attack_pattern: Vec<u8>) -> String {
    let mut secret = String::new();
    let (tx, rx) = mpsc::channel();

    for try in (1..=NUM_TRIES).rev() {
        clflush(&arr1_size);

        let train_idx = try % arr1_size;
        let mut results = [0; 256];

        for i in (0..TRAINING_LOOPS).rev() {
            clflush(&arr1_size);

            for _ in 0..INBETWEEN_DELAY {
                // Wait for in-between delay cycles
            }

            let idx = if is_attack[i as usize] {
                target_idx
            } else {
                train_idx
            };

            // Call the victim function with the training_x (to mistrain branch predictor) or target_x (to attack the SECRET address)
            fetch_function(idx);

            // Here, implement the timing attack logic to measure cache access times for each character
            // and update the `results` array
        }

        // Here, calculate the most likely character and push it into the `secret` string
    }

    secret
}

fn fetch_function(idx: usize) -> i32 {
    // Should simulate the behavior of the C++ `fetch_function`
    // by returning values from the shared memory, based on the `idx`.
    // It should also incorporate the cache access time measurement logic.
    // This part requires low-level control and may involve inline assembly.
    // Unfortunately, implementing this in Rust is not fun :(

    // Placeholder code
    0
}

fn main() {
    // Need to set up shared memory for arr1 and arr2, as in the C++ code.
    // Use appropriate Rust memory management techniques to accomplish this

    let arr1_size = 16;
    let target_idx = secret.as_ptr() as usize - arr1.as_ptr() as usize;
    let (is_attack, attack_pattern) = init_attack();
    let guessed_secret = read_memory_byte(target_idx, arr1_size, attack_pattern);

    println!("THE GUESSED SECRET IS :: {}", guessed_secret);
}
