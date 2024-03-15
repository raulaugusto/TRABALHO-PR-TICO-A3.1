//Importando as dependêndias necessárias
//
use std::io;
use std::io::{Write};
use std::thread;
use std::time::Duration;
use indicatif::{ProgressBar, ProgressStyle};

//Declaração da struct que representa os processos
struct Process {
    PID: i32,
    menSize: i32,
    timeExecution: i32,
}

//Execução principal do problema, interação com o usuário
fn main() {
    let initial_pid = 0;
    let mut processes_stack: Vec<Process> = vec![];
    let mut number_of_processes_input = String::new();
    let mut processes_summary = String::from("Resumo dos processos executados: ");

    println!("Quantos processos devem ser executados? ");
    io::stdin()
        .read_line(&mut number_of_processes_input)
        .expect("Falha ao ler o input");
    let number_of_processes: i32 = number_of_processes_input
        .trim()
        .parse()
        .expect("Falha em converter para inteiro");

    read_processes_data(&mut processes_stack, number_of_processes);

    execute_processes(&mut processes_stack, &mut processes_summary);
}

//Função responsável por coletar e armazenar as informações de cada processo
fn read_processes_data(processes_stack: &mut Vec<Process>, number_of_processes: i32) {
    let initial_pid = 0;

    for i in 1..=number_of_processes {
        let mut men_size_input = String::new();
        println!("Qual o tamanho de memoria do processo {}?", i);
        io::stdin()
            .read_line(&mut men_size_input)
            .expect("Falha ao ler a linha");
        let men_size: i32 = men_size_input.trim().parse().expect("Falha ao converter");

        let mut time_execution_input = String::new();
        println!("Qual o tempo de execução do processo {}?", i);
        io::stdin()
            .read_line(&mut time_execution_input)
            .expect("Falha ao ler a linha");
        let time_execution: i32 = time_execution_input.trim().parse().expect("Falha ao converter");

        let new_proc = Process {
            PID: initial_pid + i,
            menSize: men_size,
            timeExecution: time_execution,
        };
        processes_stack.push(new_proc);
    }
}

//Função que executa os processos à partir do último adicionado à stack, e realiza o registro das informações totais para o log final
fn execute_processes(processes_stack: &mut Vec<Process>, processes_summary: &mut String) {
    let mut men_size_sum = 0;
    let mut time_execution_sum = 0;

    for p in processes_stack.iter().rev() {
        println!("Processo com o PID: {} sendo executado", p.PID);
        
        let progress_bar = ProgressBar::new(p.timeExecution as u64);
        progress_bar.set_style(
            ProgressStyle::default_bar()
            .template("[{elapsed_precise}] {bar:40.red/blue} {pos}/{len} ({percent}%)")
            .progress_chars("#>-"),
        );
        
        for segundo in 0..p.timeExecution {
            thread::sleep(Duration::from_secs(1));
            progress_bar.inc(1);
        }
        progress_bar.finish();
        println!("Processo PID: {} finalizado com sucesso! \n", p.PID);
        
        let add_processes_summary_line = format!(
            "\nProcesso PID: {}, Tamanho de memória: {}, Tempo de execução {}",
            p.PID, p.menSize, p.timeExecution
        );
        processes_summary.push_str(&add_processes_summary_line);
        men_size_sum += p.menSize;
        time_execution_sum += p.timeExecution;
        
    }
    println!("{}", processes_summary);
    println!("\nUso de memória total: {}, Tempo de execução total: {}", men_size_sum, time_execution_sum);
}
