pub mod function;
use std::collections::HashMap;

use crate::{bi_function, f64_constant, function, std_bi_function, std_function};

pub struct Environment {
    constants: HashMap<String, Constant>
}

pub enum Constant {
    Value(f64),
    Supplier(Box<dyn Fn() -> f64>),
    Function(Box<dyn Fn(f64) -> f64>),
    BiFunction(Box<dyn Fn(f64, f64) -> f64>),
}

impl Constant {
    pub fn call(&self, args: &Vec<f64>) -> f64 {
        match self {
            Self::Value(_) => f64::NAN,
            Self::Supplier(f) => f(),
            Self::Function(f) => f(args[0]),
            Self::BiFunction(f) => f(args[0], args[1]),
        }
    }

    pub fn matches_argument_length(&self, args: &Vec<f64>) -> bool {
        args.len() == self.get_argument_length()
    }

    pub fn get_argument_length(&self) -> usize {
        match self {
            Self::Value(_) => 0,
            Self::Supplier(_) => 0,
            Self::Function(_) => 1,
            Self::BiFunction(_) => 2,
        }
    }

    pub fn get_value(&self) -> f64 {
        match self {
            Self::Value(v) => *v,
            _ => f64::NAN
        }
    }

    pub fn is_constant(&self) -> bool {
        match self {
            Self::Value(_) => true,
            _ => false
        }
    }

    pub fn is_function(&self) -> bool {
        !self.is_constant()
    }
}

impl Environment {
    pub fn exists(&self, name: &String) -> bool {
        self.constants.contains_key(name)
    }

    pub fn get(&self, name: &String) -> f64 {
        if !self.exists(name) {
            println!("Unknown constant {}", name);
            return f64::NAN;
        }

        let constant = self.constants.get(name).expect("Exists!");

        if !constant.is_constant() {
            println!("{} is not a constant", name);
        }

        constant.get_value()
    }

    pub fn call(&self, name: &String, args: Vec<f64>) -> f64 {
        if !self.exists(name) {
            println!("Unknown function {}", name);
            return f64::NAN;
        }

        let constant = self.constants.get(name).expect("Exists!");

        if !constant.is_function() {
            println!("{} is not a function", name);
            return f64::NAN;
        }

        if !constant.matches_argument_length(&args) {
            println!("{} takes {} and not {} arguments", name, constant.get_argument_length(), args.len());
            return f64::NAN;
        }

        constant.call(&args)
    }

}

impl Default for Environment {
    fn default() -> Self {
        let mut constants: HashMap<String, Constant> = HashMap::new();

        f64_constant!(constants, pi, PI);
        f64_constant!(constants, e, E);
        f64_constant!(constants, tau, TAU);

        std_function!(constants, sin);
        std_function!(constants, cos);
        std_function!(constants, tan);
        std_function!(constants, asin);
        std_function!(constants, acos);
        std_function!(constants, atan);
        std_function!(constants, sinh);
        std_function!(constants, cosh);
        std_function!(constants, tanh);
        std_function!(constants, asinh);
        std_function!(constants, acosh);
        std_function!(constants, atanh);

        std_function!(constants, abs);
        std_function!(constants, ceil);
        std_function!(constants, floor);

        std_function!(constants, sqrt);
        std_function!(constants, ln);
        function!(constants, lg, |v| v.log10());

        std_bi_function!(constants, log);
        std_bi_function!(constants, atan2);
        bi_function!(constants, root, |v, b| v.powf(1f64 / b));
        bi_function!(constants, pow, |v, b| v.powf(b));

        Self { constants }
    }
}