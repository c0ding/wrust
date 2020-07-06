use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;

const HEARTBEAT: Duration = Duration::from_secs(3);
const CLIENT_TIMEOUT: Duration = Duration::from_secs(13);

struct WebSocket {
  heartbeat: Instant,
}

impl WebSocket {
  fn new() -> Self {
    Self { heartbeat: Instant::now() }
  }

  fn heartbeat(&self, context: &mut <Self as Actor>::Context) {
    context.run_interval(HEARTBEAT, |actor, context| {
      if Instant::now().duration_since(actor.heartbeat) > CLIENT_TIMEOUT {
        println!("Websocket Client heartbeat failed!");
        context.stop();
        return;
      }

      context.ping(b"");
    });
  }
}

impl Actor for WebSocket {
  type Context = ws::WebsocketContext<Self>;

  fn started(&mut self, context: &mut Self::Context) {
    self.heartbeat(context);
  }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocket {
  fn handle(&mut self, message: Result<ws::Message, ws::ProtocolError>, context: &mut Self::Context) {
    match message {
      Ok(ws::Message::Ping(message)) => {
        self.heartbeat = Instant::now();
        context.pong(&message);
      }
      Ok(ws::Message::Pong(_)) => {
        self.heartbeat = Instant::now();
      }
      Ok(ws::Message::Text(text)) => {
        context.text(text)
      }
      Ok(ws::Message::Binary(bin)) => {
        context.binary(bin)
      }
      Ok(ws::Message::Close(reason)) => {
        context.close(reason);
        context.stop();
      }
      _ => context.stop(),
    }
  }
}

async fn ws_index(request: HttpRequest, stream: web::Payload) -> Result<HttpResponse, Error> {
  ws::start(WebSocket::new(), &request, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
  std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
  env_logger::init();

  HttpServer::new(|| {
    App::new()
      .wrap(middleware::Logger::default())
      .service(web::resource("/ws/").route(web::get().to(ws_index)))
      .service(fs::Files::new("/", "frontend/").index_file("index.html"))
  })
  .bind("0.0.0.0:3000")?
  .run()
  .await
}
