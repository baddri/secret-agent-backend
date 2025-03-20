use std::net::SocketAddr;

use axum::{
    extract::{ConnectInfo, WebSocketUpgrade, ws::Message},
    response::IntoResponse,
};
use axum_extra::{TypedHeader, headers::UserAgent};
use futures::{Sink, SinkExt, Stream, StreamExt};

#[tracing::instrument(skip(ws, user_agent))]
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    user_agent: Option<TypedHeader<UserAgent>>,
    ConnectInfo(addr): ConnectInfo<SocketAddr>,
) -> impl IntoResponse {
    let user_agent = if let Some(TypedHeader(user_agent)) = user_agent {
        user_agent.to_string()
    } else {
        "Unknown browser".to_string()
    };

    tracing::info!("{user_agent} at {addr} connected.");
    ws.on_upgrade(move |socket| {
        let (write, read) = socket.split();
        handle_socket(write, read)
    })
}

#[tracing::instrument(skip_all)]
pub async fn handle_socket<W, R>(mut write: W, mut read: R)
where
    W: Sink<Message> + Unpin,
    R: Stream<Item = Result<Message, axum::Error>> + Unpin,
{
    while let Some(Ok(msg)) = read.next().await {
        match msg {
            Message::Ping(_) => match write.send(Message::Pong("pong".into()).into()).await {
                Ok(()) => {
                    tracing::info!("sending pong message!");
                }
                Err(_err) => {
                    tracing::error!("error while responding ping message");
                }
            },
            _ => {}
        };
    }
}
