# Journey 0: Inciando

Eu decidi fazer o projeto do Load Balacener em Rust. Mas antes de começar, eu preciso aprender o básico da sintaxe do Rust. Então eu crie a pasta "rust_basic" com alguns exercícios básicos em Rust. (Não sei se precisa enviar junto com o projeto, mas por via de duvidas irei fazer o commit desses exercícios também)


# Journey 1: O que é um Load Balancer?

O Load Balancer é um método de distribuir o tráfego de rede para o backend. Ele repassa as requisições de forma a garantir que nenhum servidor fique sobrecarregado enquanto outro fica vago.

Vou usar o algoritmo Round-Robin para fazer o roteameto:

# Proxy Reverso

Um servidor web comum recebe uma mensagem e devolve um arquivo. O Load Balancer funciona como um proxy reverso, ou seja, é um servidor intermediário que fica entre o usuário/cliente e o servidor web. O Load Balancer atua como servidor para o usuário e como cliente para o backend.
       
Usuário -> Load Balancer (proxy reverso) -> Servidor Web

# Protocolo HTTP

Um pacote HTTP é estruturalmente apenas um bloco de texto padronizado. É dividido basicamente entre as seguintes partes:

*Request Line:* A primeira linha, contendo o Método, a Rota e a Versão. (Ex: GET /imagens/foto.png HTTP/1.1\r\n).

*Headers:* Linhas seguindo o formato *Chave: Valor*. (Ex: Host: localhost:8080\r\n).

*Body:* O conteúdo do pacote, usado quando fazemos requisições POST para enviar um arquivo.

OBS: A sequência de caracteres *\r\n\r\n* indica que o cabeçalho acabou.