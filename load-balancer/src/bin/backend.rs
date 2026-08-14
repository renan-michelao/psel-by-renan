use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn backend_cabuloso(mut stream: TcpStream) {
    let mut buffer = [0; 4096];
    
    if let Ok(bytes_lidos) = stream.read(&mut buffer) {
        if bytes_lidos > 0 {
            println!("Backend recebeu um quack (requisição)");   

            let status_line = "HTTP/1.1 200 OK\r\n";
            let blank_line = "\r\n";
            let body = "<html><body><h1>Toma essa resposta quack (censura)</h1></body></html>";

            let resposta = format!("{}{}{}", status_line, blank_line, body);

            // Escreve a resposta de volta para o socket (envia de volta para o balancer)
            if let Err(e) = stream.write_all(resposta.as_bytes()){
                println!("Erro ao enviar resposta do back: {}", e);
            }
        }
    }
}

fn main() {
    let endereco = "127.0.0.1:8081";
    let listener = TcpListener::bind(endereco).expect("Erro na porta 8081");
    println!("Backend escutando em {}", endereco);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            backend_cabuloso(stream);
        }
    }
}