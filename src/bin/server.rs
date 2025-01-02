use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result};
use rust_sample::config::{IP, PORT};
use serde::{Deserialize, Serialize};
use std::sync::Mutex;

#[derive(Serialize, Deserialize)]
struct ResultObject {
    result: bool,
    remain_retry_count: u8,
}

struct GameMaster {
    question: Mutex<String>,
    answer: Mutex<String>,
    retry_count: Mutex<u8>,
    is_correct: Mutex<bool>,
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

#[get("/questions")]
async fn get_question(game_master: web::Data<GameMaster>) -> impl Responder {
    let question = game_master.question.lock().unwrap();
    HttpResponse::Ok().body(question.clone())
}

#[post("/answers")]
async fn send_answer(
    req_body: String,
    game_master: web::Data<GameMaster>,
) -> Result<impl Responder> {
    let answer = game_master.answer.lock().unwrap();
    let mut retry_count = game_master.retry_count.lock().unwrap();
    let mut is_correct = game_master.is_correct.lock().unwrap();
    let result = if req_body.trim() == *answer {
        *is_correct = true;
        (*is_correct, *retry_count)
    } else {
        *is_correct = false;
        *retry_count -= 1;
        (*is_correct, *retry_count)
    };
    let result_object = ResultObject {
        result: result.0,
        remain_retry_count: result.1,
    };
    Ok(web::Json(result_object))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let game_master = web::Data::new(GameMaster {
        question: Mutex::new("".to_string()),
        answer: Mutex::new("".to_string()),
        retry_count: Mutex::new(10),
        is_correct: Mutex::new(false),
    });
    HttpServer::new(move || {
        App::new()
            .app_data(game_master.clone())
            .service(create_question)
            .service(create_correct)
            .service(get_question)
            .service(send_answer)
    })
    .bind((IP, PORT))?
    .run()
    .await
}
