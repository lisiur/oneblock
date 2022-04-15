use std::time::Duration;

pub enum Traffic {
    Green,
    Yellow,
    Red,
}

pub trait LastTime {
    fn get_last_time(&self) -> Duration;
}

impl LastTime for Traffic {
    fn get_last_time(&self) -> Duration {
        match *self {
            Traffic::Green => Duration::new(30, 0),
            Traffic::Yellow => Duration::new(3, 0),
            Traffic::Red => Duration::new(27, 0)
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn traffic_test() {
        let light = Traffic::Green;
        assert_eq!(Duration::new(30, 0), light.get_last_time());
    }
}
