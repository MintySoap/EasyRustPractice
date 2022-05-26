#![allow(unused_parens)]

use std::path::Path;
use std::io;
use fs_extra::move_items;
use fs_extra::dir::copy;
use std::fs::File;
use std::io::prelude::*;

/*Task:
Write a programme which will perform the job of moving the file from one location to another.
The source and destination path will be entered by the user.
Perform the required error checking and handle the exceptions accordingly.
*/

/*Solution 1:
1. deal with improper file paths
2. use the fs_extra::move_items function from the fs_extra crate. this does all the logistical work for me
*/

/*Solution 2:
1. deal with improper file paths
2. make a new file in the desired destination with the same name
3. copy the file to another file in the desired directory
4. delete the original file
*/


fn main() {
    println!("Please enter the file path of the file that you want to move.");
    let initial_path = initial_input();

    println!("Please enter the path of the folder that you want to move the file to.");
    let destination_path = destination_input();

    move_file_easy(initial_path.trim(), destination_path.trim())
}

//1. deal with improper file paths
fn initial_input() -> String{
    let mut valid = false;
    let mut path = String::new();
    while (!valid){
        
        io::stdin()
            .read_line(&mut path)
            .expect("Error!");

        let trimmed_path = path.trim();

        if (Path::new(trimmed_path).exists()){
            valid = true;
        }
        else{
            println!("That isn't a valid file path. Please input a correct file path.");
        }
    }

    return path
}

fn destination_input() -> String{
    let mut valid = false;
    let mut path = String::new();
    while (!valid){
        
        io::stdin()
            .read_line(&mut path)
            .expect("Error!");

        let trimmed_path = path.trim();

        if (Path::new(trimmed_path).is_dir()){
            valid = true;
        }
        else{
            println!("That isn't a valid folder path. Please input a correct folder path.");
        }
    }

    return path
}

//Solution 1 step 2:use the fs_extra::move_items function from the fs_extra crate. this does all the logistical work for me

fn move_file_easy(initial : &str, destination : &str){
    let options = fs_extra::dir::CopyOptions::new();

    let mut from_paths = Vec::new();
    from_paths.push(initial);
    move_items(&from_paths, destination, &options);
}

/*Solution 2:
2. make a new file in the desired destination with the same name
3. copy the file to another file in the desired directory
4. delete the original file
*/
fn move_file_basic(initial : &str, destination : &str){
    //2. make a new file in the desired destination with the same name
    let new_path = destination.to_string() + "/" + &name_of_file(initial);
    let mut file = File::create(&new_path);

    //3. copy the file to another file in the desired directory
    std::fs::copy(&initial,new_path);

    //4. delete the original file
    std::fs::remove_file(initial);
}

//gets the name of the file
fn name_of_file(file : &str) -> String{
    let mut name : String = String::new();

    for i in file.chars(){
        if(i == '/'){
            name = String::new();
        }
        else{
            name += &i.to_string();
        }
    }
    
    return name
}