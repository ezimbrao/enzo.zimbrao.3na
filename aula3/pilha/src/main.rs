use pilha::Stack;

fn main() {
    println!("=== Demonstração de Pilha com Limite de Capacidade ===");
    // Criando uma pilha com capacidade de 3 elementos
    let mut pilha_limitada = Stack::<i32>::nova_com_capacidade(3);

    // Empilhando elementos
    println!("Empilhando elementos: 10, 20, 30");
    pilha_limitada.empilhar(10).unwrap();
    pilha_limitada.empilhar(20).unwrap();
    pilha_limitada.empilhar(30).unwrap();

    // Tentativa de empilhar além da capacidade (deve falhar)
    println!("Tentando empilhar 40 além da capacidade:");
    match pilha_limitada.empilhar(40) {
        Ok(_) => println!("Elemento 40 empilhado com sucesso"),
        Err(e) => println!("{}", e),
    }

    // Verificando o elemento do topo
    if let Some(valor) = pilha_limitada.topo() {
        println!("Elemento no topo: {}", valor);
    }

    // Desempilhando e exibindo os elementos
    println!("Desempilhando elementos:");
    while let Some(valor) = pilha_limitada.desempilhar() {
        println!("Elemento desempilhado: {}", valor);
    }

    // Verificando se a pilha está vazia
    println!("A pilha está vazia? {}", pilha_limitada.esta_vazia());

    println!("\n=== Demonstração de Pilha sem Limite de Capacidade ===");
    // Demonstrando uma pilha sem limite de capacidade
    let mut pilha_ilimitada = Stack::<String>::nova();

    // Empilhando strings
    println!("Empilhando palavras: 'Rust', 'é', 'incrível!'");
    pilha_ilimitada.empilhar(String::from("Rust")).unwrap();
    pilha_ilimitada.empilhar(String::from("é")).unwrap();
    pilha_ilimitada.empilhar(String::from("incrível!")).unwrap();

    // Exibindo tamanho da pilha
    println!("Tamanho da pilha: {}", pilha_ilimitada.tamanho());

    // Desempilhando e exibindo as strings
    println!("Conteúdo da pilha (de cima para baixo):");
    while let Some(texto) = pilha_ilimitada.desempilhar() {
        println!("{}", texto);
    }
}