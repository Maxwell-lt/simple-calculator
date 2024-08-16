use std::u8;

struct Calculator {
    value: i64,
    held_value: i64,
    last_op: Option<Operator>,
}

enum Digit {
    D0 = 0,
    D1 = 1,
    D2 = 2,
    D3 = 3,
    D4 = 4,
    D5 = 5,
    D6 = 6,
    D7 = 7,
    D8 = 8,
    D9 = 9,
}

#[derive(Clone, Copy)]
enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Calculator {
    fn new() -> Self {
        Self { value: 0, held_value: 0, last_op: None }
    }

    fn get_value(&self) -> String {
        return self.value.to_string()
    }

    fn put_digit(&mut self, digit: Digit) {
        self.value = (self.value * 10) + digit as i64;
    }

    fn put_operator(&mut self, op: Operator) {
        self.held_value = self.value;
        self.value = 0;
        self.last_op = Some(op);
    }

    fn evaluate(&mut self) {
        if let Some(op) = self.last_op {
            match op {
                Operator::Addition => {
                    let last_value = self.value;
                    self.value = self.value + self.held_value;
                    self.held_value = last_value;
                },
                Operator::Subtraction => todo!(),
                Operator::Multiplication => todo!(),
                Operator::Division => todo!(),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_at_zero() {
        let calc = Calculator::new();
        assert_eq!(calc.get_value(), "0");
    }

    #[test]
    fn type_digits() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D2);
        assert_eq!(calc.get_value(), "2");
        calc.put_digit(Digit::D3);
        assert_eq!(calc.get_value(), "23");
    }

    #[test]
    fn addition() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D5);
        calc.put_operator(Operator::Addition);
        calc.put_digit(Digit::D9);
        calc.evaluate();
        assert_eq!(calc.get_value(), "14");
    }

    #[test]
    fn repeat_equals() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D1);
        calc.put_operator(Operator::Addition);
        calc.put_digit(Digit::D4);
        calc.evaluate();
        assert_eq!(calc.get_value(), "5");
        calc.evaluate();
        assert_eq!(calc.get_value(), "9");
    }
}
