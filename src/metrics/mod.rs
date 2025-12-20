use prometheus::{Encoder, TextEncoder, Registry, Counter, Histogram, HistogramOpts, Opts};
use actix_web::{web, HttpResponse};
use std::sync::Arc;
use anyhow::Result;

pub struct Metrics {
    registry: Registry,
    pub requests_total: Counter,
    pub request_duration: Histogram,
    pub file_uploads: Counter,
    pub file_downloads: Counter,
}

impl Metrics {
    pub fn new() -> Self {
        let registry = Registry::new();

        let requests_total = Counter::with_opts(
            Opts::new("http_requests_total", "Total number of HTTP requests")
        ).unwrap();
        registry.register(Box::new(requests_total.clone())).unwrap();

        let request_duration = Histogram::with_opts(
            HistogramOpts::new("http_request_duration_seconds", "HTTP request duration in seconds")
        ).unwrap();
        registry.register(Box::new(request_duration.clone())).unwrap();

        let file_uploads = Counter::with_opts(
            Opts::new("file_uploads_total", "Total number of file uploads")
        ).unwrap();
        registry.register(Box::new(file_uploads.clone())).unwrap();

        let file_downloads = Counter::with_opts(
            Opts::new("file_downloads_total", "Total number of file downloads")
        ).unwrap();
        registry.register(Box::new(file_downloads.clone())).unwrap();

        Self {
            registry,
            requests_total,
            request_duration,
            file_uploads,
            file_downloads,
        }
    }

    pub fn gather(&self) -> Result<Vec<u8>> {
        let encoder = TextEncoder::new();
        let metric_families = self.registry.gather();
        let mut buffer = vec![];
        encoder.encode(&metric_families, &mut buffer)?;
        Ok(buffer)
    }
}

pub async fn metrics_handler(metrics: web::Data<Arc<Metrics>>) -> HttpResponse {
    match metrics.gather() {
        Ok(buffer) => HttpResponse::Ok()
            .content_type("text/plain; version=0.0.4")
            .body(buffer),
        Err(e) => HttpResponse::InternalServerError()
            .body(format!("Failed to gather metrics: {}", e)),
    }
}
