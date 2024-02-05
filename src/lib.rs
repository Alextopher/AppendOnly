mod ints;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// Core trait for types that can be stored in an append-only database
///
/// This trait is implemented for some primitive types for convenience.
pub trait AppendOnly<'de>: Default {
    type Transition: Serialize + Deserialize<'de>;

    fn update(s: &mut Self, transition: Self::Transition);
}

/// A database backed by an append-only log
pub struct Database<A, W> {
    state: A,
    writer: W,
}

/// An event in the log. Timestamped and serialized.
#[derive(Serialize, Deserialize)]
struct Event<T> {
    tm: DateTime<Utc>,
    ts: T,
}

impl<'de, A, W> Database<A, W>
where
    A: AppendOnly<'de> + Default,
    W: std::io::Write,
{
    /// Create a new database from a writer and reader
    ///
    /// New events will be appended to the writer, and the reader will be read
    /// to initialize the database state.
    pub fn new(writer: W, reader: impl std::io::Read) -> Self {
        let mut db = Database {
            state: Default::default(),
            writer,
        };
        db.read(reader);
        db
    }

    /// Writes an event to the log
    fn write(&mut self, event: Event<&A::Transition>) -> std::io::Result<()> {
        // Write struct to log, followed by newline
        let mut buf = serde_json::to_vec(&event)?;
        buf.push(b'\n');
        self.writer.write_all(&buf)
    }

    /// Reads a log and applies the events to the database
    fn read(&mut self, reader: impl std::io::Read) {
        let reader =
            serde_json::Deserializer::from_reader(reader).into_iter::<Event<A::Transition>>();
        for event in reader {
            A::update(&mut self.state, event.unwrap().ts);
        }
    }

    /// Apply a state transition to the database
    ///
    /// # Errors
    ///
    /// This function will return an error if the transition cannot be written
    /// to the log.
    pub fn apply(&mut self, transition: A::Transition) -> std::io::Result<()> {
        let event = Event {
            tm: chrono::Utc::now(),
            ts: &transition,
        };
        self.write(event)?;
        A::update(&mut self.state, transition);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
