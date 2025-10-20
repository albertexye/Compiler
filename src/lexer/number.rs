use super::*;

impl Lexer {
    /// Reads a number token (dispatches to decimal, float, hex, or binary).
    pub(crate) fn read_number(&mut self) -> Result<TokenValue, Error> {
        let ch = *self.peek().unwrap();
        if ch == '0'
            && let Some(&next_ch) = self.input.get(self.index + 1)
        {
            if next_ch == 'x' || next_ch == 'X' {
                return self.read_hexadecimal_number();
            } else if next_ch == 'b' || next_ch == 'B' {
                return self.read_binary_number();
            }
        }
        self.read_decimal_or_float_number()
    }

    fn read_decimal_or_float_number(&mut self) -> Result<TokenValue, Error> {
        let negative = self.consume_negative_sign();
        let number = self.collect_digits()?;
        if self.peek() != Some(&'.') {
            return self.make_integer(number, negative);
        }
        self.advance(); // skip '.'
        let fraction = self.collect_fraction()?;
        let float_value = (number as f64) + fraction;
        let final_value = if negative { -float_value } else { float_value };
        Ok(TokenValue::Literal(Literal::Float(final_value)))
    }

    fn consume_negative_sign(&mut self) -> bool {
        if *self.peek().unwrap() == '-' {
            self.advance();
            true
        } else {
            false
        }
    }

    fn collect_digits(&mut self) -> Result<u64, Error> {
        let mut number = 0u64;
        let mut found = false;
        while let Some(&ch) = self.peek() {
            if !ch.is_ascii_digit() {
                break;
            }
            found = true;
            number = number
                .checked_mul(10)
                .and_then(|n| n.checked_add(ch.to_digit(10).unwrap() as u64))
                .ok_or_else(|| {
                    self.error(
                        ErrorType::InvalidNumber,
                        "Integer overflow in number".to_string(),
                    )
                })?;
            self.advance();
        }
        if !found {
            return Err(self.error(
                ErrorType::InvalidNumber,
                "No digits found in number".to_string(),
            ));
        }
        Ok(number)
    }

    fn collect_fraction(&mut self) -> Result<f64, Error> {
        let mut fraction = 0f64;
        let mut divisor = 10f64;
        let mut found = false;
        while let Some(&ch) = self.peek() {
            if !ch.is_ascii_digit() {
                break;
            }
            found = true;
            fraction += (ch.to_digit(10).unwrap() as f64) / divisor;
            divisor *= 10f64;
            self.advance();
        }
        if !found {
            return Err(self.error(
                ErrorType::InvalidNumber,
                "No digits found after decimal point".to_string(),
            ));
        }
        Ok(fraction)
    }

    fn make_integer(&self, number: u64, negative: bool) -> Result<TokenValue, Error> {
        if negative {
            if number - 1 > i64::MAX as u64 {
                Err(self.error(
                    ErrorType::InvalidNumber,
                    "Integer overflow in negative number".to_string(),
                ))
            } else {
                Ok(TokenValue::Literal(Literal::Int(-(number as i64))))
            }
        } else {
            Ok(TokenValue::Literal(Literal::UInt(number)))
        }
    }

    fn read_hexadecimal_number(&mut self) -> Result<TokenValue, Error> {
        self.advance(); // skip '0'
        self.advance(); // skip 'x' or 'X'
        let mut hex_str = String::new();
        while let Some(&ch) = self.peek() {
            if ch.is_ascii_hexdigit() {
                hex_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if hex_str.is_empty() {
            return Err(self.error(
                ErrorType::InvalidNumber,
                "No digits found in hexadecimal number".to_string(),
            ));
        }
        if let Ok(value) = u64::from_str_radix(&hex_str, 16) {
            Ok(TokenValue::Literal(Literal::UInt(value)))
        } else {
            Err(self.error(
                ErrorType::InvalidNumber,
                format!("Invalid hexadecimal number: 0x{}", hex_str),
            ))
        }
    }

    fn read_binary_number(&mut self) -> Result<TokenValue, Error> {
        self.advance(); // skip '0'
        self.advance(); // skip 'b' or 'B'
        let mut bin_str = String::new();
        while let Some(&ch) = self.peek() {
            if ch == '0' || ch == '1' {
                bin_str.push(ch);
                self.advance();
            } else {
                break;
            }
        }
        if bin_str.is_empty() {
            return Err(self.error(
                ErrorType::InvalidNumber,
                "No digits found in binary number".to_string(),
            ));
        }
        if let Ok(value) = u64::from_str_radix(&bin_str, 2) {
            Ok(TokenValue::Literal(Literal::UInt(value)))
        } else {
            Err(self.error(
                ErrorType::InvalidNumber,
                format!("Invalid binary number: 0b{}", bin_str),
            ))
        }
    }
}
