use std::thread;

const MAX_IDX: u64 = 93;
const CHUNK_SIZE: u64 = 10;

fn main() {
    let mut handles = vec![];

    for chunk in (0..=MAX_IDX).step_by(CHUNK_SIZE as usize) {
        let end_idx = (chunk + CHUNK_SIZE).min(MAX_IDX) - 1;
        let handle = thread::spawn(move || {
            for i in chunk..=end_idx {
                let result = fibonacci(i);
                println!("fibonacci({}) = {}", i, result);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }
}

fn fibonacci(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}
