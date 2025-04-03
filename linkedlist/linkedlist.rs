use std::fmt::Display;

// Option<T> puede tener Some(valor)` o None

struct Node<T> {
    value: T,
    // puntero al siguiente nodo
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>
}

impl<T: Display> LinkedList<T> {
    
    // constructor 
    fn new() -> Self {
        LinkedList { 
            head: None 
        }
    }
    
    // nuevo nodo al principio
    fn push(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            // take() extrae el head actual y deja None
            next: self.head.take(), 
        });
        self.head = Some(new_node);
    }
    
    // eliminar el nodo del principio
    fn pop(&mut self) -> Option<T> {
        let temp_head = self.head.take();

        if let Some(node) = temp_head {
            self.head = node.next;
            Some(node.value)
        } else { 
            None
        }
    }
    
    fn print(&self) {
        // referencia al valor del head
        let mut actual = self.head.as_ref(); 
        while let Some(node) = actual {
            print!("{} -> ", node.value);
            actual = node.next.as_ref();
        }
        println!("None");
    }
}

fn main() {
    let mut list = LinkedList::new();
    list.push(10);
    list.push(20);
    list.push(30);
    
    list.print();
    
    list.pop();
    list.print();
}