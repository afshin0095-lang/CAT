use cat_eventbus::{CatEvent, EventEnvelope};
use uuid::Uuid;

use crate::{ContentId, ContentPublished};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicationContext {
    pub producer: String,
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
}

impl PublicationContext {
    pub fn new(producer: impl Into<String>) -> Self {
        Self {
            producer: producer.into(),
            correlation_id: None,
            causation_id: None,
        }
    }

    pub fn with_correlation_id(mut self, id: Uuid) -> Self {
        self.correlation_id = Some(id);
        self
    }

    pub fn with_causation_id(mut self, id: Uuid) -> Self {
        self.causation_id = Some(id);
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublicationEventFactory;

impl PublicationEventFactory {
    pub fn published(
        content_id: ContentId,
        version: u32,
        publication_id: Uuid,
        context: PublicationContext,
    ) -> Result<EventEnvelope, serde_json::Error> {
        let event = ContentPublished {
            content_id,
            version,
            publication_id,
        };

        let mut envelope = EventEnvelope::from_typed(event, context.producer)?
            .with_subject_id(content_id);

        if let Some(id) = context.correlation_id {
            envelope = envelope.with_correlation_id(id);
        }
        if let Some(id) = context.causation_id {
            envelope = envelope.with_causation_id(id);
        }

        Ok(envelope)
    }

    pub fn event_contract() -> (&'static str, u16) {
        (ContentPublished::TYPE, ContentPublished::VERSION)
    }
}
