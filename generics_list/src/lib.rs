
#[derive(Clone, Debug)]
pub struct List<T> {
    pub head: Option<Node<T>>,
}

#[derive(Clone, Debug)]
pub struct Node<T> {
    pub value: T,
    pub next: Option<Box<Node<T>>>,
}

impl<T :Clone+ std::fmt::Debug> List<T> {
    pub fn new() -> List<T> {
        return List {
            head: None,
        };
    }

    pub fn push(&mut self, value: T) {
        let  next:  Option<Box<Node<T>>>;
        match &self.head{
            Some(node)=>next=Some(Box::new(Node{value: node.value.clone(),  next: node.next.clone()})),
            None=>next=None,
        }
        let node= Node {
            value: value.clone(), 
            next,
        };
        self.head=Some(*Box::new(node));
    }

    pub fn pop(&mut self) {
   
    let  next:  Option<Node<T>>;
      match &self.head {
        Some(list) => {  
            match &list.next{
                Some(node)=>next=Some(Node{value: node.value.clone(),  next: node.next.clone()}),
                None=>next=None,
            }
            
        },
        None => next= None,
       }
      self.head= next;
    }

    pub fn len(&self) -> usize {
        let mut counter: usize=0;

        let mut current= self.head.clone();
        while !current.is_none() {
            match current {
                Some(list)=>{
                    match list.next {
                        Some(node)=> current=  Some(*node),
                        None=> current=None,
                    }
                },
                None=>current=None,
                
            }
            counter+=1;
        }
        return counter;
    }
}