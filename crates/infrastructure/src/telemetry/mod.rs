use opentelemetry::sdk::trace::{self, Sampler};
use opentelemetry::sdk::Resource;
use opentelemetry::{global, KeyValue};
use opentelemetry_jaeger::new_pipeline;
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::Registry;

pub fn init_telemetry() {
    // Configure the Jaeger exporter
    let tracer = new_pipeline()
        .with_service_name("saas-boilerplate")
        .with_trace_config(
            trace::config()
                .with_sampler(Sampler::AlwaysOn)
                .with_resource(Resource::new(vec![KeyValue::new("service.name", "saas-boilerplate")])),
        )
        .install_batch(opentelemetry::runtime::Tokio)
        .expect("Failed to install OpenTelemetry tracer");

    // Create a tracing layer with the configured tracer
    let telemetry = tracing_opentelemetry::layer().with_tracer(tracer);

    // Use the tracing subscriber `Registry`
    let subscriber = Registry::default().with(telemetry);

    // Set the subscriber as the global default
    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");
}

pub fn shutdown_telemetry() {
    global::shutdown_tracer_provider();
}