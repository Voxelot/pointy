use std::time::{Duration, Instant};

use actix::prelude::*;
use actix_files as fs;
use actix_web::{middleware, web, App, Error, HttpRequest, HttpServer};
use actix_web_actors::ws;
use actix_web::dev::Server;
use crate::ActorSet;
use std::sync::Arc;
use crate::model::{Messages};
use crate::model::Messages::Point;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// websocket connection is long running connection, it easier
/// to handle with an actor
struct MyWebSocket {
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    actor_set: Arc<ActorSet>
}

impl Actor for MyWebSocket {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start. We start the heartbeat process here.
    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

/// Handler for `ws::Message`
impl StreamHandler<ws::Message, ws::ProtocolError> for MyWebSocket {
    fn handle(&mut self, msg: ws::Message, ctx: &mut Self::Context) {
        // process websocket messages
        println!("WS: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                self.hb = Instant::now();
            }
            ws::Message::Text(text) =>  {
                let message: Messages = serde_json::from_str(text.as_ref()).unwrap_or_default();
                match message {
                    Point(_) => {
                        self.actor_set.laser_actor.do_send(message);
                    },
                    _ => {
                        println!("unrecognized command {}", text);
                    }
                }
                ctx.text(text)
            },
            ws::Message::Binary(bin) => ctx.binary(bin),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl MyWebSocket {
    fn new(actor_set: Arc<ActorSet>) -> Self {
        Self {
            hb: Instant::now(),
            actor_set
        }
    }

    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping("");
        });
    }
}

pub fn start(actor_set: ActorSet) -> Result<Server, Error> {
    let actor_set: Arc<ActorSet> = Arc::new(actor_set);
    let server = HttpServer::new(move || {
        let actor_set = actor_set.clone();
        App::new()
            // enable logger
            .wrap(middleware::Logger::default())
            // websocket route
            .service(web::resource("/ws/").route(web::get().to(move |r: HttpRequest, stream: web::Payload| {
                println!("{:?}", r);
                let res = ws::start(MyWebSocket::new(actor_set.clone()), &r, stream);
                println!("{:?}", res.as_ref().unwrap());
                res
            })))
            // static files
            .service(fs::Files::new("/", "static/").index_file("index.html"))
    })
    // start http server on 127.0.0.1:8080
    .bind("127.0.0.1:8080")?
    .start();
    Ok(server)
}
