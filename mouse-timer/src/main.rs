use std::sync::atomic::AtomicU32;
use std::time::{SystemTime, UNIX_EPOCH};
use chrono::{DurationRound, NaiveDateTime, NaiveTime, Utc};
use rdev::{Button, Event, EventType, listen};

fn main() {
    if let Err(error) = listen(callback) {
        println!("Error: {:?}", error);
    }
}

fn callback(event: Event) {
    static mut FIRST: Option<NaiveDateTime> = None;
    static mut LAST: Option<NaiveDateTime> = None;
    static mut COUNTER: u32 = 0;

    match event.event_type {
        EventType::ButtonPress(Button::Right) => unsafe {
            let now = Utc::now().naive_utc();

            println!("time.{}: \t{}\tduration: {:.4} ms,\ttotal: {:.4} ms", COUNTER, now,
                     get_duration(LAST.take(), now.clone()),
                     get_duration(FIRST.clone(), now.clone()));
            if FIRST.is_none() {
                FIRST = Some(now.clone());
            }
            COUNTER += 1;
            LAST = Some(now);
        }
        _ => {}
    }

    fn get_duration(start: Option<NaiveDateTime>, end: NaiveDateTime) -> f64 {
        start.map(|start| end.clone() - start)
            .and_then(|duration| duration.num_nanoseconds())
            .map(|duration| duration as f64 / 1_000_000.)
            .unwrap_or(0.)
    }
}
