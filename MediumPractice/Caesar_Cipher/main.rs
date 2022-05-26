use std::io;
use std::fs;
use std::io::BufRead;
use std::char;
use std::fs::OpenOptions;
use std::io::Write;
/*Problem : Simple file encryption using ROT13. Optional: allow a choice of encryption eg Caeser cipher.*/

/*
Goal : I'm gonna take it one step further. I'm gonna ask them for the direction for the rotation as well as the number of rotations.
*/

fn main() {
    inputs();
}

fn inputs(){
    let mut direction = String::new();
    let mut number = String::new();
    let mut filename = String::new();

    //Step 1: Ask the user what direction they want to rotate
    println!("Do you want to encrypt the message by shifting to the left or to the right?");
    io::stdin()
        .read_line(&mut direction)
        .expect("Error!");


    //Step 2: Ask the user how many rotations they want the encryption to be
    println!("How many rotations do you want?");
    io::stdin()
        .read_line(&mut number)
        .expect("Error!");

    let number = number.trim().parse::<i32>().unwrap();
    let direction = direction.trim();


    /*
    Step 3: Ask the user for the file path of the text file they want to encrypt
    Step 3B: Find a more user friendly way for the user to select a text file to encrypt. Maybe like a file explorer?
    */
    println!("Input the file path of the TEXT file that you want to encrypt:");
    io::stdin()
        .read_line(&mut filename)
        .expect("Error!");

    let file = fs::File::open(filename.trim()).unwrap();
    encryption(file,direction,number);

    println!("Your file has been encrypted! You're free to communicate agent!");
}

fn encryption(file : std::fs::File , direction : &str, num : i32){ //Step 4: Encrypt their input.
    let _encrypted_file = std::fs::File::create("encrypted.txt").expect("create failed"); //creates a new file that we will encrypt the data into
    let mut encrypted_file = OpenOptions::new().append(true).open("encrypted.txt").unwrap(); //opens the file to write in
    let reader = io::BufReader::new(file);
    let lines = reader.lines();
    for i in lines{ //cycles through the lines of the unencrypted file
        for f in i.unwrap().chars(){ //cycles through the characters in the line
            if direction.to_lowercase() == "right"{ //rotate to the right
                if char::is_lowercase(f){
                    let encrypted_char = char::from_u32((f as u32 + (num as u32%26) - 97)%26 + 97).unwrap().to_string();
                    write!(encrypted_file, "{}",encrypted_char);//this should write the character rotated to the right n times to the newly created encrypted file
                }
                else if char::is_uppercase(f){
                    let encrypted_char = char::from_u32((f as u32 + (num as u32%26) - 65)%26 + 65).unwrap().to_string();
                    write!(encrypted_file, "{}",encrypted_char);
                }
                else{
                    write!(encrypted_file, "{}",f.to_string());
                }
            }
            else{ //rotate to the left
                if char::is_lowercase(f){
                    let encrypted_char = char::from_u32((f as u32 + 26 - (num as u32%26) - 97)%26 + 97).unwrap().to_string();
                    write!(encrypted_file, "{}",encrypted_char);//this should write the character rotated to the right n times to the newly created encrypted file
                }
                else if char::is_uppercase(f){
                    let encrypted_char = char::from_u32((f as u32 + 26 - (num as u32%26) - 65)%26 + 65).unwrap().to_string();
                    write!(encrypted_file, "{}",encrypted_char);
                }
                else{
                    write!(encrypted_file, "{}",f.to_string());
                }
            }
        }
        write!(encrypted_file, "\n");
    }
}
