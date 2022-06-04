#![allow(unused_parens)]
use std::io;

/*Create a command-line todo list. Users should be able to add, complete and delete items. 
Bonus: use a database (eg SQLite) to persist todo items between programme runs.
*/

/*Plan: 
1. Create a prototype todo list that doesn't persist
    a. Add items to the todo list
    b. Mark items as complete
    c. Delete finished items
    d. (Optional) Make categories for todo lists and make sub-lists
    
2. Figure out how to make the items persist between program runs using a database
    a. store the items into a database
*/


fn main() {
    println!("Todo :");
    let mut items : Vec<String> = Vec::new();

    loop{
        //get the user's input for what they want to do with the todo list
        let mut input = String::new();
        println!("Action? add,complete,delete,done");
        io::stdin()
            .read_line(&mut input)
            .expect("Error");
        let input = input.trim();

        //based on the user's input for the action, do the following action
        match input{
            "add" => items.push(format!("{}{}",(add().trim())," (Status: incomplete)")),
            "complete" => complete(&mut items),
            "delete" => items = delete(&mut items),
            "list" => list(&mut items),
            "done" => break,
            &_ => println!("Incorrect input, try again.")
        };

        list(&mut items);
    }
}

fn add() -> String{

    let mut item = String::new();
    println!("What do you want to add?");

    io::stdin()
        .read_line(&mut item)
        .expect("Error");

    item
}

fn complete(items : &mut Vec<String>){
    
    list(items);

    //gets the input of the item we want to mark as complete
    let mut item_num = String::new();
    println!("What do you want to mark as complete? Input the number of the item that you want to complete");
    io::stdin()
        .read_line(&mut item_num)
        .expect("Error");
    let item_num = item_num.trim().parse::<usize>().unwrap();

    //marks item as complete
    items[item_num-1] = format!("{}{}",&items[item_num-1][0..items[item_num-1].len()-11],"complete)");
}

fn delete(items : &mut Vec<String>) -> Vec<String>{

    list(items);

    //gets the input of the item we want to mark as complete
    let mut item_num = String::new();
    println!("What do you want to delete? Input the number of the item that you want to delete");
    io::stdin()
        .read_line(&mut item_num)
        .expect("Error");
    let item_num = item_num.trim().parse::<usize>().unwrap();
    
    //adds all the items to the new list except the deleted item
    let mut new_item_list : Vec<String> = Vec::new();
    for e in (0..items.len()){
        if(e != item_num-1){
            new_item_list.push(items[e].clone());
        }
    }
    new_item_list
}

fn list(items : &mut Vec<String>){
    //prints the todo list
    for i in (1..items.len()+1){
        print!("{}. ",i);
        println!("{}",items[i-1]);
    }
}