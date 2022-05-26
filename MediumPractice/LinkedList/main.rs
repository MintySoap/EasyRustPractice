#![allow(unused_parens)]

use std::io;

/*Task:
Create a sophisticated linked list class. You should be able to insert and delete nodes anywhere in the list, 
and the nodes should have pointers to nodes both in front and behind them.*/

/*Breaking it down:
Linked list: structure with fields a field for value, the node before it, and a pointer to the next object in the list
insert and delete nodes: these will be methods implementing the linked list struct*/

//debug: so for some weird reason, this just isn't working and I have no idea why. I think it's because I keep cloning the nodes as a workaround

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
            let value = value.trim().parse::<i32>().unwrap();
            list.add(value);
        }
        else{
            break
        }
    }

    println!("Here is your initial Linked List:");
    list.to_string(list.head.as_ref().unwrap());

    println!("Do you want to insert or delete a node? type your answer in lowercase");
    io::stdin()
        .read_line(&mut choice)
        .expect("Error!");

    unsafe{ //we need to do this to tell rust that we understand that what we're trying to do is unsafe
        if (choice.trim() == "insert"){
            list.insert();
        }
        if(choice.trim() == "delete"){
            list.delete();
        }
    }
}


//Create the linked list and node struct
#[derive(Clone)]
#[derive(Debug)]
struct Node{
    before : Option<Box<Node>>, //use Box here so we have a set size for the reference which satisfies the issue with the recursive struct
    value : Option<i32>, 
    after : Option<Box<Node>>, 
}

struct LinkedList{
    head : Option<Node>,
    length : i32
}

//Create the necessary methods
impl LinkedList{
    //appends a node to the end of the linked list
    fn add(&mut self, num : i32){
        if(self.length == 0){ //checks if the linked list is empty
            let new_node = Node{
                before : None,
                value : Some(num),
                after : None
            };
            self.head = Some(new_node);
        }
        else{ 
            let new_node = Node{
                before : Some(Box::new(LinkedList::last(&self.head.as_ref().unwrap()))),
                value : Some(num),
                after : None
            };
            //gets the pointer of the previous last node and points it to this one
            LinkedList::last(&self.head.as_ref().unwrap()).after = Some(Box::new(new_node)); //debug: okay this is the problem line. I need to find some way to alter the head. that's the key.
        }
        self.length += 1;
    }

    //returns the last node in the linked list
    fn last(node : &Node) -> Node{
        if(node.after.is_none()){ //if the node doesn't have anything after it then it is the last node
            return node.clone()
        }
        else{
            return LinkedList::last(&*node.after.as_ref().unwrap()) //if the node has something after it then it then we move onto the next node
        }
    }

    //prints the linked list in sequential order based on index
    fn to_string(&self, node : &Node) -> Node{
        if(node.after.is_none()){ //if the node doesn't have anything after it then it is the last node
            println!("{:?}",node);
            return node.clone()
        }
        else{
            print!("{:?},",node.value.unwrap());
            return LinkedList::to_string(&self, node.after.as_ref().unwrap()) //if the node has something after it then it then we move onto the next node
        }
    }

    unsafe fn insert(&mut self){
        let mut index = String::new();
        let mut num = String::new();

        //gets the index of the new node
        println!("Tell me the index you want to insert the node in. If it's an invalid index, it'll be added to the end.");
        io::stdin()
            .read_line(&mut index)
            .expect("Error!");
        let index = index.trim().parse::<i32>().unwrap();

        //gets the value of the new node
        println!("What number do you want the node to have as a value?");
        io::stdin()
            .read_line(&mut num)
            .expect("Error!");
        let num = num.trim().parse::<i32>().unwrap();

        if(index >= self.length-1 || index < 0){ //adds the new node to the end of the linked list
            LinkedList::add(self, num);
        }
        else if(index == 0){//adds the new new node to the beginning of the linked list
            let new_node : Node = Node{
                before : None,
                value : Some(num), 
                after : Some(Box::new(LinkedList::traverse(self.head.clone().unwrap(),1))) 
            };
            INDEX = 0;
            self.head.clone().unwrap().before = Some(Box::new(new_node.clone()));
            self.head = Some(new_node.clone());
        }
        else{ //inserts the new node at the index, adjusting the surrounding nodes appropriately
            let mut new_node : Node = Node{
                before : Some(Box::new(LinkedList::traverse(self.head.clone().unwrap(),index-1))),
                value : Some(num), 
                after : None 
            };
            INDEX = 0;
            new_node.after = Some(Box::new(LinkedList::traverse(self.head.clone().unwrap(),index+1)));
            INDEX = 0;
            LinkedList::traverse(self.head.clone().unwrap(),index+1).before = Some(Box::new(new_node.clone()));
            INDEX = 0;
            LinkedList::traverse(self.head.clone().unwrap(),index-1).after = Some(Box::new(new_node.clone()));
            INDEX = 0;
        }
        self.length += 1;
    }

    unsafe fn traverse(node : Node,num : i32) -> Node{
        if(INDEX == num){
            INDEX = 0;
            return node
        }
        else{
            INDEX += 1;
            println!("{}", node.value.unwrap()); //note: this is a debug statement
            return LinkedList::traverse(*node.after.unwrap(), num)
        }
    }

    unsafe fn delete(&mut self){
        let mut invalid = true;
        let mut index = 0;

        //gets index of element we wanna delete and checks if the index is correct
        while(invalid){
            let mut input = String::new();

            println!("Tell me the index of the node you want to delete");
            io::stdin()
                .read_line(&mut input)
                .expect("Error!");
            index = input.trim().parse::<i32>().unwrap();

            if(index < 0 || index > self.length-1){
                println!("Invalid index! Try again.")
            }
            else{
                invalid = false;
            }
        }

        if(index == 0){ //deletes first index
            LinkedList::traverse(self.head.clone().unwrap(), 1).before = None;
            self.head = Some(LinkedList::traverse(self.head.clone().unwrap(), 1));
            self.length -= 1;
        }
        else if(index == self.length-1){ //deletes last index
            LinkedList::traverse(self.head.clone().unwrap(), self.length-1).before = None;
            LinkedList::traverse(self.head.clone().unwrap(), self.length-2).after = None;
            self.length -= 1;
        }
        else{ //deletes specified index
            LinkedList::traverse(self.head.clone().unwrap(), index+1).before = Some(Box::new(LinkedList::traverse(self.head.clone().unwrap(), index-1)));
            LinkedList::traverse(self.head.clone().unwrap(), index-1).after = Some(Box::new(LinkedList::traverse(self.head.clone().unwrap(), index+1)));
            self.length -= 1;
        }
    }
}