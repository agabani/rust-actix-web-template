use actix_web::{web, HttpResponse};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.route("liveness", web::get().to(liveness_get))
        .route("readiness", web::get().to(readiness_get));
}

fn liveness_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}

fn readiness_get() -> HttpResponse {
    HttpResponse::Ok().finish()
}
