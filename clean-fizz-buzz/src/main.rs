// ちょうぜつソフトウェア設計入門

use std::fmt::format;

trait ReplaceRule {
    fn rule_match(&self, carry: &str, n: i32) -> bool;
    fn rule_apply(&self, carry: &str, n: i32) -> String;
}

struct NumberConverter {
    rules: Vec<Box<dyn ReplaceRule>>,
}

impl NumberConverter {
    fn new(rules: Vec<Box<dyn ReplaceRule>>) -> NumberConverter {
        NumberConverter { rules }
    }

    fn convert(&self, n: i32) -> String {
        let mut carry = String::new();
        for rule in &self.rules {
            if rule.rule_match(&carry, n) {
                carry = rule.rule_apply(&carry, n);
            }
        }
        carry
    }
}

struct CyclicNumberRule {
    base: i32,
    replacement: String,
}

impl CyclicNumberRule {
    fn new(base: i32, replacement: String) -> CyclicNumberRule {
        CyclicNumberRule { base, replacement }
    }
}

impl ReplaceRule for CyclicNumberRule {
    fn rule_match(&self, carry: &str, n: i32) -> bool {
        n % self.base == 0
    }

    fn rule_apply(&self, carry: &str, n: i32) -> String {
        format!("{}{}", carry, self.replacement)
    }
}

struct PassThroughRule {}

impl PassThroughRule {
    fn new() -> PassThroughRule {
        PassThroughRule {}
    }
}

impl ReplaceRule for PassThroughRule {
    fn rule_match(&self, carry: &str, n: i32) -> bool {
        carry.is_empty()
    }

    fn rule_apply(&self, carry: &str, n: i32) -> String {
        n.to_string()
    }
}

fn main() {
    let fizzbuzz = NumberConverter::new(vec![
        Box::new(CyclicNumberRule::new(3, "Fizz".to_string())),
        Box::new(CyclicNumberRule::new(5, "Buzz".to_string())),
        Box::new(PassThroughRule::new()),
    ]);

    println!("{}", fizzbuzz.convert(1));
    println!("{}", fizzbuzz.convert(3));
    println!("{}", fizzbuzz.convert(5));
    println!("{}", fizzbuzz.convert(15));
    println!("{}", fizzbuzz.convert(16));
    println!("{}", fizzbuzz.convert(115));
}
