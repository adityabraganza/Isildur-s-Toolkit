#![allow(non_snake_case)]

use std::io;
use std::process::Command;

mod functions;
use crate::functions::{network, qol::{sout, clear}};

fn main(){
    clear();

    let mut should_continue = true;

    while should_continue{
        let user_input = get_input();

        match user_input[0].as_str(){
            "network"|"net"=>{
                if user_input.len() > 1{
                    match user_input[1].as_str(){
                        "local_device_discovery"|"ldd" => {
                            network::local_device_discovery();
                        },
                        _ => {
                            invalid_command(user_input);
                        }
                    }
                } else{
                    invalid_command(user_input);
                }
            },
            "clear"|"clr" => clear(),
            "end"|"e" => {
                sout("Exited program");
                should_continue = false;
            },
            "help"|"h" => {
                sout("
The format in which commands are written in the Commands section:

parent_1
    child_1
        child_child_1
            child_child_child_1 (explanation)
        child_child_2
    child_2
parent_2

While inputting the command you must go in descending order, from the parent command to the child command, with spaces in between.

A command will only function if it does not have any children. If it does have children it will say that the command was not found. All functions with no children will be followed by an explanation in brackets in the Commands section. In the above example format the only valid functions would be:

1. parent_1 child_1 child_child_1 child_child_child_1
2. parent_1 child_1 child_child_2
3. parent_1 child_2
4. parent_2

Where there is a '|' in a line there is a shorter alternative for the command that is accepted too. Ie. if the command is writen as 'network|net' then either 'network' or 'net' can be used to the same effect.

Commands:

help|h (Prints this help message)

end|e (Exits the program)

clear|clr (Clears the console)

network|net
    local_device_discovery|ldd (Displays all devices on the local network based on IP Address and MAC Address)
                ");
            },
            _ =>{invalid_command(user_input)}
        }
    }
}

fn invalid_command(user_input: Vec<String>){
    let mut command_string = "".to_string();

    for part in user_input{
        command_string = format!("{} {}", command_string, &part);
    }

    sout(&format!("Command '{}' not found. Use command 'help' or 'h' to get a list of commands.", command_string));
}

fn get_input() -> Vec<String>{
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let input_string = input.trim().to_string();
    let pointer_vector: Vec<&str> = input_string.split(" ").collect();

    let mut return_vector: Vec<String> = Vec::new();
    for pointer in pointer_vector{
        return_vector.push(pointer.to_string());
    }

    return return_vector;
}