use std::thread;

fn main() {
    let mut fib_nums: Vec<u64> = vec![0, 1]; // Inicia a sequência com os números 0 e 1
    let mut threads = Vec::new();

    // Define uma função para calcular a sequência de Fibonacci em uma faixa de valores
    fn calculate_fib_range(start: u64, end: u64, fib_nums: &mut Vec<u64>) {
        for i in start..end {
            let next_fib = fib_nums[i as usize - 1] + fib_nums[i as usize - 2];
            fib_nums.push(next_fib);
        }
    }

    // Dispara uma thread para cada faixa de 1000 valores
    for i in (2..1000).step_by(1000) {
        let start = i;
        let end = i + 1000;
        let fib_nums_ref = &mut fib_nums;
        let handle = thread::spawn(move || calculate_fib_range(start, end, fib_nums_ref));
        threads.push(handle);
    }

    // Aguarda todas as threads terminarem antes de continuar
    for handle in threads {
        handle.join().unwrap();
    }

    // Imprime os números da sequência de Fibonacci até o número 1.000.000
    for i in 0..fib_nums.len() {
        if fib_nums[i] > 1_000_000 {
            break;
        }
        println!("{}", fib_nums[i]);
    }
}
