use std::thread;

fn main() {
    let num_threads = 4;
    let mut handles = vec![];

    let fib_nums = vec![0; 50];

    let chunk_size = fib_nums.len() / num_threads;

    for i in 0..num_threads {
        let start = i * chunk_size;
        let end = if i == num_threads - 1 {
            fib_nums.len()
        } else {
            (i + 1) * chunk_size
        };

        let mut fib_nums_thread = vec![0; 50];

        let handle = thread::spawn(move || {
            calculate_fib_range(start, end, &mut fib_nums_thread);
            fib_nums_thread
        });

        handles.push(handle);
    }

    let mut result = vec![];
    for handle in handles {
        let fib_nums_thread = handle.join().unwrap();
        result.extend_from_slice(&fib_nums_thread);
    }

    for i in 0..result.len() {
        if result[i] > 1_000_000 {
            println!("fib({}) = {}", i, result[i]);
        }
    }

    // Adição do print
    println!("{:?}", result);
}

fn calculate_fib_range(start: usize, end: usize, fib_nums: &mut [u64]) {
    fib_nums[start] = 0;
    fib_nums[start + 1] = 1;

    for i in (start + 2)..end {
        fib_nums[i] = fib_nums[i - 1] + fib_nums[i - 2];
    }
}
