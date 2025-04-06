// Definição da estrutura de Pilha (Stack)
pub struct Stack<T> {
    // A implementação será adicionada após criar os testes
    elementos: Vec<T>,
    capacidade_maxima: Option<usize>,
}

// A implementação será adicionada após criar os testes
impl<T> Stack<T> {
    // A implementação será adicionada após criar os testes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nova_pilha_esta_vazia() {
    //Esta linha define uma função chamada nova_pilha_esta_vazia. A função não recebe parâmetros 
    // e não retorna valores explícitos (ou seja, ela retorna () por padrão em Rust).
        let pilha: Stack<i32> = Stack::new();
        //Aqui, uma nova pilha (stack) é criada e armazenada na variável pilha. A pilha é do tipo Stack<i32>,
        // o que significa que ela armazena valores do tipo i32 (inteiros de 32 bits). 
        // A função Stack::nova() é usada para criar uma nova pilha vazia.
        assert!(pilha.isEmpty());
       // Esta linha usa a macro assert! para verificar se a condição pilha.esta_vazia() é verdadeira. 
       // Se a pilha estiver vazia, a função esta_vazia() deve retornar true, e o programa continuará 
       // a executar sem problemas. Se a pilha não estiver vazia, o programa irá falhar e exibir uma mensagem
       // de erro, indicando que a asserção falhou.
    }

    #[test]
    fn nova_pilha_com_capacidade() {
    //Esta linha define a função que contém o teste. A função não recebe parâmetros e não retorna valores explícitos.
        let pilha: Stack<i32> = Stack::nova_com_capacidade(5);
        //Aqui, uma nova pilha do tipo Stack<i32> é criada usando o método nova_com_capacidade(5). Este método deve criar
        // uma pilha com uma capacidade inicial de 5 elementos. A pilha é armazenada na variável pilha.
        assert!(pilha.esta_vazia());
        //Esta linha verifica se a pilha está vazia logo após sua criação. A macro assert! garante que o programa
        // falhe e exiba uma mensagem de erro se a pilha não estiver vazia.
        assert_eq!(pilha.tamanho(), 0);
        //Aqui, o tamanho da pilha é verificado para garantir que seja igual a 0. A macro assert_eq! compara o
        // tamanho da pilha com o valor 0 e falha se eles forem diferentes.
    }


}
