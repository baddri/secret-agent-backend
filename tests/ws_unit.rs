use axum::extract::ws::Message;
use futures::{SinkExt, StreamExt};
use secret_agent::routes::ws::handle_socket;

#[tokio::test]
async fn ws_unit_test() {
    let (socket_write, mut test_rx) = futures::channel::mpsc::channel(1024);
    let (mut test_tx, socket_read) = futures::channel::mpsc::channel(1024);

    tokio::spawn(handle_socket(socket_write, socket_read));

    test_tx
        .send(Ok(Message::Ping("ping".into())))
        .await
        .unwrap();

    let msg = match test_rx.next().await.unwrap() {
        Message::Pong(msg) => msg,
        _ => panic!("expected pong message got error"),
    };

    assert_eq!(std::str::from_utf8(&msg).unwrap(), "pong");
}
