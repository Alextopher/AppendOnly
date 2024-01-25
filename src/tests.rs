// Counter backed by append-only log

use std::io::Cursor;

use crate::{ints::IntegerTransitions, Database};

// Database that stores a counter
#[test]
fn test_counter() {
    type Transition = IntegerTransitions<i32>;

    let writer = Cursor::new(vec![]);
    let reader = Cursor::new(vec![]);
    let mut db: Database<i32, _> = Database::new(writer, reader);

    db.apply(Transition::Add(1)).unwrap();
    db.apply(Transition::Add(1)).unwrap();
    db.apply(Transition::Sub(1)).unwrap();

    assert_eq!(db.state, 1);

    // Read the log
    let writer2 = Cursor::new(vec![]);
    let reader2 = Cursor::new(db.writer.into_inner());

    println!("{}", std::io::read_to_string(reader2.clone()).unwrap());

    let db: Database<i32, _> = Database::new(writer2, reader2);

    assert_eq!(db.state, 1);
}
