use std::net::{TcpListener};

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
            }
            Err(e) => {
                println!("Erro ao aceitar conexão: {}", e);
            }
        }
    }
}