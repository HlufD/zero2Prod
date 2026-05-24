use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:4000").expect("Faild to bind to address");

    println!("Server is running on port 4000");

    run(listener)?.await
}
