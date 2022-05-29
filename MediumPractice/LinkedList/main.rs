#![allow(unused_parens)]

use std::io;
use std::rc::Rc;

/*Task:
Create a sophisticated linked list class. You should be able to insert and delete nodes anywhere in the list, 
and the nodes should have pointers to nodes both in front and behind them.*/

/*Breaking it down:
Linked list: structure with fields a field for value, the node before it, and a pointer to the next object in the list
insert and delete nodes: these will be methods implementing the linked list struct*/

static mut INDEX : i32 = 0;

fn main() {
    let mut choice = String::new();
    
    let mut list : LinkedList = LinkedList{
        head : None,
        length : 0
    };

    println!("okay first we need to make an initial linked list.");

    //creates the initial linked list
    loop{
        let mut resume = String::new();
        let mut value = String::new();
        println!("Do you want to keep going? type yes or no in lowercase and then press enter");
        io::stdin()
            .read_line(&mut resume)
            .expect("Error");
        let resume = resume.trim();

        if(resume.eq("yes")){
            println!("Please type the value that you want to append to the initial linked list:");
            io::stdin()
                .read_line(&mut value)
                .expect("Error!");
            let value : i32 = value.trim().parse::<i32>().unwrap();
            list = list.add(value);
        }
        else{
            println!("Here is your initial Linked List:");
            list.head.unwrap().to_string();
            break
        }
    }
}


//Create the linked list and node struct
#[derive(Clone,Debug)]
struct Node{
    before : Option<Box<Node>>, //use Box here so we have a set size for the reference which satisfies the issue with the recursive struct
    value : Option<i32>, 
    after : Option<Box<Node>>, 
}

#[derive(Clone,Debug)]
struct LinkedList{
    head : Option<Node>,
    length : i32
}

//Create the necessary methods
impl LinkedList{
    fn add(mut self , num : i32) -> LinkedList{
        if(self.length == 0){ //adds the head of the linked list if the linked list is empty
            let new_node = Node{
                before : None,
                value : Some(num),
                after : None
            };
            self.head = Some(new_node);
        }
        else{ //adds the node to the end of the linked list
            let mut last = self.clone().head.unwrap().last(); //debug: clone
            let new_node = Node{
                //Note: look into take() some more. got it from this link https://tinyurl.com/2kl6rj6r
                //traverses through the linked list to find the last node
                before : Some(Box::new(last.clone())),//debug: clone
                value : Some(num),
                after : None
            };
            println!("{:?}",new_node);
            //debug: I think the line below is what's causing all of the trouble. For some reason it ain't altering what I need it to alter
            last.after = Some(Box::new(new_node)); //changes the node before the new node so it points to the new node
        }
        self.length += 1;
        self
    }
}

impl Node{
    fn last(self) -> Node{
        match self.after{
            //debug: need to dereference argument
            None => self,
            _ => Node::last(*self.after.unwrap())
        }
    }

    //traverses through the nodes in the linked list and prints them in order of index
    fn to_string(&self) -> &Node{
        if(self.after.is_none()){
            println!("{:?}",&self);
            return self
        }
        else{
            print!("{:?},",self.value.unwrap());
            return Node::to_string(self.after.as_ref().unwrap()) //if the node has something after it then it then we move onto the next node
        }
    }
}