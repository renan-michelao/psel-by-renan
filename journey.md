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


# Journey 2: Iniciando o backend

Comecei a fazer o backend que será o servidor de arquivos. Por enquanto, o backend está extremamente simples, apenas retorna
um HTML simples de teste. 

Bom, até o momento deste commit, o que eu tenho é um esqueleto de um load balancer (ainda não é um load balancer e nem funciona como tal, mas chegaremos lá)
rodando na porta 8080, onde recebe uma requisição e a repassa para o backend, que está rodando na porta 8081. O backend confirma que recebeu a requisição e retorna um HTML para o socket, devolvendo a resposta para o load balancer, e o load balancer devolve a resposta para o cliente (é possível ver a resposta no navegador).

Eu tive um problema para conectar o load balancer com o backend, e depois de um tempo analisando o código e pensando no que estava errado, eu desisti de procurar e apelei para a IA. E pasmem, eram apenas dois erros simples: um erro de sintaxe (ainda não estou habituado com o Rust) e o outro era que eu estava usando a variável errada (falta de atenção que me custou pelo menos 15 minutos).

Também tive dificuldade com a sintaxe que usei para ler a resposta do backend, onde 'let bytes_resposta' = match... Esse operador de controle de fluxo é muito bom, mas ainda não me acostumei muito com ele hahaha. Tive que escrever e reescrever esse bloco de código algumas vezes até conseguir fazer funcionar e compreender como ele funciona.

# Dia 2 - Backend

Fiz algumas melhorias no backend:

+ "extrai_caminho_arquivo": É uma função que pega a primeira linha da requisição e a divide em duas partes. A primeira parte é o método, onde extraimos o método usado na requisição (como a requisição está sempre pedindo um arquivo, o método será sempre GET). A segunda parte é o caminho efetivo do arquivo.

+ "backend_cabuloso": Essa função é responsável por manipular o arquivo que foi requisitado, ou seja, a função vai tentar abrir o arquivo. Caso o arquivo seja encontrado, a função vai escrever todo o conteúdo do arquivo de volta para socket, ou seja, vai mandar o conteúdo do arquivo de volta para o cliente. 
> Preciso dar um nome melhor para essa função 

Tenho dois arquivos para testes: Ao rodar 127.0.0.1:8080 (caminho padrão), vai ser retornado o arquivo "index.html". No caminho 127.0.0.1:8080/imagem.jpg, é retornado a logo do patos (que por algum motivo desconhecido aparece cortada no navegador).

Well, acho que por enquanto é isso. Ainda não temos um Load Balancer de fato, é apenas um proxy reverso (por enquanto), vamos chegar lá.