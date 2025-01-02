use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use std::sync::Mutex;

struct GameMaster {
    question: Mutex<String>,
    answer: Mutex<String>,
    // hint: Mutex<String>, //TODO: Implement a hint.
}

#[post("/questions")]
async fn create_question(req_body: String, game_master: web::Data<GameMaster>) -> impl Responder {
    let mut question = game_master.question.lock().unwrap();
    *question = req_body.clone();
    HttpResponse::Ok().body(format!("Question set: {:?}", *question))
}

#[post("/corrects")]
async fn create_correct(req_body: String, game_master: web::Data<GameMaster>) -> impl Responder {
    let mut answer = game_master.answer.lock().unwrap();
    *answer = req_body.clone();
    HttpResponse::Ok().body(format!("Answer set: {:?}", *answer))
}

#[post("/answers")]
async fn send_answer(req_body: String) -> impl Responder {
    //TODO: 回答送信の実装
    HttpResponse::Ok().body(req_body)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let game_master = web::Data::new(GameMaster {
        question: Mutex::new("".to_string()),
        answer: Mutex::new("".to_string()),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(game_master.clone())
            .service(create_question)
            .service(create_correct)
            .service(send_answer)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
