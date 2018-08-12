extern crate rusqlite;

use std::io;
use io::prelude::*;
use rusqlite::Connection;
use std::path::Path;
use std::env;
use std::fs::File;
use std::string::String;

#[derive(Debug)]
struct Item {
    id: i32,
    name: String,
    price: f64, 
    description: String,
    ingredients: String
}

fn print_items(path: &str) {
    let conn = Connection::open(path).unwrap();
    let mut item_query = conn.prepare("SELECT id, name, price, description, ingredients FROM Items").unwrap();
    let item_iter = item_query.query_map(&[], |row|{
        Item {
            id: row.get(0),
            name: row.get(1),
            price: row.get(2),
            description: row.get(3),
            ingredients: row.get(4)
        }
    }).unwrap();

    // 'x' is a simple counter for item number in the list
    let mut x: i32 = 0;
    for item in item_iter {
        x = x + 1;
        println!("Item #{}", x);
        println!("--------------");
        println!("{:?}\n", item.unwrap());
    }
}

fn create_db(path: &str) {
    let f = File::create(path).expect("Error creating DB.");

    let conn = Connection::open(path).unwrap();
    conn.execute("CREATE TABLE Items (
                  id    INTEGER PRIMARY KEY,
                  name  TEXT NOT NULL,
                  price REAL NOT NULL,
                  description TEXT,
                  ingredients TEXT
                  )", &[]).unwrap();
}

fn insert_item(path: &str) {
    let mut input = String::new();
    let stdin = io::stdin();
    stdin.lock().read_line(&mut input).unwrap();

    let mut name = String::new();
    println!("Name: ");
    stdin.lock().read_line(&mut name).unwrap();

    let mut price = String::new();
    println!("Price: ");
    stdin.lock().read_line(&mut price).unwrap();

    let mut description = String::new();
    println!("Description: ");
    stdin.lock().read_line(&mut description).unwrap();

    let mut ingredients = String::new();
    println!("Ingredients: ");
    stdin.lock().read_line(&mut ingredients).unwrap();

    let conn = Connection::open(path).unwrap();
    conn.execute("INSERT INTO Items (name, price, description, ingredients) VALUES (?1, ?2, ?3, ?4)", 
    &[&name, &price, &description, &ingredients]).unwrap();

}

fn main() {
    let db_path = "src/roffee.db";
    if Path::new(db_path).exists() == false {
        println!("Can't find DB, creating new one...");
        create_db(db_path);
    }

    let args: Vec<String> = env::args().collect();

    for arg in &args {
        if arg == "list" {
            print_items(db_path);
        }
        if arg == "new" {
            insert_item(db_path);
        }
    }

    loop {
        let mut input = String::new();
        let stdin = io::stdin();
        stdin.lock().read_line(&mut input).unwrap();
        println!("Input: {}", input);
        // This if is really gross, as in it fundamentally hurts my soul. Better solution would be awesome
        if input.contains("!new") {
            insert_item(db_path);
        } else if input.contains("!list") {
            print_items(db_path);
        }
    }
}
