use rusqlite::{params, Connection, Result};
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, HttpRequest};

#[derive(Debug)]
struct User{
    name: String,
    phone_num: String,
    email: String,
    dob: String,
    allergies: String,
    id: u32,
    id_type: String,
    password: String,
}

impl User{
    fn add_to_db(&self, conn: &Connection){
        println!("Ha");
        conn.execute("INSERT INTO User (name, phone_num, email, dob, allergies, id, id_type, password) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)", params![self.name, self.phone_num, self.email, self.dob, self.allergies, self.id, self.id_type, self.password]).unwrap();
    }
}


#[get("/echo")]
async fn echo(req: HttpRequest) -> impl Responder {
    let msg = req.query_string();
    println!("Req: {:#?}", req);
    HttpResponse::Ok().body(format!("{}", &msg))
}

async fn index() -> impl Responder {
    "OptiMUM"
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let conn = Connection::open_in_memory().unwrap();
    // Note, I'm pretty sure there's something other than Text which is a better fit.
    // Yay for storing passwords in the database as plaintext
    println!("Creating table!");
    conn.execute(
    "CREATE TABLE User (
                    id              INTEGER PRIMARY KEY,
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
        id: 1,
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
    HttpServer::new(|| {
        App::new()
            .service(echo)
            .route("/", web::get().to(index))

    }).bind(format!("127.0.0.1:{}",port)).unwrap().run().await
}
