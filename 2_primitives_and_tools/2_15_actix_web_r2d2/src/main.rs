extern crate r2d2;
extern crate r2d2_postgres;
extern crate postgres;

use std::thread;
use r2d2_postgres::{TlsMode, PostgresConnectionManager};

struct Person {
    id: i32,
    username: String
}

fn main() {
    let manager = PostgresConnectionManager::new("postgres://jeka:0454@localhost/diesel_demo", TlsMode::None).unwrap();
    let pool = r2d2::Pool::new(manager).unwrap();

    for i in 0..10i32 {
        let pool = pool.clone();
        thread::spawn(move || {
            let conn = pool.get().unwrap();
           // conn.execute("INSERT INTO foo (bar) VALUES ($1)", &[&i]).unwrap();

            for row in &conn.query("SELECT id, username FROM users", &[]).unwrap() {
                let person = Person {
                    id: row.get(0),
                    username: row.get(1)
                };
                println!("Found person {}: {}", person.id, person.username);
            }
        });
    }
}

