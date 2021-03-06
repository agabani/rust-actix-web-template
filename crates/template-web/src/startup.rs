use crate::configuration::Configuration;
use crate::routes::health;
use crate::tracing::TraceErrorExt;
use actix_web::dev::Server;
use actix_web::{web, App, HttpServer};
use actix_web_prom::PrometheusMetricsBuilder;

/// Configures the HTTP server and dependencies.
///
/// # Panics
///
/// Will panic if configuration cannot be fully loaded due to missing environment variables.
///
/// Will panic if http server cannot bind socket address.
pub fn run(overrides: &[(&str, &str)]) -> (Server, u16, Configuration) {
    let configuration = Configuration::load(overrides)
        .trace_err()
        .expect("Failed to load configuration");

    // configure http listener
    let listener = configuration
        .http_server
        .tcp_listener()
        .trace_err()
        .expect("Failed to bind port");
    let port = listener.local_addr().unwrap().port();

    // configure prometheus
    let prometheus = PrometheusMetricsBuilder::new("web")
        .endpoint("/metrics")
        .build()
        .expect("Failed to instantiate Prometheus");

    // configure server
    let server = HttpServer::new(move || {
        App::new()
            .wrap(prometheus.clone())
            .service(web::scope("/health").configure(health::config))
    })
    .listen(listener)
    .trace_err()
    .expect("Failed to bind address")
    .run();

    (server, port, configuration)
}
