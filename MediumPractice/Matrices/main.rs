#![allow(unused_parens)]
use std::io;

/*Problem : 
Write a programme which performs addition, subtraction, multiplication of matrices.
The dimensions of both the matrices would be specified by the user (dynamic memory allocation required).
Use of structure or a class to define the matrix would be a good idea.*/

fn main() {
    println!("Your answer is {:?}",input());
}

fn input() -> Vec<Vec<i32>>{
    let mut answer : Vec<Vec<i32>> = Vec::new();
    let mut exit = false;
    while (!exit){
        let mut operation = String::new();
        println!("Do you want to perform addition, subtraction, or multiplication?");
        io::stdin()
            .read_line(&mut operation)
            .expect("Error!");
        let operation = operation.trim();


        if(operation.to_lowercase() == "addition"){
            let matrix1 = matrix_creator();
            let matrix2 = matrix_creator();
            answer = addition(matrix1,matrix2);
            exit = true
        }
        else if(operation.to_lowercase() == "subtraction"){
            let matrix1 = matrix_creator();
            let matrix2 = matrix_creator();
            answer = subtraction(matrix1,matrix2);
            exit = true
        }
        else if(operation.to_lowercase() == "multiplication"){
            let matrix1 = matrix_creator();
            let matrix2 = matrix_creator();
            answer = multiplication(matrix1,matrix2);
            exit = true
        }
        else{
            println!("Please input a proper input. Either addition, subtraction, or multiplication.");
        }
    }

    return answer
}

fn addition(matrix1 : Vec<Vec<i32>>, matrix2 : Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut sum : Vec<Vec<i32>> = Vec::new();
    for i in (0..matrix1.len()){
        sum.push(Vec::new());
        for f in (0..matrix1[0].len()){
            sum[i].push(matrix1[i][f]+matrix2[i][f]);
        }
    }
    return sum
}

fn subtraction(matrix1 : Vec<Vec<i32>>, matrix2 : Vec<Vec<i32>>) -> Vec<Vec<i32>>{
    let mut difference : Vec<Vec<i32>> = Vec::new();
    for i in (0..matrix1.len()){
        difference.push(Vec::new());
        for f in (0..matrix1[0].len()){
            difference[i].push(matrix1[i][f]-matrix2[i][f]);
        }
    }
    return difference
}

//note: I think this works? check it out to make sure though
fn multiplication(matrix1 : Vec<Vec<i32>>, matrix2 : Vec<Vec<i32>>) -> Vec<Vec<i32>>{ //dot product of matrices
    let mut product : Vec<Vec<i32>> = Vec::new();
    for i in (0..matrix1.len()){
        product.push(Vec::new());
        for f in (0..matrix2[0].len()){
            let mut sum = 0;
            for g in (0..matrix2.len()){
                sum += matrix1[i][g]*matrix2[g][f];
            }
            product[i].push(sum.clone());
        }
    }
    return product
}

fn matrix_creator() -> Vec<Vec<i32>>{ //creates a matrix given the user's specifications
    let mut rows = String::new();
    let mut columns = String::new();

    println!("How many rows?");
    io::stdin()
        .read_line(&mut rows)
        .expect("Error!");
    println!("How many columns?");
    io::stdin()
        .read_line(&mut columns)
        .expect("Error!");

    let rows : i32 = rows.trim().parse().unwrap();
    let columns : i32 = columns.trim().parse().unwrap();

    let mut matrix : Vec<Vec<i32>> = Vec::new();

    //creates the empty 2D Vector which will hold the rows
    for i in (0..rows){
        matrix.push(Vec::new());
        for f in (0..columns){
            //takes the user's input
            let mut num = String::new();
            println!("What is the number you want for row {} column {}?",i,f);
            io::stdin()
                .read_line(&mut num)
                .expect("Error!");
            let num = num.trim().parse::<i32>().unwrap();

            matrix[i as usize].push(num.clone()); //adds the user's input
        }
    }

    return matrix
}