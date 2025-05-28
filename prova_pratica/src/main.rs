//nivel1 

use std::collections::LinkedList;


//lista encadeada
fn main(){
    let mut  lista: LinkedList<i32> = LinkedList::new();
    
    //adiciona e printa os elementos na lista
    lista.push_front(10);//adiciona o elemento 10 no início da lista
    //printa o primeiro elemento da lista
    println!("Primeiro elemento da lista: {:?}", lista.front());
    
    lista.push_back(20);//adiciona o elemento 20 no final da lista
    //printa o último elemento da lista
    println!("Último elemento da lista: {:?}", lista.back());

    //printa a lista atual
    println!("Lista atual: {:?}", lista);

    //remove o primeiro elemento da lista
    lista.pop_front();
    
    //lista atualizada depois da remoção
    println!("Lista atualizada: {:?}", lista);

}











//nivel2

//Implemente manualmente uma LinkedList do zero utilizando struct e Box para alocação dinâmica.

//estrutura do nó da lista encadeada
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

//estrutura da lista encadeada
struct LinkedList<T> { // contém o nó cabeça da lista
    head: Option<Box<Node<T>>>,
}

//implementação da lista encadeada
impl<T> LinkedList<T> { //

    // cria uma nova lista encadeada
    fn new() -> Self {
        LinkedList { head: None}
    }
    // insere o elemento 10 no início da lista
    fn push_front(&mut self, value: T) {// cria um novo nó e o adiciona ao início da lista
        let new_node: Box<Node<T>> = Box::new(Node { 
            value: value,
            next: self.head.take(),
        });
        self.head = Some(new_node);
       
    }
    // insere um elemento no final da lista
    fn push_back(&mut self, value: T) { // cria um novo nó e o adiciona ao final da lista
        let new_node: Box<Node<T>> = Box :: new(Node{
            value: value,
            next: None,
        });
    }

    //remove o elemento do início da lista
    fn pop_front(&mut self) -> Option<T> { // remove o nó do início da lista e retorna seu valor
        if let Some(mut node) = self.head.take() { 
            self.head = node.next.take(); 
            return Some(node.value); 
        }
        None
    }

    //exibir todos os elementos da lista
    fn display(&self) // percorre a lista e imprime o valor de cada no
    where
        T: std::fmt::Display,
    {
        let mut current = &self.head;
        while let Some(node) = current {
            println!("{}", node.value);
            current = &node.next;
        }
    }

  
} impl LinkedList<i32> {
    fn main() {
        let mut list = LinkedList::new();
        list.push_front(10);
        list.push_front(20);
        list.push_back(30);
        list.display();
        println!("Removed: {:?}", list.pop_front());
        list.display();
    }
} fn main() {
    let mut list = LinkedList::new();
    list.push_front(10);
    list.push_front(20);
    list.push_back(30);
    list.display();
    println!("Removed: {:?}", list.pop_front());
    list.display();
}



