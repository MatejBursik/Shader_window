use axum::{
    extract::ws::{ WebSocketUpgrade, WebSocket, Message },
    response::IntoResponse,
    routing::get,
    Router
};
use tokio::{ sync::broadcast, net::TcpListener, runtime::Runtime };

async fn ws_handler(ws: WebSocketUpgrade, tx: broadcast::Sender<Vec<u8>>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, tx.subscribe()))
}

async fn handle_socket(mut socket: WebSocket, mut rx: broadcast::Receiver<Vec<u8>>) {
    while let Ok(frame) = rx.recv().await {
        if socket.send(Message::Binary(frame.into())).await.is_err() {
            break;
        }
    }
}

pub fn start_server(tx: broadcast::Sender<Vec<u8>>) {
    std::thread::spawn(move || {
        let rt = Runtime::new().unwrap();
        rt.block_on(async move {
            let app = Router::new().route(
                "/ws",
                get({
                    let tx = tx.clone();
                    move |ws| ws_handler(ws, tx.clone())
                })
            );

            let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();

            axum::serve(listener, app).await.unwrap();
        });
    });
}
