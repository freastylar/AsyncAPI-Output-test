use crate::{config::*, model::*, publish_message, stream_publish_message};
use async_nats::jetstream::Context;
use async_nats::{Client, Message, jetstream};
use log::{debug, warn};
use opentelemetry::global;
use opentelemetry::trace::Tracer;
use std::time;

/// Publish a message in the receive_telemetry channel
/// Channel messages:
///
///     telemetry_message
///

pub async fn producer_telemetry_message(client: &Client, payload: TelemetryMessage) {
    let tracer = global::tracer("telemetry_message_producer");
    let _span = tracer.start("producer_telemetry_message");
    let subject = get_env("receive_telemetry_SUBJECT").unwrap().clone();

    let payload = match serde_json::to_string(&payload) {
        Ok(payload) => payload,
        Err(_) => {
            warn!("Failed to serialize message payload: TelemetryMessage");
            return;
        }
    };
    publish_message(client, &subject, &payload).await;
}
