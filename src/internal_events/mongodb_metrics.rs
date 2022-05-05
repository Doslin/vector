use std::time::Instant;

use super::prelude::{error_stage, error_type};
use metrics::{counter, histogram};
use mongodb::{bson, error::Error as MongoError};
use vector_core::internal_event::InternalEvent;

#[derive(Debug)]
pub struct MongodbMetricsBytesReceived<'a> {
    pub byte_size: usize,
    pub protocol: &'a str,
    pub endpoint: &'a str,
}

impl InternalEvent for MongodbMetricsBytesReceived<'_> {
    fn emit(self) {
        trace!(
            message = "Bytes received.",
            byte_size = %self.byte_size,
            protocol = %self.protocol,
            endpoint = %self.endpoint,
        );
        counter!(
            "component_received_bytes_total", self.byte_size as u64,
            "protocol" => self.protocol.to_owned(),
            "endpoint" => self.endpoint.to_owned(),
        );
    }
}

#[derive(Debug)]
pub struct MongoDbMetricsEventsReceived<'a> {
    pub count: usize,
    pub byte_size: usize,
    pub endpoint: &'a str,
}

impl<'a> InternalEvent for MongoDbMetricsEventsReceived<'a> {
    fn emit(self) {
        trace!(
            message = "Events received.",
            count = self.count,
            byte_size = self.byte_size,
            endpoint = self.endpoint,
        );
        counter!(
            "component_received_events_total", self.count as u64,
            "endpoint" => self.endpoint.to_owned(),
        );
        counter!(
            "component_received_event_bytes_total", self.byte_size as u64,
            "endpoint" => self.endpoint.to_owned(),
        );
        // deprecated
        counter!(
            "events_in_total", self.count as u64,
            "endpoint" => self.endpoint.to_owned(),
        );
    }
}

#[derive(Debug)]
pub struct MongoDbMetricsCollectCompleted {
    pub start: Instant,
    pub end: Instant,
}

impl InternalEvent for MongoDbMetricsCollectCompleted {
    fn emit(self) {
        debug!(message = "Collection completed.");
        counter!("collect_completed_total", 1);
        histogram!("collect_duration_seconds", self.end - self.start);
    }
}

pub struct MongoDbMetricsRequestError<'a> {
    pub error: MongoError,
    pub endpoint: &'a str,
}

impl<'a> InternalEvent for MongoDbMetricsRequestError<'a> {
    fn emit(self) {
        error!(
            message = "MongoDb request error.",
            endpoint = %self.endpoint,
            error = ?self.error,
            error_type = error_type::REQUEST_FAILED,
            stage = error_stage::RECEIVING,
        );
        counter!(
            "component_errors_total", 1,
            "error_type" => error_type::REQUEST_FAILED,
            "stage" => error_stage::RECEIVING,
        );
        // deprecated
        counter!("request_errors_total", 1);
    }
}

pub struct MongoDbMetricsBsonParseError<'a> {
    pub error: bson::de::Error,
    pub endpoint: &'a str,
}

impl<'a> InternalEvent for MongoDbMetricsBsonParseError<'a> {
    fn emit(self) {
        error!(
            message = "BSON document parse error.",
            endpoint = %self.endpoint,
            error = ?self.error,
            error_type = error_type::PARSER_FAILED,
            stage = error_stage::RECEIVING,
        );
        counter!(
            "component_errors_total", 1,
            "error_type" => error_type::PARSER_FAILED,
            "stage" => error_stage::RECEIVING,
            "endpoint" => self.endpoint.to_owned(),
        );
        // deprecated
        counter!("parse_errors_total", 1);
    }
}
