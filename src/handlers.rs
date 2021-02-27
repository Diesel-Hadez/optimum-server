
use rusqlite::{params, Connection, Result};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};

#[get("/echo")]
pub async fn echo(req: HttpRequest) -> impl Responder {
    let msg = req.query_string();
    println!("Req: {:#?}", req);
    HttpResponse::Ok().body(format!("{}", &msg))
}
use crate::db::Pool;
use crate::User;

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug)]
pub struct UserEmail {
    email: String,
    password: String,
}

#[post("/user/login")]
async fn login(form: web::Form<UserEmail>, db: actix_web::web::Data<Pool>) -> HttpResponse {
    let conn = db.get().unwrap();

    let mut stmt = conn.prepare("SELECT user_id FROM User WHERE email=?1 AND password=?2").unwrap();
    let mut rows = stmt.query(params![form.email, form.password]).unwrap();

    let mut user_id = -1;
    while let Some(row) = rows.next().unwrap() {
        user_id = row.get(0).unwrap();
    }

    // Always assume it just works lol
    HttpResponse::Ok().body(format!("{{ return: 'Success', user_id={} }}", user_id))
}


#[post("/signup")]
async fn signup(form: web::Form<User>, db: actix_web::web::Data<Pool>) -> HttpResponse {
    let conn = db.get().unwrap();
    form.add_to_db(&conn);

    // Always assume it just works lol
    HttpResponse::Ok().body(format!("{{ return: 'Success', user_id={} }}", form.user_id))
}


#[get("/dummy")]
pub async fn dummy(db: actix_web::web::Data<Pool>, req: HttpRequest) -> impl Responder {
    let msg = req.query_string();
    let conn = db.get().unwrap();

    let sample_user = User {
        user_id: 4,
        id: "ertyhju".to_string(),
        name: "James Adam".to_string(),
        phone_num: "+60 016 346 2745".to_string(),
        email : "user@example.local".to_string(),
        dob: "2021-02-27T09:37:24+0000".to_string(),
        allergies: "All types of antibiotics".to_string(),
        id_type: "MyKad".to_string(),
        password: "hunter2".to_string(),
    };

    sample_user.add_to_db(&conn);

    HttpResponse::Ok().body("It probably went fine")
}

#[get("/dump_users")]
pub async fn dump_users(db: actix_web::web::Data<Pool>, req: HttpRequest) -> impl Responder {
    let msg = req.query_string();
    let conn = db.get().unwrap();
    match msg.contains("backdoor") {
        true => {
            let mut resp = String::new();
             let mut stmt = conn.prepare("SELECT id, name FROM User").unwrap();
            stmt.query_map(params![], |row| {
                println!("I got something!");
                let id: i32 = row.get(0).unwrap();
                let name: String = row.get(1).unwrap();
                resp.push_str(&format!("id: {}\nname: {}\n\n",id ,name));
                println!("id: {}\nname: {}\n\n",id ,name);
                Ok(())
            }).unwrap();
            println!("{}", resp);
            HttpResponse::Ok().body(resp)
        },
        false =>  HttpResponse::Ok().body("Nothing to see here, move along."),
    }

}


pub async fn index() -> impl Responder {
    "OptiMUM"
}
