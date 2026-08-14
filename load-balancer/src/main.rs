use std::io::{Read, Write};

// Importa os componentes de rede 
use std::net::{TcpListener, TcpStream};

fn processa_clinte(mut client_stream: TcpStream){
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
            let requisicao = String::from_utf8_lossy(&buffer[..bytes_lidos]);

            println!("---- Nova Requisição ----");
            println!("{}", requisicao);

            // Conecta ao servidor backend
            let endereco_backend = "127.0.0.1:8081"; // Vou usar a porta 8081 para o backend
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
            let bytes_resposta = match backent_stream.read(&mut buffer_resposta){
                Ok(bytes) =>{
                    if bytes == 0{
                        println!("Conexão do backend fechada");
                        return;
                    }

                    println!("Resposta gerada (funciona pelo amor de Deus");

                    bytes // retorna o valor para o 'let bytes_resposta'
                }
                Err(e) => {
                    println!("Erro ao ler resposta do backend: {}", e);
                    return;
                }
            };

            // Devolve a resposta do backend para o cliente
            if let Err(e) = client_stream.write_all(&buffer_resposta[..bytes_resposta]){
                println!("Erro ao devolver resposta para o cliente: {}", e);
            }

        }
        Err(e) => {
            println!("Erro ao ler do socket: {}", e);
        }
    }
}

fn main(){
    // Define o endereço e a porta que o load balancer vai escutar
    let endereco = "127.0.0.1:8080";
    let listener = TcpListener::bind(endereco).expect("Erro na porta 8080");

    println!("Load Balancer escutando em {}", endereco);

    // loop infinito que mantem o servidor rodando, esperando conexão
    for stream in listener.incoming(){
        match stream{
            Ok(stream) => {
                println!("Nova conexão");

                processa_clinte(stream);
            }
            Err(e) => {
                println!("Erro ao aceitar conexão: {}", e);
            }
        }
    }
}