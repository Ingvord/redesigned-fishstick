use actix_web::{web, App, HttpServer, Responder};
use std::time::Instant;
use tokio::time::{sleep, Duration};

// Function for CPU-bound load simulation
async fn cpu_load(duration: web::Path<u64>) -> impl Responder {
    let duration_ms = *duration;
    let start = Instant::now();

    // Busy loop that performs CPU work for the specified duration
    while start.elapsed().as_millis() < duration_ms.into() {
        // Some arbitrary math to simulate CPU load
        let _x: f64 = (1..1000).map(|x| (x as f64).sqrt()).sum();
    }

    format!("Simulated CPU load for {} milliseconds", duration_ms)
}

// Function for IO-bound load simulation
async fn io_load(duration: web::Path<u64>) -> impl Responder {
    let duration_ms = *duration;

    // Simulate IO-bound work by sleeping
    sleep(Duration::from_millis(duration_ms)).await;

    format!("Simulated IO load for {} milliseconds", duration_ms)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/cpu_load/{duration}", web::get().to(cpu_load))
            .route("/io_load/{duration}", web::get().to(io_load))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
