use bytes::Bytes;
use tokio::time::Instant;

/// Entry is a key-value store.
pub(crate) struct Entry {
    /// Uniquely identifiers this entry.
    id: u64,

    /// Stored data in raw format.
    data: Bytes,

    /// Instant at which the entry expires and should be removed from the database.
    expires_at: Option<Instant>,
}
