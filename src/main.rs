use std::{io::{self, Write}};

use redis::{Commands, Connection};

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    loop {
        println!("Choose f to add a friend relation, choose p to publish a message");
        let choice = get_input();
        match choice.as_str() {
            "f" => {
                add_relation(&mut con);
            },
            "p" => {

            },
            _ => {
                println!("invalid input")
            }
        }
    }
}

fn get_input() -> String {
    let mut input = String::new();
    io::stdout().flush();
    io::stdin().read_line(&mut input);
    return input.trim().to_string();
}

fn add_relation(con: &mut Connection) {
    println!("Type a username");
    let input = get_input();
    let input_friends = input.clone() + &":friends".to_string();
    println!("type another username to add as friend of new user");
    let user_to_add = get_input();
    let _: () = con.sadd(input_friends, user_to_add).unwrap();
    let _: () = redis::cmd("BF.ADD").arg("users").arg(input).execute(con);
}

fn publish(con: &mut Connection, message: String) {
    println!("Who are you?");
    let user = get_input();
    let bf_result: usize = redis::cmd("BF.EXISTS").arg("users").arg(&user).query(con).unwrap();

    if bf_result == 0 {
        println!("user does not exist");
    }

    println!("What do you wanna say?");
    let message = get_input();
    let _: () = con.publish("", message).unwrap();
}