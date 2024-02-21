use std::fmt;

pub struct Wallet {
    amount: i32,
    payer: String,
    payee: String,
}

impl fmt::Display for Wallet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Amount: {}, Payee: {}, Payer: {}",
            self.amount, self.payee, self.payer
        )
    }
}
