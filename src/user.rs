use rusqlite::{params, Connection, Result};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize)]
pub struct User{
    pub name: String,
    pub phone_num: String,
    pub email: String,
    pub dob: String,
    pub allergies: String,
    pub user_id: u32,
    pub id: String,
    pub id_type: String,
    pub password: String,
}

impl User{
    pub fn new(id: String, name: String, phone_num: String, email: String, dob: String, allergies: String, id_type: String, password: String) -> User {
        User {
            user_id: 1,
            id,
            name,
            phone_num,
            email,
            dob,
            allergies,
            id_type,
            password,
            
        }
    }
    pub fn add_to_db(&self, conn: &Connection){
        println!("Ha");
        conn.execute("INSERT INTO User (name, phone_num, email, dob, allergies, id, id_type, password) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", params![self.name, self.phone_num, self.email, self.dob, self.allergies, self.id, self.id_type, self.password]).unwrap();
    }
}


