#![allow(unused_parens)]
use std::io;
use rusqlite::{params,Connection, Result};

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


fn main() -> Result<()>{
    //opens a connection to the database that will hold the todo list
    let conn = Connection::open("practice.db")?;

    //creates the todo table if it doesn't exist already
    conn.execute(
        "create table if not exists ToDo(
            task text primary key,
            status text not null
        )",
        []
    )?;

    //Loop that gets user inputs and acts accordingly
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
            "add" => add(&conn)?,
            "complete" => complete(&conn)?,
            "delete" => delete(&conn)?,
            "done" => break,
            &_ => error()?
        };

        print_list(&conn)?;
    }

    println!("Okay here's your final todo list:");
    print_list(&conn)?;

    //test(&conn);
    Ok(())
}

fn add(connection : &Connection) -> Result<()>{
    //gets the task that the user wants to add to the todo list
    let mut item = String::new();
    println!("What do you want to add?");
    io::stdin()
        .read_line(&mut item)
        .expect("Error");
    let item = item.trim();

    //adds the task to the database
    //debug: doesn't work if it already exists
    connection.execute(
        "INSERT INTO ToDo (Task,Status) VALUES (?1,?2)",
        params![item,"Incomplete"],
    )?;

    Ok(())
}

fn complete(connection : &Connection) -> Result<()>{
    //gets the input of the item we want to mark as complete
    let mut item_num = String::new();
    println!("What do you want to mark as complete? Input the number of the item that you want to complete");
    io::stdin()
        .read_line(&mut item_num)
        .expect("Error");
    let item_num = item_num.trim().parse::<usize>().unwrap();

    //updates the according row so that that the task is shown as completed
    connection.execute(
        //"UPDATE (SELECT ROW_NUMBER() over () from ToDO) SET Status = 'Complete'",
        "UPDATE ToDo SET Status = 'Complete' WHERE _rowid_ = ?1", //debug: need to figure out the rownum crap here too
        params![item_num]
    )?;

    Ok(())
}

fn delete(connection : &Connection) -> Result<()>{
    print_list(connection);

    //gets the input of the item we want to mark as complete
    let mut item_num = String::new();
    println!("What do you want to delete? Input the number of the item that you want to delete");
    io::stdin()
        .read_line(&mut item_num)
        .expect("Error");
    let item_num = item_num.trim().parse::<usize>().unwrap();

    //deletes the according row from the table
    connection.execute(
        "DELETE FROM ToDo WHERE _rowid_ = ?1",
        params![item_num]
    )?;

    Ok(())
}

//Note: Come back to this later because I don't understand how it works
//debug: so basically the tasks and statuses have different rowid's so I gotta use the rownum instead
fn print_list(conn: &Connection) -> Result<Vec<String>> {
    let tasks = get_tasks(conn).unwrap();
    let statuses = get_status(conn).unwrap();

    for i in (0..tasks.len()){
        println!("{}. {:?} | {:?}",i+1,tasks.get(i).unwrap(),statuses.get(i).unwrap());
    }

    Ok(tasks)
}

//gets the tasks from the todo list to print
fn get_tasks(conn: &Connection) -> Result<Vec<String>>{
    //prepares the statement that will be executed
    //debug:for some reason this keeps organizing the tasks in alphabetical order?
    //solution: so for some reason doing "Select Task from ToDo" kept ordering the rows implicitly? not sure why though
    let mut stmt = conn.prepare("SELECT * FROM ToDo")?;

    //executes the statement and applies the row.get(0) function on every row so we get all of the items
    let items = stmt.query_map([], |row| row.get(0))?;

    //puts all of the items into a vector
    let mut tasks = Vec::new();
    for task in items {
        tasks.push(task?);
    }

    Ok(tasks)
}

//gets the statuses of the items to print
fn get_status(conn : &Connection) -> Result<Vec<String>>{
    let mut stmt2 = conn.prepare("SELECT Status FROM ToDo")?;
    let completeness = stmt2.query_map([], |row| row.get(0))?;
    let mut statuses = Vec::new();
    for status in completeness{
        statuses.push(status?);
    }

    Ok(statuses)
}

fn error() -> Result<()>{
    println!("Incorrect input, try again.");
    Ok(())
}

fn test(connection : &Connection) ->Result<()>{
    connection.execute(
        "SELECT * from ToDo",
        []
    )?;

    Ok(())
}