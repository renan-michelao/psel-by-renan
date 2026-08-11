use std::io::Read;

// Importa os componentes de rede 
use std::net::{TcpListener, TcpStream};

fn processa_clinte(mut stream: TcpStream){
    // Buffer de 4k preenchido com zeros
    let mut buffer = [0; 4096];

    // Lê os dados do socket e coloca no buffer
    match stream.read(&mut buffer){
        Ok(bytes_lidos) => {
            if bytes_lidos == 0 {
                println!("Conexão fechada.");
                return;
            }

            // Converte os bytes para uma string
            let requisicao = String::from_utf8_lossy(&buffer[..bytes_lidos]);

            println!("---- Nova Requisição ----");
            println!("{}", requisicao);
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