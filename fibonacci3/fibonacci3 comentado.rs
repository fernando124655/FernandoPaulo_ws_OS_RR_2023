use std::thread;                          // importa o módulo "thread" da biblioteca padrão do Rust

const MAX_IDX: u64 =                     // define uma constante para o maior índice a ser calculado
const CHUNK_SIZE: u64 = 10;              // define uma constante para o tamanho do "chunk" (pedaço) de cálculos a serem realizados em paralelo

fn main() {
    let mut handles = vec![];            // cria um vetor vazio que irá armazenar os identificadores das threads criadas

// para cada "chunk" de índices a serem calculados...
    for chunk in (0..=MAX_IDX).step_by(CHUNK_SIZE as usize) {
        let end_idx = (chunk + CHUNK_SIZE).min(MAX_IDX) - 1;   // define o índice final do "chunk"
        let handle = thread::spawn(move || {                   // cria uma nova thread para calcular os números de Fibonacci no "chunk"
            for i in chunk..=end_idx {                         // para cada índice no "chunk"...
                let result = fibonacci(i);                     // calcula o número de Fibonacci correspondente
                println!("fibonacci({}) = {}", i, result);     // imprime o resultado
            }
        });
        handles.push(handle);            // armazena o identificador da thread no vetor
    }

    for handle in handles {              // para cada identificador de thread...
        handle.join().unwrap();          // aguarda a thread finalizar
    }
}

// função recursiva que calcula o número de Fibonacci de um dado índice
fn fibonacci(n: u64) -> u64 {
    match n { // verifica o valor de n
        0 => 0, // se n = 0, retorna 0
        1 => 1, // se n = 1, retorna 1
        _ => fibonacci(n - 1) + fibonacci(n - 2),       // se n > 1, retorna a soma dos números de Fibonacci correspondentes aos dois índices anteriores
    }
}
