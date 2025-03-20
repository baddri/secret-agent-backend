use futures::{SinkExt, StreamExt};
use secret_agent::utils::test::spawn_server_e2e;

#[tokio::test]
async fn ws_integration_test() {
    use tokio_tungstenite::tungstenite;

    let addr = spawn_server_e2e().await;

    let (mut socket, _) = tokio_tungstenite::connect_async(format!("ws://{addr}/ws"))
        .await
        .unwrap();

    socket
        .send(tungstenite::Message::Ping("ping".into()))
        .await
        .unwrap();

    let msg = match socket.next().await.unwrap().unwrap() {
        tungstenite::Message::Pong(msg) => msg,
        _ => panic!("expected pong!"),
    };

    assert_eq!(std::str::from_utf8(&msg).unwrap(), "pong");
}
