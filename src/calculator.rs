pub struct Calculator {
    /// Value displayed on screen
    value: i64,
    /// When typing the RHS, stores the LHS value
    held_value: Option<i64>,
    /// If an operator was pressed and the calculator is accepting the RHS,
    /// this field will store which operator is active
    last_op: Option<Operator>,
    /// If the last button pressed was an operator
    op_pressed: bool,
    /// If the last button pressed was equals, this field stores the applied RHS value and
    /// operator. This is used to emulate the behavior of some calculators where pressing the
    /// equals button multiple times repeats the most recent calculation.
    last_eval: Option<(i64, Operator)>,
}

pub enum Digit {
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

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Operator {
    Addition,
    Subtraction,
    Multiplication,
    Division,
}

impl Calculator {
    pub fn new() -> Self {
        Self {
            value: 0,
            held_value: None,
            last_op: None,
            op_pressed: false,
            last_eval: None,
        }
    }

    pub fn get_value(&self) -> String {
        return self.value.to_string();
    }

    pub fn put_digit(&mut self, digit: Digit) {
        self.last_eval = None;
        if self.op_pressed {
            self.held_value = Some(self.value);
            self.value = 0;
            self.op_pressed = false;
        }
        self.value = (self.value * 10) + digit as i64;
    }

    pub fn put_operator(&mut self, op: Operator) {
        self.last_eval = None;
        if self.last_op != None && self.op_pressed == false {
            self.perform_calculation();
        }
        self.last_op = Some(op);
        self.op_pressed = true;
    }

    fn perform_calculation(&mut self) {
        if let (Some(op), Some(held)) = (self.last_op, self.held_value) {
            self.value = match op {
                Operator::Addition => held + self.value,
                Operator::Subtraction => held - self.value,
                Operator::Multiplication => held * self.value,
                Operator::Division => held / self.value,
            };
        }
        self.last_op = None;
        self.held_value = None;
    }

    pub fn evaluate(&mut self) {
        if !self.op_pressed {
            if let Some((value, op)) = self.last_eval {
                self.held_value = Some(self.value);
                self.value = value;
                self.last_op = Some(op);
            } else {
                if let Some(op) = self.last_op {
                    self.last_eval = Some((self.value, op));
                }
            }
            self.perform_calculation();
        }
    }

    pub fn clear_all(&mut self) {
        self.value = 0;
        self.held_value = None;
        self.op_pressed = false;
        self.last_op = None;
        self.last_eval = None;
    }

    pub fn clear_entry(&mut self) {
        if self.last_eval != None {
            self.clear_all();
        } else {
            self.value = 0;
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
        calc.evaluate();
        assert_eq!(calc.get_value(), "13");
    }

    #[test]
    fn subtraction() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D7);
        calc.put_operator(Operator::Subtraction);
        calc.put_digit(Digit::D6);
        calc.evaluate();
        assert_eq!(calc.get_value(), "1");
        calc.evaluate();
        assert_eq!(calc.get_value(), "-5");
        calc.evaluate();
        assert_eq!(calc.get_value(), "-11");
    }

    #[test]
    fn multiplication() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D8);
        calc.put_operator(Operator::Multiplication);
        calc.put_digit(Digit::D2);
        calc.evaluate();
        assert_eq!(calc.get_value(), "16");
        calc.evaluate();
        assert_eq!(calc.get_value(), "32");
        calc.evaluate();
        assert_eq!(calc.get_value(), "64");
    }

    #[test]
    fn division() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D1);
        calc.put_digit(Digit::D2);
        calc.put_digit(Digit::D0);
        calc.put_operator(Operator::Division);
        calc.put_digit(Digit::D2);
        calc.evaluate();
        assert_eq!(calc.get_value(), "60");
        calc.evaluate();
        assert_eq!(calc.get_value(), "30");
        calc.evaluate();
        assert_eq!(calc.get_value(), "15");
    }

    #[test]
    fn integer_division() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D7);
        calc.put_operator(Operator::Division);
        calc.put_digit(Digit::D2);
        calc.evaluate();
        assert_eq!(calc.get_value(), "3");
    }

    #[test]
    fn clear_all() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D1);
        calc.put_operator(Operator::Addition);
        calc.put_digit(Digit::D2);
        calc.clear_all();
        assert_eq!(calc.get_value(), "0");
        assert_eq!(calc.held_value, None);
        assert_eq!(calc.op_pressed, false);
        assert_eq!(calc.last_op, None);
        assert_eq!(calc.last_eval, None);
    }

    #[test]
    fn clear_entry() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D4);
        calc.put_operator(Operator::Multiplication);
        calc.put_digit(Digit::D1);
        calc.put_digit(Digit::D3);
        calc.clear_entry();
        calc.put_digit(Digit::D1);
        calc.put_digit(Digit::D2);
        calc.evaluate();
        assert_eq!(calc.get_value(), "48");
    }

    #[test]
    fn repeated_addition() {
        let mut calc = Calculator::new();
        for _ in 0..5 {
            calc.put_digit(Digit::D1);
            calc.put_operator(Operator::Addition);
        }
        calc.put_digit(Digit::D1);
        calc.evaluate();
        assert_eq!(calc.get_value(), "6");
    }

    #[test]
    fn combined_operations() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D5);
        calc.put_operator(Operator::Addition);
        calc.put_digit(Digit::D5);
        calc.put_operator(Operator::Multiplication);
        assert_eq!(calc.get_value(), "10");
        calc.put_digit(Digit::D6);
        calc.put_operator(Operator::Division);
        assert_eq!(calc.get_value(), "60");
        calc.put_digit(Digit::D1);
        calc.put_digit(Digit::D5);
        calc.put_operator(Operator::Subtraction);
        assert_eq!(calc.get_value(), "4");
        calc.put_digit(Digit::D3);
        calc.evaluate();
        assert_eq!(calc.get_value(), "1");
    }

    #[test]
    fn equals_after_operator() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D5);
        calc.put_operator(Operator::Addition);
        calc.evaluate();
        calc.evaluate();
        assert_eq!(calc.get_value(), "5");
    }

    #[test]
    fn clear_entry_clears_all_after_eval() {
        let mut calc = Calculator::new();
        calc.put_digit(Digit::D1);
        calc.put_operator(Operator::Addition);
        calc.put_digit(Digit::D1);
        calc.evaluate();
        calc.clear_entry();
        assert_eq!(calc.get_value(), "0");
        assert_eq!(calc.held_value, None);
        assert_eq!(calc.op_pressed, false);
        assert_eq!(calc.last_op, None);
        assert_eq!(calc.last_eval, None);
    }
}
