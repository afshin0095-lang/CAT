#![forbid(unsafe_code)]
#![deny(clippy::all)]

mod error;
mod in_memory;
mod model;
mod ports;

pub use error::{RagError, RagResult};
pub use in_memory::InMemoryIndex;
pub use model::{DocumentChunk, Embedding, RetrievalHit, RetrievalQuery};
pub use ports::{DocumentIndex, Embedder, Retriever};
