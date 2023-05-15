use std::{io::{self, Write}};

use redis::{Commands, Connection};

fn main() {
    let client = redis::Client::open("redis://127.0.0.1/").unwrap();
    let mut con = client.get_connection().unwrap();

    loop {
        println!("Type a username");
        let input = get_input() + &":friends".to_string();
        println!("type another username to add as friend of new user");
        let user_to_add = get_input();
        let _: () = con.sadd(input, user_to_add).unwrap();
    }

}

fn get_input() -> String {
    let mut input = String::new();
    io::stdout().flush();
    io::stdin().read_line(&mut input);
    return input.trim().to_string();
}

fn publish(con: Connection) {

}
