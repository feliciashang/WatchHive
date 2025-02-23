use axum::{
  extract::ws::{WebSocket, WebSocketUpgrade},
  response::IntoResponse,
  routing::any,
  Router,
};

use std::net::SocketAddr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  tracing_subscriber::fmt()
    .with_max_level(tracing::Level::DEBUG)
    .init();
  let app = Router::new().route("/ws", any(handler));

  // run it with hyper
  let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;

  tracing::debug!("listening on {}", listener.local_addr().unwrap());
  axum::serve(
    listener,
    app.into_make_service_with_connect_info::<SocketAddr>(),
  )
  .await?;

  Ok(())
}

async fn handler(ws: WebSocketUpgrade) -> impl IntoResponse {
  tracing::debug!("client connected");
  ws.on_upgrade(move |socket| handle_socket(socket))
}

async fn handle_socket(mut socket: WebSocket) {
  while let Some(msg) = socket.recv().await {
    let msg = match msg {
      Ok(msg) => msg,
      Err(err) => {
        tracing::error!("client error: {}", err);
        return;
      }
    };

    tracing::debug!("received message: {:?}", msg);

    if let Err(err) = socket.send(msg).await {
      tracing::error!("failed to send message: {}", err);
      return;
    }
  }
}
