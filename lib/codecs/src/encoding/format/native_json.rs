use bytes::{BufMut, BytesMut};
use serde::{Deserialize, Serialize};
use tokio_util::codec::Encoder;
use vector_core::{event::Event, schema};

/// Config used to build a `NativeJsonSerializer`.
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct NativeJsonSerializerConfig;

impl NativeJsonSerializerConfig {
    /// Build the `NativeJsonSerializer` from this configuration.
    pub const fn build(&self) -> NativeJsonSerializer {
        NativeJsonSerializer
    }

    /// The schema required by the serializer.
    pub fn schema_requirement(&self) -> schema::Requirement {
        schema::Requirement::empty()
    }
}

/// Serializer that converts an `Event` to bytes using the JSON format.
#[derive(Debug, Clone)]
pub struct NativeJsonSerializer;

impl NativeJsonSerializer {
    /// Encode event as native JSON.
    pub fn encode_json(&self, event: Event) -> Result<serde_json::Value, serde_json::Error> {
        event.try_into()
    }
}

impl Encoder<Event> for NativeJsonSerializer {
    type Error = vector_core::Error;

    fn encode(&mut self, event: Event, buffer: &mut BytesMut) -> Result<(), Self::Error> {
        let writer = buffer.writer();
        serde_json::to_writer(writer, &event).map_err(Into::into)
    }
}
