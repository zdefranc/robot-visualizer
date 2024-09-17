mod robot;

use std::sync::Arc;

use axum::routing::get;
use robot::{robot_state::RobotState, RobotLock};
use socketioxide::{
    extract::{Data, SocketRef, State},
    SocketIo,
};
use tokio::time::{sleep, Instant, Duration};
use tokio::sync::RwLock;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    // todo!("Accept msg to set joint positions");

    // todo!("Accept msg to set to set end effector position");

    socket.on(
        "set joint state",
        |Data::<RobotState>(data), robot_lock: State<RobotLock>| async move {
            info!("Set state {}", data.elbow_rotation_deg);
            
            {
                robot_lock.write().await.set_target_state(data);
            }
        },
    );


    socket.on_disconnect(|| async move {
        info!("Client disconnected");
    });
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Enables logging
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;
    
    let robot_sim: RobotLock = robot::Robot::new();

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
    
    // Remove
    let c_lock: RobotLock = robot_sim.clone();

    // let n = robot_sim.state.read().await;

    // Task to send a message every second
    tokio::spawn(async move {
        loop {
            let start = Instant::now();
            // Send a status message to the client
            {
                let robot = c_lock.read().await;
                let _ = io_handler.read().await.emit("state", robot.state);
            }


            let loop_duration = Instant::now().duration_since(start);
            if let Some(sleep_duration) = Duration::from_millis(1000).checked_sub(loop_duration) {
                sleep(sleep_duration).await;
            }
            
        }
    });

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;


    Ok(())
}