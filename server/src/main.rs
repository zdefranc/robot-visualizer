mod robot;

use std::sync::Arc;

use axum::routing::get;
use socketioxide::{
    extract::{Data, SocketRef, State},
    SocketIo,
};
use tokio::time::{sleep, Duration};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use serde_json::json;

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    // socket.on(
    //     "message",
    //     |socket: SocketRef, Data::<MessageIn>(data), store: State<state::MessageStore>| async move {
    //         info!("Received message: {:?}", data);

    //         let response = state::Message {
    //             text: data.text,
    //             user: format!("anon-{}", socket.id),
    //             date: chrono::Utc::now(),
    //         };

    //         store.insert(&data.room, response.clone()).await;

    //         let _ = socket.within(data.room).emit("message", response);
    //     },
    // )

    socket.on_disconnect(|| async move {
        info!("Client disconnected");
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Enables logging
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    
    let robot_sim = Arc::new(RwLock::new(robot::Robot::default()));

    let (layer, io) = SocketIo::builder().with_state(robot_sim.clone()).build_layer();

    io.ns("/", on_connect);

    let io_handler: Arc<RwLock<SocketIo>> = Arc::new(RwLock::new(io));

    let app = axum::Router::new()
        // What does this do? Do I need to include it?
        .route("/", get(|| async { "Robot Server" }))
        .with_state(io_handler.clone())
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    info!("Starting server");

    let c_lock = robot_sim.clone();

    // let n = robot_sim.state.read().await;

    // Task to send a message every second
    tokio::spawn(async move {
        loop {
            // Send a message to the client
            let robot = c_lock.read().await; {
                let _ = io_handler.read().await.emit("state", robot.state.clone());
            }
            sleep(Duration::from_secs(1)).await;
        }
    });

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;


    Ok(())
}