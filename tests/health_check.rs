use std::net::TcpListener;

use zero2prod::run;

#[actix_web::test]
async fn health_check_works() {
    let listener = spawn_app();

    println!("listnet {}", listener);

    let client = reqwest::Client::new();

    let response = client
        .get(&format!("http://{}/health", &listener))
        .send()
        .await
        .expect("Faild to send request");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Faild to bind to address.");

    let port = listener.local_addr().unwrap().port();

    let server = run(listener).expect("Faild to bind address!");

    let _ = tokio::spawn(server);

    format!("127.0.0.1:{}", port)
}
