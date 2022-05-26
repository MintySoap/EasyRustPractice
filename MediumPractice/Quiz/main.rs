#![allow(unused_parens)]
use std::fs;
use std::io::prelude::*;
use std::io::BufReader;
use std::io;

/*Problem :
Write a quiz which retrieves a question and answer from a file. 
Allow the user to take the quiz, count points total and show score.
*/


/*Steps:
4. Calculate the score and output it to the user
*/
fn main() {
    quiz();
}

fn quiz(){
    //1. Get file path for both the questions and the answer files
    //opening the files
    let questions = fs::File::open("Questions.txt").unwrap();
    let answer_sheet = fs::File::open("Answers.txt").unwrap();
    //converting the files to BufReader to make it easier to read and work with
    let questions = BufReader::new(questions);
    let answer_sheet = BufReader::new(answer_sheet);
    //holds all of the user's answers
    let mut answers : Vec<String> = Vec::new();

    //2. Print each question and have the user answer them
    for line in questions.lines(){
        let current_line = line.unwrap();
        if (current_line.len() != 0){
            println!("{}", current_line);
        }
        else{ //note:has to have an empty line at the end to work
            let mut user_answer = String::new();
            println!("What is the answer? Either input the letter or the answer itself, but not both");
            io::stdin()
                .read_line(&mut user_answer)
                .expect("ERROR: something went wrong when processing your answer");
            let user_answer = user_answer.trim();
            answers.push(user_answer.clone().to_string());//clone's the data here and turns it to a String so we can then send it to the Vector without ownership issues
        }
    }

    let mut question_index = 0;
    let mut correct = 0;

    //3. Evaluate the user's answers and compare them to the correct answers
    for answer in answer_sheet.lines(){
        let current_answer = answer.unwrap();
        if (current_answer.len() != 0){
            if(answers[question_index].to_uppercase() == &current_answer[4..5] || answers[question_index] == &current_answer[7..]){
                correct += 1;
            }
            question_index += 1;
        }
    }

    println!("You got {} questions correct out of {} questions, giving you a score of %{}.", &correct, &question_index, ((correct as f64/question_index as f64)*100.0));
}