//! Types describing event relations after MSC 2674, 2675, 2676, 2677.

use std::fmt::Debug;

use js_int::{Int, UInt};
use serde::{Deserialize, Serialize};

/// Summary of all reactions with the given key to an event.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, Eq, Serialize)]
pub struct BundledReaction {
    /// The key (emoji) used for reaction.
    pub key: String,
    /// Time of the bundled reaction being compiled on the server.
    pub origin_server_ts: Option<Int>,
    /// Number of times it was reacted with the key to the event.
    pub count: UInt,
}

/// Type of bundled annotation.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(tag = "type")]
pub enum BundledAnnotation {
    /// An emoji reaction and its count.
    #[serde(rename = "m.reaction")]
    Reaction(BundledReaction),
}

/// The first chunk of annotations with a token for loading more.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct AnnotationChunk {
    /// The first batch of bundled annotations.
    pub chunk: Vec<BundledAnnotation>,
    /// Token to receive the next annotation batch.
    pub next_batch: Option<String>,
}

/// Precompiled list of relations to this event grouped by relation type.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Relations {
    /// Annotation relations.
    #[serde(rename = "m.annotation")]
    pub annotation: Option<AnnotationChunk>,
}
