use actix_web::{App, HttpServer, web};
use jemallocator::Jemalloc;
use std::sync::Arc;

#[global_allocator]
static GLOBAL: Jemalloc = Jemalloc;

mod handler;
mod repository;
mod usecase;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    if let Ok(version) = jemalloc_ctl::version::read() {
        println!("✅ jemalloc version: {}", version);
    } else {
        println!("⚠️ jemalloc not detected");
    }
    
    // Dependency injection
    let repo = Arc::new(repository::MessageRepoImpl) as Arc<dyn repository::MessageRepository>;
    let usecase = Arc::new(usecase::MessageUsecaseImpl::new(repo)) as Arc<dyn usecase::MessageUsecase>;

    // Logging test
    log::info!("Server starting on http://127.0.0.1:8080");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(usecase.clone()))
            .configure(handler::init_routes)
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
