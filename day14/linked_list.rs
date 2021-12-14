//use std::rc::Rc;

#[derive(Clone)]
enum Address {
    Address(Box<Node>),
    Nil,
}

#[derive(Clone)]
struct Node {
    value : char,
    next : Address
}

impl Node {
    fn insert(&mut self, value: char) {
        match self.next {
            Address::Address(ref mut next_address) => {
                let node = Node {
                    value: value, 
                    next: Address::Address(next_address.clone())
                };
                self.next = Address::Address(Box::new(node))
            }
            Address::Nil => {
                let node = Node {
                    value: value, 
                    next: Address::Nil,
                };
                self.next = Address::Address(Box::new(node))
            }
        }
    }

    fn append(&mut self, value: char) {
        match self.next {
            Address::Address(ref mut next_address) => {
                next_address.append(value);
            }
            Address::Nil => {
                self.insert(value);
            }
        }
    }

}

//impl Iterator for Node {
//    type Item = Box<Node>;
//
//    fn next(&mut self) -> Option<Self::Item> {
//        match self.next {
//            Address::Address(ref mut next_address) => {
//                Some(*next_address)
//            }
//            Address::Nil => {
//                None
//            }
//        }
//    }
//}


                
                

fn main() {
    
    let mut list = Node { value: '0', next: Address::Nil };
    ['A','B','C','D'].iter().for_each(|l| list.append(*l));
    
    let mut node = &mut list;
    loop {
        print!("{}", node.value);
        match node.next {
            Address::Address(ref mut next) => node = next,
            Address::Nil => break,
        }
    }

    loop {
        match node.next {
            Address::Address(ref mut next) => {
                node.insert('1');
                node = next
            },
            Address::Nil => break,
        }
    }

}
