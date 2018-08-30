// FIXME: Make me pass! Diff budget: 25 lines.

#[derive(Debug)]
enum Duration {
    MilliSeconds(u64),
    Seconds(u32),
    Minutes(u16)
}

impl Duration {
    pub fn to_ms(&self) -> u64 {
        match *self {
            MilliSeconds(ms) => ms,
            Seconds(s) => s as u64 * 1000,
            Minutes(m) => m as u64 * 1000 * 60,
        }
    }
}

impl PartialEq for Duration {
    fn eq(&self, other: &Duration) -> bool {
        let self_ms = self.to_ms();
        let other_ms = other.to_ms();

        self_ms == other_ms
    }
}


use Duration::{MilliSeconds, Seconds, Minutes};

fn main() {
    assert_eq!(Seconds(120), Minutes(2));
    assert_eq!(Seconds(420), Minutes(7));
    assert_eq!(MilliSeconds(420000), Minutes(7));
    assert_eq!(MilliSeconds(43000), Seconds(43));
}
