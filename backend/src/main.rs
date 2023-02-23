mod api;
mod websockets;

use actix_files::Files;
use actix_web::{
    middleware, post, web, App, Error, HttpRequest, HttpResponse, HttpServer, Responder,
};
use actix_web_actors::ws;
use hive_lib::{history::History, state::State};
use websockets::echo::Echo;

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[post("/board/{id}/move/{move}")]
async fn board_record_move(path: web::Path<(u32, String)>) -> impl Responder {
    let (board_id, board_move) = path.into_inner();
    println!("board_id: {}, move: {}", board_id, board_move);
    let game = "game.txt";
    let mut history;
    match History::from_filepath(game) {
        Ok(h) => history = h,
        Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
    };
    println!("{:?}", history);
    let mut state;
    match State::new_from_history(&history) {
        Ok(s) => state = s,
        Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
    };
    // TODO this is hacky af
    let tokens = board_move.split_whitespace().collect::<Vec<&str>>();
    let piece = *tokens.get(0).unwrap();
    let position = *tokens.get(1).unwrap();
    match state.play_turn_from_notation(piece, position) {
        Ok(_) => {},
        Err(e) => return HttpResponse::BadRequest().body(format!("{}", e)),
    };
    state.history.write_move(game, state.turn, board_move);
    HttpResponse::Ok().into()
}

/// WebSocket handshake and start `Echo` actor.
async fn echo_ws(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
    ws::start(Echo::new(), &req, stream)
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));

    log::info!("starting HTTP server at http://127.0.0.1:8080");

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(middleware::Logger::default())
            .service(web::resource("/ws/").route(web::get().to(echo_ws)))
            .service(web::scope("/api").service(echo).service(board_record_move))
            .service(Files::new("/", "dist/").index_file("index.html"))
    })
    .workers(4)
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
