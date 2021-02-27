
use rusqlite::{params, Connection, Result};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
mod user;
use user::User;
use r2d2_sqlite::{self, SqliteConnectionManager};


mod handlers;
use handlers::{echo, index, dummy, dump_users, login, signup};

mod db;
use db::{Pool};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = SqliteConnectionManager::file("database.db");
    let pool = Pool::new(manager).unwrap();
    let conn = pool.get().unwrap();
    // Note, I'm pretty sure there's something other than Text which is a better fit.
    // Yay for storing passwords in the database as plaintext
    println!("Creating table!");
    conn.execute(
    "CREATE TABLE User (
                    user_id         INTEGER PRIMARY KEY,
                    id              TEXT NOT NULL,
                    name            TEXT NOT NULL,
                    phone_num       TEXT NOT NULL,
                    email           TEXT NOT NULL,
                    dob             TEXT NOT NULL,
                    allergies       TEXT NOT NULL,
                    id_type         TEXT NOT NULL,
                    password        TEXT NOT NULL
                    )",
                params![],
                ).unwrap();
        println!("Done Creating table!");

    let sample_user = User {
        user_id: 1,
        id: "234567".to_string(),
        name: "John Travolta".to_string(),
        phone_num: "+60 016 346 2745".to_string(),
        email : "user@example.local".to_string(),
        dob: "2021-02-27T09:37:24+0000".to_string(),
        allergies: "All types of antibiotics".to_string(),
        id_type: "MyKad".to_string(),
        password: "hunter2".to_string(),
        
    };
    sample_user.add_to_db(&conn);
    let port: u16 = 8000;
    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(echo)
            .service(dummy)
            .service(dump_users)
            .service(login)
            .service(signup)
            .route("/", web::get().to(index))

    }).bind(format!("0.0.0.0:{}",port)).unwrap().run().await
}
