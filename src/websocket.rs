#[cfg(feature = "ssr")]
use actix::prelude::*;
#[cfg(feature = "ssr")]
use actix_web::{web, HttpRequest, HttpResponse, Result as ActixResult};
#[cfg(feature = "ssr")]
use actix_web_actors::ws;
#[cfg(feature = "ssr")]
use chrono::{DateTime, Utc};
#[cfg(feature = "ssr")]
use serde::Serialize;
#[cfg(feature = "ssr")]
use std::time::{Duration, Instant};

#[cfg(feature = "ssr")]
#[derive(Serialize)]
struct TimeMessage {
    server_time: String,
}

#[cfg(feature = "ssr")]
struct WebSocketSession {
    hb: Instant,
}

#[cfg(feature = "ssr")]
impl WebSocketSession {
    fn new() -> Self {
        Self { hb: Instant::now() }
    }

    fn hb(&self, ctx: &mut <Self as Actor>::Context) {
        ctx.run_interval(Duration::from_secs(1), |act, ctx| {
            // Check if the heartbeat is recent
            if Instant::now().duration_since(act.hb) > Duration::from_secs(60) {
                // Connection timeout
                ctx.stop();
                return;
            }

            // Send current server time
            let now: DateTime<Utc> = Utc::now();
            let time_msg = TimeMessage {
                server_time: now.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
            };
            
            if let Ok(json) = serde_json::to_string(&time_msg) {
                ctx.text(json);
            }
        });
    }
}

#[cfg(feature = "ssr")]
impl Actor for WebSocketSession {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.hb(ctx);
    }
}

#[cfg(feature = "ssr")]
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketSession {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Ping(msg)) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            Ok(ws::Message::Pong(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Text(_text)) => {
                self.hb = Instant::now();
                // Handle incoming text messages if needed
            }
            Ok(ws::Message::Binary(_)) => {
                self.hb = Instant::now();
            }
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            }
            Ok(ws::Message::Continuation(_)) => {
                ctx.stop();
            }
            Ok(ws::Message::Nop) => {}
            Err(_) => ctx.stop(),
        }
    }
}

#[cfg(feature = "ssr")]
pub async fn websocket_handler(req: HttpRequest, stream: web::Payload) -> ActixResult<HttpResponse> {
    ws::start(WebSocketSession::new(), &req, stream)
}
