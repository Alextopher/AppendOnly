mod ints;

use serde::{Deserialize, Serialize};

pub trait AppendOnly<'de>: Default {
    type Transition: Serialize + Deserialize<'de>;

    fn update(s: &mut Self, transition: Self::Transition);
}

/**
 * A database backed by an append-only log
 */
pub struct Database<A, W> {
    state: A,
    writer: W,
}

#[derive(Serialize, Deserialize)]
struct Event<T> {
    tm: chrono::DateTime<chrono::Utc>,
    ts: T,
}

impl<'de, A, W> Database<A, W>
where
    A: AppendOnly<'de> + Default,
    W: std::io::Write,
{
    fn write(&mut self, transition: &A::Transition) -> std::io::Result<()> {
        let event = Event {
            ts: transition,
            tm: chrono::Utc::now(),
        };
        // Write struct to log, followed by newline
        let mut buf = serde_json::to_vec(&event)?;
        buf.push(b'\n');
        self.writer.write_all(&buf)
    }

    fn read(&mut self, reader: impl std::io::Read) {
        let reader =
            serde_json::Deserializer::from_reader(reader).into_iter::<Event<A::Transition>>();
        for event in reader {
            A::update(&mut self.state, event.unwrap().ts);
        }
    }

    pub fn new(writer: W, reader: impl std::io::Read) -> Self {
        let mut db = Database {
            state: Default::default(),
            writer,
        };
        db.read(reader);
        db
    }

    pub fn apply(&mut self, transition: A::Transition) -> std::io::Result<()> {
        self.write(&transition)?;
        A::update(&mut self.state, transition);
        Ok(())
    }
}

#[cfg(test)]
mod tests;
