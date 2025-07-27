use crate::usecase::MessageUsecase;
use actix_web::{HttpResponse, Responder, web};
use jemalloc_ctl::stats;
use std::sync::Arc;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.route("/", web::get().to(get_message));
    cfg.route("/debug/memory", web::get().to(get_mem));
}

pub async fn get_message(usecase: web::Data<Arc<dyn MessageUsecase>>) -> impl Responder {
    let msg = usecase.get_message().await;
    HttpResponse::Ok().body(msg)
}

pub async fn get_mem() -> impl Responder {
    let allocated = stats::allocated::read().unwrap_or(0);
    let active = stats::active::read().unwrap_or(0);
    let resident = jemalloc_ctl::stats::resident::read().unwrap_or(0);

    let msg = format!(
        "allocated: {:.2} MiB\nactive: {:.2} MiB\nresident (RSS): {:.2} MiB",
        allocated as f64 / 1048576.0,
        active as f64 / 1048576.0,
        resident as f64 / 1048576.0,
    );

    HttpResponse::Ok().body(msg)
}
