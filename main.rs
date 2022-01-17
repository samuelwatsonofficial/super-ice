use std::env::temp_dir;
use std::io::{self, BufRead};
use std::process::exit;
use std::ptr::null;
//custom icecream tuple or "class"
struct IceCream {price: f64,flavour: Box<str>,nuts: bool,flake: bool,sauce: bool}

//custom basic input system
fn input(text:&str) -> String {
    //prints text for user
    println!("{}",text);
    let mut buffer:String = "".to_string();
    //match statement to check if the user is supplying a bad unicode string
    match io::stdin().lock().read_line(&mut buffer)
    {
        Ok(..) => return String::from(buffer.trim()),
        Err(..) => return String::from("empty"),
    }
}
//custom input system specifically for numbers, this makes the user re-enter a string if not between the set values
fn numcheck(text:&str,low:i8,high:i8) -> i8
{
    //infinite loop until broken by return
    loop
    {
        //match statement to check if the user has entered an i8
        match input(text, ).parse()
        {
            //checks if value is between given limits
            Ok(x) => {
                if (x<=high) & (x>=low)
                {
                    return x;
                }
                println!("please enter a number between {} and {}",low,high);
            }
            //throws an error if user enters non-parseable string
            Err(x) => println!("{}",x),
        }
    }
}
//custom input system specifically for y and n
fn yesno(text:&str) -> bool
{
    //infinite loop until broken by return
    loop
    {
        //creates non mutable variable called temp as a reference of a string
        let temp:&str =&input(text);
        //checks if the user given input is y n or else and only breaks loop if y or n
        match temp
        {
            "y" => {
                return true;
            }
            "n" => {
                return false;
            }
            _ => println!("only input y or n"),
        }
    }
}
//small easily callable menu system
fn menu()
{
    //uses nocheck method to find what the user wants to do
    match numcheck("exit(type 1), check prices(type 2) order ice cream(type 3)",1,3)
    {
        1=>exit(0),
        2=>check(),
        3=>order(),
        //since it is impossible to give a number that is not between 1 and 3 the _/other values do nothing but must exist for compilation
        _ => {}
    }
}
//tiny function that tells the user what everything costs
fn check()
{
    println!("Whippy ice cream is £1,Chocolate or Strawberry ice cream cost £1.50, nuts cost an extra £0.20, a flake or sauce costs an extra £0.30");
    menu()
}
//order function that loops asking the user for as many ice creams as they want
fn order()
{
    //gets number of ice creams
    let number_of_ice_creams = numcheck("how many ice cream cones do you want?", 1, 3);
    //creates list of ice cream struct (a struct is a collection of variables)
    let mut ice_creams:Vec<IceCream>=Vec::new();
    //loops through asking
    for idx in 0..number_of_ice_creams
    {
        let current_ice_cream = icecreate();
        println!("the price of that cone is £{0:.2}", current_ice_cream.price);
        ice_creams.push(current_ice_cream);
    }
    //returns to menu when finished
    menu()
}
//order function to ask user for ice cream details
fn icecreate() -> IceCream
{
    //takes input from user and processes with numcheck to detect what flavour is wanted
    let numtype = numcheck("what do you want for your scoop? Whippy(type 1), Chocolate(type 2), Strawberry(type 3), Vanilla(type 4) or Luxury ice cream(type 5)",1,5);
    //sets flavour to what the user asks for using match statement
    let flavour:&str= match numtype
    {
        1=> "Whippy",
        2=> "Chocolate",
        3=> "Strawberry",
        4=> "Vanilla",
        5=> "Luxury",
        //underscore meaning anything other than 1-5
        _ => "none"
    };
    //creates mutable variable called price which is dependent on the ice cream type
    let mut price:f32=
        match numtype
        {
            1 => 1.0,
            2..=4 => 1.5,
            5 => 1.75,
            _ => 0.0,
        } as f32;
    //takes user input on whether they want nuts and a flake or sauce
    let nuts:bool =yesno("do you want nuts?");
    let flake:bool =yesno("do you want a flake?");
    let sauce:bool =yesno("do you want sauce?");
    //uses these booleans for branchlessly programmed calculations
    price+=0.2*((nuts as i32) as f32);
    price+=0.3*((flake as i32) as f32);
    price+=0.3*((sauce as i32) as f32);
    //creates new variable of type IceCream as defined on line 6
    let tempice = IceCream
    {
        price: price as f64,
        flavour: Box::from(flavour),
        nuts,
        flake,
        sauce
    };
    //after inputting all data this function returns IceCream to the order function
    return tempice;
}
//start of the program and menu call
fn main()
{
    menu();
}