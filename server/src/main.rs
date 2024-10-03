mod robot;



#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let _ = robot::Robot::new().await;


    Ok(())
}