use std::fmt::Display;



pub struct ResponseMessage {
    pub status: u16,
    pub time_in_ms: u128,
    pub body: String,
}

impl Display for ResponseMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_code = match self.status {
            0..=299 => "32",
            300..=309 => "33",
            400..=499 => "35",
            _ => "31"
        };
        writeln!(f, "\x1b[0;{}mSTATUS CODE: {}\x1b[0m", color_code, self.status)
        .and_then(|_| writeln!(f, "took: {}ms", self.time_in_ms))
        .and_then(|_| writeln!(f, "\n{}", self.body))
    }
}