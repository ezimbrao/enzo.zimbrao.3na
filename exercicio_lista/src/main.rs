/*use std::collections::LinkedList;

fn main() {
    // Crie uma lista ligada vazia
    let mut frases = LinkedList::new();

    // Adicione 10 frases motivacionais à lista
    frases.push_back("Nem pensa, só vai.");
    frases.push_back("A vida não é um problema a ser resolvido, mas uma realidade a ser vivida.");    
    frases.push_back("Grandes coisas são compostas por uma serie de pequenas coisas resumidas.");
    frases.push_back("Técnica e habilidade sozinhas não levam ao topo.");
    frases.push_back("A vida é o que acontece enquanto você está ocupado fazendo outros planos.");
    frases.push_back("Só sei que nada sei.");
    frases.push_back("Seja a mudança que você quer ver no mundo.");
    frases.push_back("Os proletários nada tem a perder a não ser seus grilhões.");
    frases.push_back("O homem não é nada além daquilo que a educação faz dele.");
    frases.push_back("Uma vida não examinada não merece ser vivida.");
    
    
    
    // Acesse e imprima o terceiro elemento
    let terceiro_elemento = frases.iter().nth(2).unwrap();
        println!("Terceiro elemento: {}", terceiro_elemento);
    
    
    
    // Imprima o tamanho total da lista
    println!("Tamanho total da lista: {}", frases.len());
}
*/

//atividade2
// Criar lista ligada sem o std::collections::LinkedList
// Definição de um nó da lista ligada
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
}

impl<T: std::fmt::Display> LinkedList<T> {
    // Cria uma nova lista ligada vazia
    fn new() -> Self {
        LinkedList { head: None }
    }

    // Adiciona um elemento ao final da lista
    fn push_back(&mut self, value: T) {
        let new_node = Box::new(Node { value, next: None });

        let mut current = &mut self.head;
        while let Some(ref mut next) = current {
            current = &mut next.next;
        }
        *current = Some(new_node);
    }

    // Imprime todos os elementos da lista
    fn print(&self) {
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            println!("{}", node.value);
            current = node.next.as_ref();
        }
    }

    // Retorna o tamanho da lista
    fn len(&self) -> usize {
        let mut count = 0;
        let mut current = self.head.as_ref();
        while let Some(node) = current {
            count += 1;
            current = node.next.as_ref();
        }
        count
    }
}

fn main() {
    // Criação de uma lista ligada
    let mut list = LinkedList::new();

    // Adicionando elementos à lista
    list.push_back("Nem pensa, só vai.");
    list.push_back("A vida não é um problema a ser resolvido, mas uma realidade a ser vivida.");
    list.push_back("Grandes coisas são compostas por uma série de pequenas coisas resumidas.");

    // Imprimindo os elementos da lista
    println!("Elementos da lista:");
    list.print();

    // Imprimindo o tamanho da lista
    println!("Tamanho da lista: {}", list.len());
}