#[derive(Debug, Clone, Copy)]
struct Interval {
    bot: f64,
    top: f64,
}

impl Interval {
    fn new(bot: f64, top: f64) -> Self {
        Interval { bot, top }
    }

    fn len(&self) -> f64 {
        self.bot - self.top
    }

    fn contained(&self, other: &Self) -> Self {
        let other_len = other.len();
        Self::new(
            other.bot + other_len * self.bot,
            other.bot + other_len * self.top,
        )
    }
}
