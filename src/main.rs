#![allow(unused)]

use std::time::Duration;
use std::{array, io, string, thread, u32};
use rand::Rng;
use std::io::{Write, BufRead, BufReader, ErrorKind};
use std::fs::File;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::thread; 


// collect and print out input
// fn main() {
//     println!("what is your name...");
//     let mut name: String = String::new();  
//     let greeting: &str = "Nice to meet you brother";
//     io::stdin().read_line(&mut name)
//         .expect("Didn't receive an input");

//     println!("Hello {} {}", name.trim_end(), greeting);
// }


// variables defined with const should be in UPPERCASE

// fn main(){
//     // const MONEY: u32 = 1_000_000;
//     // const SALARY: u32 = 2_000_000;
//     // const PI: f32 = 3.1234;
//     // let age: &str = "47";
//     // let mut age: u32 = age.trim().parse()
//     //     .expect("Age was not defined");


//     // println!("please fill in your Name and age");
//     // let mut name: String = String::new();
//     // let mut age: String = String::new();

    
//     // input that accepts name value
//     // io::stdin().read_line(&mut name)
//     // .expect("name cannot be blank");

//     // input that accepts age
//     // io::stdin().read_line(&mut age)
//     //     .expect("age was not defined");

//     // println!("Your name is {} and you are {} years", name.trim_end(), age.trim_end());
// }

// fn main(){
//        println!("u32 {}", u32::MAX);  
//        println!("U64 {}", u64 ::MAX);  
// }


// maths operations
// fn main(){
//     let mut num_1: u32 = 8;
//     let num_2: u32 = 4;
//     num_1 += 1;

//     println!("8 + 4 = {}", num_1 + num_2);
//     println!("8 - 4 = {}", num_1 - num_2);
//     println!("8 / 4 = {}", num_1 / num_2);
//     println!("8 * 4 = {}", num_1 * num_2);
//     println!("8 % 4 = {}", num_1 % num_2);


// }

// generate random number
// fn main(){
//     let random_num: u32   = rand::thread_rng().gen_range(1..50);
//     println!("random number : {}", random_num);
// }


// if else statements

// fn main(){
//     let mut age = String::new();
//     println!("Enter your age..");

//     // input to enter your age
//     io::stdin().read_line(&mut age)
//     .expect("input your age");

//     // converting the age string to an integer
//     let myage: u32 = age.trim().parse()
//         .expect("wrong");


//     if( myage >= 1 ) && ( myage <= 18 ){
//         println!("you are young brother");
//         } else if ( myage == 21 ) || ( myage == 51 ){
//             println!("you are an old man")
//     } else if myage >= 65 {
//         println!("you are a freaking old brother")
//     } else {
//         println!("fuck off")
//         }
// }

// fn main(){
//     // let mut age: i32 = 18;
//     println!("Enter your age");

//     let mut age: String = String::new();

//     io::stdin().read_line(&mut age)
//         .expect("Wrong");

//     let myage: u32 = age.trim().parse()
//         .expect("Wrong");

//     let can_vote: bool = if myage >= 18 {
//         true
//     } else {
//         false
//     };
//     println!("you can vote : {}", can_vote);
// }


// fn main(){
//     // let age:u32 = 18;
//     println!("Enter your fucking age 😠");
//     let mut age: String = String::new();
//     io::stdin().read_line(&mut age)
//         .expect("msg");
//     let newage: u32 = age.trim().parse()
//         .expect("Wrong");

//     match  newage {
//         0..= 18 => println!("you are young"),
//         20 | 21 => println!("You are me"),
//         22..=u32::MAX => println!("Old Motherfucker"),
//         _ => println!("You are not important brother")
//     };
// }


// printing index of an array
// fn main(){
//     let arr: [u32; 5] = [1,2,3,4,5];
//     println!("1st : {}", arr[0]);
//     println!("2nd : {}", arr[1]);
//     println!("3rd : {}", arr[2]);
//     println!("4th : {}", arr[3]);
// }


// loops to get all odd numbers
// fn main(){
//     let arr: [i32; 11] = [1,2,3,4,5,6,7,8,9,10,11];
//     let mut arr_loop: usize = 0;

//     loop {
//         if arr[arr_loop] % 2 == 0{
//             arr_loop +=1;
//             continue;
//         }
//         if arr[arr_loop] == 11{
//             break;
//         }
//         println!("value {}", arr[arr_loop]);
//         arr_loop +=1;
        
//     }
// }

// fn main(){
//     let tupple: (u8, String, i32) = (47, "James".to_string(), 50_000_000);
//     let(v1, v2, v3) = tupple;
//     println!("Name : {}", v1);
// }


// functions

// fn get_sum(x: i32, y: i32, z: i32){
//     println!("{} + {} * {} = {}", x, y, z, x+y*z);
// }

// fn main(){
//     get_sum(7,9,6);
// }



// Hashmap

// fn main(){
//     let mut developer = HashMap::new();
//     // k: keys and v: values
//     developer.insert("DEVELOPER", "JAMES ABRAHAM"); 

//     for(k, v) in developer.iter(){
//         println!("{} = {}", k, v);
//     }
// }


// Structs

// fn main(){
//     struct Customer{
//         name : String,
//         address : String,
//         age : i32,
//         balance : f32,
//     }

//     let mut james: Customer = Customer {
//         name: String::from("James Abraham"),
//         address: String::from("Paris"),
//         age: 43,
//         balance: 4000.0,
//     };
    
// }

// let main(){
//    thread::spawn(|| {
//     for i in 1..25{
//         println!("Spawned thread : {}", i);
//         thread::sleep(Duration::from_millis(1));
//     }
//    });

//    for i in 1..20 {
//     println!("Main thread : {}", i);
//     thread::sleep(Duration::from_millis(1));
//    }
// }

for main() {
    thread::spawn(|| {
        for i i32 in 1..20{
            
        }
    });
}