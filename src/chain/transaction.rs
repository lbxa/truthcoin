use chrono::Utc;

use std::fmt;

#[derive(Debug, Clone)]
pub struct Transaction {
    from: String,
    to: String,
    amount: f64,
    timestamp: i64,
}

impl Transaction {
    pub fn new(from: String, to: String, amount: f64) -> Self {
        return Transaction {
            from,
            to,
            amount,
            timestamp: Utc::now().timestamp(),
        };
    }
}

impl fmt::Display for Transaction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Amount: {}, From: {}, To: {}, At: {}",
            self.amount, self.from, self.to, self.timestamp
        )
    }
}
