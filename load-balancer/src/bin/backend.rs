use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::fs;

fn extrai_caminho_arquivo(requisicao: &str) -> (String, String)   {
    if let Some(primeira_linha) = requisicao.lines().next() {

        // Divide a linha em pedaços usando espaços em branco
        let partes: Vec<&str> = primeira_linha.split_whitespace().collect();

        // garante que a requisição está bem formatada (pelo menos método e caminho)
        if partes.len() >= 2 {
            let metodo = partes[0].to_string();
            let caminho = partes[1];

            println!("Método: {}", metodo);
            println!("Caminho: {}", caminho);


            let caminho_formatado = if caminho == "/" {
                String::from("index.html")
            } else{
                caminho[1..].to_string()
            };

            return (metodo, caminho_formatado);

        }

    }

    // Retorno padrão caso a requisição esteja errada
    (String::from("GET"), String::from("404.html"))
}

fn extrai_tamanho_arquivo(requisicao: &str) -> usize {
    for linha in requisicao.lines() {
        if linha.to_lowercase().starts_with("content-length") {
            let pedacos: Vec<&str> = linha.split(":").collect();

            if pedacos.len() == 2 { // caso tenha um espaço de separação entre : e o número
                // Pega o número (ex: " 15042"), remove os espaços e converte para número (usize)
                let numero_str = pedacos[1].trim();
                if let Ok(tamanho) = numero_str.parse::<usize>() {
                    return tamanho;
                }
            }
        }
    }

    0   // se não achar o cabeçalho, assume que o arquivo tme 0 bytes
}

fn backend_cabuloso(mut stream: TcpStream) {
    let mut buffer = [0; 4096];

    if let Ok(bytes_lidos) = stream.read(&mut buffer) {
        if bytes_lidos > 0{
            // Converte a requisição para um string
            let requisicao_str = String::from_utf8_lossy(&buffer[..bytes_lidos]);

            // Extrai qual arquivo o navegador está pedindo
            let (metodo, nome_do_arquivo) = extrai_caminho_arquivo(&requisicao_str);
            println!("Backend vai tentar ler: {}", nome_do_arquivo);

            if metodo == "POST" {
                println!("Método POST, salvando arquivo: {}", nome_do_arquivo);

                // O corpo do arquivo começa depois de dois \r\n seguidos
                let mut inicio_arquivo = 0;

                for i in 0..bytes_lidos.saturating_sub(3) {
                    if buffer[i] == b'\r' && buffer[i+1] == b'\n' && buffer[i+2] == b'\r' && buffer[i+3] == b'\n' {
                        inicio_arquivo = i + 4; // Pula os caracteres de quebra de linha
                        break;
                    }
                }

                // Pega os bytes do arquivo
                let mut bytes_do_arquivo = buffer[inicio_arquivo..bytes_lidos].to_vec();

                let tamanho = extrai_tamanho_arquivo(&requisicao_str);
                println!("tamanho do arquivo: {}", tamanho);

                while bytes_do_arquivo.len() < tamanho {
                    let mut buffer = [0; 4096];

                    match stream.read(&mut buffer) {
                        Ok(lidos) => {
                            if lidos == 0 {
                                break;
                            }

                            bytes_do_arquivo.extend_from_slice(&buffer[..lidos]);
                        }
                        Err(_) => break, // erro na rede
                    }
                }

                // tenta escrever no HD
                match fs::write(&nome_do_arquivo, &bytes_do_arquivo){
                    Ok(_) => {
                        println!("Arquivo salvo, vamooooooo");
                        let resposta = "HTTP/1.1 201 Created\r\n\r\n<h1>Arquivo salvo</h1>";
                        let _ = stream.write_all(resposta.as_bytes()); // manda a resposta para o cliente
                    }
                    Err(e) => {
                        println!("deu merda na hora de salvar o arquivo: {}", e);
                        let resposta = "HTTP/1.1 500 deu Internal Server Error\r\n\r\n<h1>deu red mano</h1>";
                        let _ = stream.write_all(resposta.as_bytes());
                    }
                }
            } else if metodo == "GET" {

            match fs::read(&nome_do_arquivo) {
                Ok(conteudo_do_arquivo) => {
                    println!("Arquivo encontrado. manda o bglh pro load balancer");

                    let cabecalho = "HTTP/1.1 200 OK\r\n\r\n";

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