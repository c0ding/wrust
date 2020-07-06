use std::time::Duration;
use std::{io, thread};

use actix::io::SinkWrite;
use actix::*;
use actix_codec::Framed;

use awc::{error::WsProtocolError, ws::{Codec, Frame, Message}, BoxedSocket, Client};
use bytes::Bytes;
use futures::stream::{SplitSink, StreamExt};

struct ChatClient(SinkWrite<Message, SplitSink<Framed<BoxedSocket, Codec>, Message>>);

#[derive(Message)]
#[rtype(result = "()")]
struct ClientCommand(String);

impl ChatClient {
  fn heartbeat(&self, context: &mut Context<Self>) {
    context.run_later(Duration::new(1,0), |actor, context|{
      actor.0.write(Message::Ping(Bytes::from_static(b""))).unwrap();
      actor.heartbeat(context);
    });
  }
}

impl Handler<ClientCommand> for ChatClient {
  type Result = ();

  fn handle(&mut self, message: ClientCommand, _context: &mut Context<Self>) {
    self.0.write(Message::Text(message.0)).unwrap();
  }
}

impl Actor for ChatClient {
  type Context = Context<Self>;

  fn started(&mut self, context: &mut Context<Self>) {
    self.heartbeat(context)
  }

  fn stopped(&mut self, _: &mut Context<Self>) {
    println!("Disconnected");
    System::current().stop();
  }
}

impl StreamHandler<Result<Frame, WsProtocolError>> for ChatClient {
  fn handle(&mut self, message: Result<Frame, WsProtocolError>, _: &mut Context<Self>) {
    if let Ok(Frame::Text(text)) = message {
      println!("Server: {:?}", text)
    }
  }

  fn started(&mut self, _ctx: &mut Context<Self>) {
    println!("Connected");
  }

  fn finished(&mut self, context: &mut Context<Self>) {
    println!("Server disconnected");
    context.stop();
  }
}

impl actix::io::WriteHandler<WsProtocolError> for ChatClient {}

fn main() {
  std::env::set_var("RUST_LOG", "actix_web=info");
  env_logger::init();

  let system = System::new("wrust-client");
  
  Arbiter::spawn(async {
    let (response, framed) = Client::new()
      .ws("http://0.0.0.0:3000/ws/")
      .connect()
      .await
      .map_err(|error| {
        println!("Error: {}", error)
      })
      .unwrap();

    println!("{:?}", response);
    let (sink, stream) = framed.split();
    let address = ChatClient::create(|context| {
      ChatClient::add_stream(stream, context);
      ChatClient(SinkWrite::new(sink, context))
    });

    thread::spawn(move || loop {
      let mut cmd = String::new();
      if io::stdin().read_line(&mut cmd).is_err() {
        println!("error");
        return;
      }
      address.do_send(ClientCommand(cmd));
    });
  });

  system.run().unwrap();
}
