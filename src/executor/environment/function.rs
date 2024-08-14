#[macro_export]
macro_rules! f64_constant {
    ($vector:expr, $name:ident, $constant:ident) => {
        $vector.insert(stringify!($name).to_string(), Constant::Value(std::f64::consts::$constant));
    };
}

#[macro_export]
macro_rules! std_function {
    ($vector:expr, $function:ident) => {
        $vector.insert(stringify!($function).to_string(), Constant::Function(Box::new(|v| v.$function())));
    };
}

#[macro_export]
macro_rules! function {
    ($vector:expr, $name:ident, $function:expr) => {
        $vector.insert(stringify!($name).to_string(), Constant::Function(Box::new($function)));
    };
}

#[macro_export]
macro_rules! std_bi_function {
    ($vector:expr, $function:ident) => {
        $vector.insert(stringify!($function).to_string(), Constant::BiFunction(Box::new(|v, b| v.$function(b))));
    };
}


#[macro_export]
macro_rules! bi_function {
    ($vector:expr, $name:ident, $function:expr) => {
        $vector.insert(stringify!($name).to_string(), Constant::BiFunction(Box::new($function)));
    };
}
