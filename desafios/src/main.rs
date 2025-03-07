use std::boxed::Box;
use std::ptr;

pub struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: *mut Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Queue<T> {
    // Cria uma nova fila vazia
    pub fn new() -> Self {
        Queue {
            head: None,
            tail: ptr::null_mut(), // Inicializa como ponteiro nulo
            len: 0,
        }
    }

    // Adiciona um elemento no final da fila
    pub fn enqueue(&mut self, value: T) {
        let new_node = Box::new(Node {
            value,
            next: None,
        });

        // Se a fila estiver vazia, o novo nó é a cabeça e a cauda
        if self.len == 0 {
            self.head = Some(new_node);
            self.tail = unsafe { &mut self.head.as_mut().unwrap().next as *mut _ }; // Ponteiro para o próximo nó
        } else {
            // Se a fila não estiver vazia, atualizamos a cauda
            unsafe {
                (*self.tail).replace(new_node);
            }
            self.tail = unsafe { &mut (*self.tail).as_mut().unwrap().next as *mut _ };
        }
        self.len += 1;
    }

    // Remove e retorna o primeiro elemento da fila
    pub fn dequeue(&mut self) -> Option<T> {
        if let Some(head) = self.head.take() {
            self.head = head.next;
            if self.head.is_none() {
                self.tail = ptr::null_mut(); // Se a fila ficar vazia, a cauda deve ser nula
            }
            self.len -= 1;
            Some(head.value)
        } else {
            None
        }
    }

    // Retorna uma referência ao primeiro elemento da fila
    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| &node.value)
    }

    // Retorna o número de elementos na fila
    pub fn len(&self) -> usize {
        self.len
    }

    // Verifica se a fila está vazia
    pub fn is_empty(&self) -> bool {
        self.len == 0
    }
}

impl<T> Drop for Queue<T> {
    fn drop(&mut self) {
        while let Some(_) = self.dequeue() {} // Desaloca a memória da fila
    }
}

fn main() {
    let mut fila = Queue::new();

    // Simula a chegada de clientes na fila
    fila.enqueue("Cliente 1");
    fila.enqueue("Cliente 2");
    fila.enqueue("Cliente 3");

    println!("Fila inicial:");
    println!("Primeiro da fila: {:?}", fila.peek());

    // Simula o atendimento dos clientes na ordem de chegada
    while !fila.is_empty() {
        if let Some(cliente) = fila.dequeue() {
            println!("Atendendo: {}", cliente);
        }
    }

    println!("Todos os clientes foram atendidos.");
}
