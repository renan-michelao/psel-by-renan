use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;


fn extrai_caminho_arquivo(requisicao: &str) -> String   {
    if let Some(primeira_linha) = requisicao.lines().next() {

        // Divide a linha em pedaços usando espaços em branco
        let partes: Vec<&str> = primeira_linha.split_whitespace().collect();

        // garante que a requisição está bem formatada (pelo menos método e caminho)
        if partes.len() >= 2 {
            let metodo = partes[0];
            let caminho = partes[1];

            println!("Método: {}", metodo);
            println!("Caminho: {}", caminho);


            if caminho == "/" {
                return String::from("index.html"); // retorna um arquivo padrão
            } else{
                return caminho[1..].to_string();
            }
        }

    }

    // Retorno padrão caso a requisição esteja errada
    String::from("404.html")
}

fn backend_cabuloso(mut stream: TcpStream) {
    let mut buffer = [0; 4096];

    if let Ok(bytes_lidos) = stream.read(&mut buffer) {
        if bytes_lidos > 0{
            // Converte a requisição para um string
            let requisicao_str = String::from_utf8_lossy(&buffer[..bytes_lidos]);
            println!("bateu no 3");

            // Extrai qual arquivo o navegador está pedindo
            let nome_do_arquivo = extrai_caminho_arquivo(&requisicao_str);
            println!("Backend vai tentar ler: {}", nome_do_arquivo);

            match fs::read(&nome_do_arquivo) {
                Ok(conteudo_do_arquivo) => {
                    println!("Arquivo encontrado. manda o bglh pro load balancer");

                    let cabecalho = "HTTP/1.1 200\r\n\r\n";

                    let mut resposta_completa = cabecalho.as_bytes().to_vec();
                    resposta_completa.extend(conteudo_do_arquivo);

                    // Escreve tudo de volta no socket
                    if let Err(e) = stream.write_all(&resposta_completa) {
                        println!("Erro ao enviar a resposta: {}", e);
                    }

                }
                Err(_) => {
                    println!("Arquivo não encontrado. 404");

                    let cabecalho = "HTTP/1.1 404 Not Found\r\n\r\n";

                    let body_error = "<html><body><h1>Erro 404: o pato nao encontrou o bglh</h1></body></html>";

                    // Junta tudo
                    let resposta_erro = format!("{}{}", cabecalho, body_error);

                    if let Err(e) = stream.write_all(resposta_erro.as_bytes()){
                        println!("Erro ao enviar erro 404: {}", e);
                    }
                }
            }
        }
    }
}

fn main() {
    let endereco = "127.0.0.1:8083";
    let listener = TcpListener::bind(endereco).expect("Erro na porta 8081");
    println!("Backend escutando em {}", endereco);

    for stream in listener.incoming() {
        if let Ok(stream) = stream {
            backend_cabuloso(stream);
        }
    }
}