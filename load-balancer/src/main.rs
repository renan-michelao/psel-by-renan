use std::io::{Read, Write};

// Importa os componentes de rede 
use std::net::{TcpListener, TcpStream};

use std::thread;
use std::sync::{Arc, Mutex}; // componentes para lidar com memória compartilhada entre threads

fn processa_clinte(mut client_stream: TcpStream, endereco_backend: String){
    // Buffer de 4k preenchido com zeros
    let mut buffer = [0; 4096];

    // Lê os dados (requisição) do socket e coloca no buffer
    match client_stream.read(&mut buffer){
        Ok(bytes_lidos) => {
            if bytes_lidos == 0 {
                println!("Conexão fechada.");
                return;
            }

            // Converte os bytes para uma string
            //let requisicao = String::from_utf8_lossy(&buffer[..bytes_lidos]);

            println!("---- Nova Requisição ----");
            println!("Encaminhando para o backend: {}", endereco_backend);
            //println!("{}", requisicao);

            // Conecta ao servidor backend
            //let endereco_backend = "127.0.0.1:8081"; // Vou usar a porta 8081 para o backend
            let mut backent_stream = match TcpStream::connect(endereco_backend){
                Ok(stream) => {
                    println!("Backend conectado");
                    stream
                }
                Err(e) => {
                    println!("Erro ao conectar no backend {}", e);
                    return;
                }
            };

            // Repassa os bytes da requisição para o backend
            if let Err(e) = backent_stream.write_all(&buffer[..bytes_lidos]) {
                println!("Erro ao enviar requisição para o back: {}", e);
                return;
            }

            // Lê a resposta que o backend gerou
            let mut buffer_resposta = [0; 4096]; // Buffer para a resposta do back
            
            println!("Iniciando transferênca da resposta");

            loop {
                match backent_stream.read(&mut buffer_resposta){
                    Ok(bytes_lidos) => {
                        if bytes_lidos == 0{
                            println!("transferênca concluida. Fim do arquivo");
                            break;
                        }

                        // Se leu alguma coisa, pega exatamente esse pedaço e manda para o cliente
                        if let Err(e) = client_stream.write_all(&buffer_resposta[..bytes_lidos]) {
                            println!("Erro ao repassar pedaço para o cliente: {}", e);
                            break;
                        }
                    }
                    Err(e) => {
                        println!("Erro ao ler pedaço do backend: {}", e);
                        break;
                    }
                }
            }
        }
        Err(e) => {
            println!("Erro ao ler do socket: {}", e);
        }
    }
}

fn main(){

    // Lista com os servidores backend
    let lista_backends = vec![
        String::from("127.0.0.1:8081"),
        String::from("127.0.0.1:8082"),
        String::from("127.0.0.1:8083"),
    ];

    let contador_round_robin = Arc::new(Mutex::new(0));

    // Define o endereço e a porta que o load balancer vai escutar
    let endereco = "127.0.0.1:8080";
    let listener = TcpListener::bind(endereco).expect("Erro na porta 8080");

    println!("Load Balancer escutando em {}", endereco);

    // loop infinito que mantem o servidor rodando, esperando conexão
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                println!("Nova conexão");

                // Faz uma cópia das variáveis para jogar dentro da thread
                let contador_clone = Arc::clone(&contador_round_robin);
                let backends_clone = lista_backends.clone();

                thread::spawn(move || {
                    // Trava o mutex
                    let mut indice_atual = contador_clone.lock().unwrap();
                    let backend_escolhido = backends_clone[*indice_atual].clone();

                    *indice_atual = *indice_atual + 1;

                    // Se passou do tamanho da lista -> volta pra zero (round robin)
                    if *indice_atual >= backends_clone.len(){
                        *indice_atual = 0;
                    }

                    // Tira a trava do mutex
                    drop(indice_atual);

                    processa_clinte(stream, backend_escolhido);
                });
            }
            Err(e) => {
                println!("Erro ao aceitar conexão: {}", e);
            }
        }
    }
}