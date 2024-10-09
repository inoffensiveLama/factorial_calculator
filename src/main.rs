use std::io;
use std::thread;
use std::time::Duration;

use webbrowser;


//this is the main function. it is simply calling the other functions in the correct order.
//at the end it will print out all the factorial numbers
fn main() {
    let mut user_numbers: Vec<i32> = Vec::new();
    loop {
        let user_factorial: i32 = factorial(ask_user_for_number());
        println!("The factorial of your number is: {}", user_factorial);
        user_numbers.push(user_factorial);


        let user_choice = ask_user_continue();
        if user_choice == 0 {
            break;
        }
    }
    println!("Your numbers were: ");
    for number in &user_numbers {
        println!("{}", number);
    }
}


//this function is called recusively for calculation.
//It returns either the recursion and calls itself with it, or it will return 1, and thereby breaking the recursion
fn factorial(i: i32) -> i32 {
    if i > 1 {
        return i * factorial(i - 1);
    } else {
        return 1;
    }
}


//this function asks the user for a number between 1 and 12 (12 is the highest without it causing memory issues)
//if the user enters a number bigger than 12 it will call the function "easter egg"
fn ask_user_for_number() -> i32 {
    let mut input = String::new();

    println!("Please enter an integer between 1 and 12 to calculate its factorial: ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let number: i32 = input.trim().parse().expect("Please enter an integer!");


    match number {
        1..=12 => return number,
        13.. => easter_egg(),
        _ => {
            println!("Invalid input. Please enter an integer between 1 and 12.");
            ask_user_for_number()
        }
    }

}


//this function is here to ask the user if they want to continue. It will return a 1 or a 0, depending on if the user enters y or n.
//there is also some validation happening. It will ask the user for as long as it takes to input either n or y
fn ask_user_continue() -> i32 {
    let mut input = String::new();

    println!("Do you want to continue? (y/n): ");

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let trimmed_input = input.trim();

    match trimmed_input {
        "y" | "Y" => return 1,
        "n" | "N" => return 0,
        _ => {
            println!("Invalid input. Please enter \"y\" or \"n\".");
            ask_user_continue()
        }
    }
}


//this is the easter egg. The text explains why recursion is dangerous and will lead to memory issues if not being used carefully.
//it will then wait a couple of seconds and open a browser where some coding rules of NASA are explained.
fn easter_egg() -> i32{
    println!("Did you know that recursion can be very dangerous for mission critical tasks?");
    println!("Maybe you are wondering why, well that is because with each call there is new stuff added to the stack.");
    println!("And all of this only gets resolved, once the very last call is being taken care of.");
    println!("you still want to use recursion? Well even NASA doesn't allow it.");
    println!("Don't believe me? Check it out. (A webpage is about to open.)");
    thread::sleep(Duration::from_secs(10));
    open_browser();

    println!("Since you had to put in a higher number just to see how the program crumbles into pieces I will set your number to 12.");

    return 12;
}


//this just takes care of opening the browser and going to the right page.
fn open_browser() {
    let url = "https://en.wikipedia.org/wiki/The_Power_of_10:_Rules_for_Developing_Safety-Critical_Code";
    
    if webbrowser::open(url).is_ok() {
        println!("Opened browser to {}", url);
    } else {
        println!("Failed to open the browser");
    }
}