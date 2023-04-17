use std::thread;
/*Solulção 2 - Que tal usarmos Vetor para armazenar os resultados?!*/
fn main() {
    let num_threads = 4;                           // Número de threads que serão criadas
    let mut handles = vec![];                      // Vetor para armazenar as referências das threads
    let fib_nums = vec![0; 50];                    // Vetor para armazenar os números de Fibonacci, inicializado com 50 zeros
    let chunk_size = fib_nums.len() / num_threads; // Tamanho de cada "chunk" de números de Fibonacci. chunk_size=12

    for i in 0..num_threads {//Aqui ele roda as 4 threads pois ele loop é não inclusivo
        let start = i * chunk_size;                // Índice de início do chunk atual
        let end = if i == num_threads - 1 {
            fib_nums.len()                         // Se for a última thread, o índice final é o último elemento do vetor
        } else {
            (i + 1) * chunk_size                   // Caso contrário, o índice final é o início do próximo chunk
        };

        let mut fib_nums_thread = vec![0; 50];     // Vetor para armazenar os números de Fibonacci calculados pela thread atual
        let handle = thread::spawn(move || {
            calculate_fib_range(start, end, &mut fib_nums_thread);  // Calcula os números de Fibonacci do chunk atual e armazena no vetor da thread
            fib_nums_thread                        // Retorna o vetor com os números de Fibonacci calculados pela thread
        });

        handles.push(handle);                      // Armazena a referência da thread no vetor handles
    }

    let mut result = vec![];                       // Vetor para armazenar os números de Fibonacci calculados por todas as threads
    for handle in handles {
        let fib_nums_thread = handle.join().unwrap(); // Espera a thread terminar e obtém o vetor de números de Fibonacci calculados
        result.extend_from_slice(&fib_nums_thread); // Adiciona os números de Fibonacci calculados ao vetor result
    }

    for i in 0..result.len() {
        if result[i] > 1_000_000 {
            println!("fib({}) = {}", i, result[i]);  // Imprime o índice e o número de Fibonacci se for maior do que 1_000_000
        }
    }
    println!("{:?}", result);                       // Imprime o vetor completo com os números de Fibonacci
}

fn calculate_fib_range(start: usize, end: usize, fib_nums: &mut [u64]) {
    fib_nums[start] = 0;                            // Define o primeiro número de Fibonacci como zero
    fib_nums[start + 1] = 1;                        // Define o segundo número de Fibonacci como um
    for i in (start + 2)..end {
        fib_nums[i] = fib_nums[i - 1] + fib_nums[i - 2]; // Calcula os demais números de Fibonacci do chunk atual
    }
}
