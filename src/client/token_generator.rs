
use crate::{
    client::TokenPrefix,
    error::BadElementError,
    types::OrderToken
};

/// Simple placeholder token generation scheme.
/// Will be made into a trait for future flexibility.
pub struct TokenGenerator {
    prefix: TokenPrefix,
    counter: u64,
    // This is the max value that can be represented by the number 
    // of characters following the prefix. 
    counter_max: u64,
    // This is cached to avoid repeated calculations.
    counter_size: usize
}

impl TokenGenerator {

    /// Create a new TokenGenerator that uses the given prefix.
    /// The shorter the prefix used, the more orders that can be placed 
    /// before the sequence tracker rolls over. (See `TokenPrefix::new`.)
    pub fn new(prefix: &str) -> Result<Self, BadElementError> {

        let prefix = TokenPrefix::new(prefix)?;

        let max_digits = 14 - prefix.len() as u32;
        let counter_max = 10u64.pow(max_digits) - 1;

        Ok(TokenGenerator {
            prefix,
            counter: 0u64,
            counter_size: max_digits as usize,
            counter_max
        })
    }

    /// This is the current sequence count for order tokens.
    pub fn count(&self) -> u64 { self.counter }

    /// This is the current prefix for order tokens.
    pub fn prefix(&self) -> &str { &self.prefix }

    /// Change the order token prefix and update counter ranges.
    /// Automatically rolls the counter over.
    pub fn set_prefix(&mut self, prefix: &str) -> Result<(), BadElementError> {

        let prefix = TokenPrefix::new(prefix)?;

        let max_digits = 14 - prefix.len() as u32;
        let counter_max = 10u64.pow(max_digits) - 1;

        self.rollover();

        self.prefix = prefix;
        self.counter_max = counter_max;
        self.counter_size = max_digits as usize;

        Ok(())
    }

    /// Increment the counter and generate a new token.
    /// Automatically rolls counter over when counter_max is reached.
    pub fn next_order_token(&mut self) -> Result<OrderToken, BadElementError> {

        self.counter += 1;
        if self.counter > self.counter_max { 
            self.rollover()
        }

        let (prefix, counter, size) = (
            &self.prefix, 
            self.counter, 
            self.counter_size
        );

        OrderToken::new(format!("{prefix}{counter:0>size$}"))
    }

    /// Restart the count for a new generation.
    /// WARN: This may lead to repetition of order numbers. 
    /// If you are leaving orders active for a long duration, 
    /// or are placing a high volume of orders,
    /// or there is some other risk for conflict,
    /// it is better to change your prefix using `set_prefix` 
    /// to start a new generation before the counter rolls over.
    pub fn rollover(&mut self) {
        // todo: warn log or other side effects.
        self.counter = 0u64;
    }
}

