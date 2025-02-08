pub mod elf;

#[macro_export]
macro_rules! build_main {
    ($input:literal, $( $part:literal => $solver:expr),+) => {
        use std::time::Instant;
        fn main() {
            let input: &str = include_str!(concat!("../input/", $input));
            $(
            let start = Instant::now();
            let result = $solver(input);
            let duration = start.elapsed().as_micros();
            println!("{}: {} (Time: {}μs)", $part, result, duration);
            )+
        }
    };
}

#[macro_export]
macro_rules! build_main_res {
    ($input:literal, $( $part:literal => $solver:expr),+) => {
        use std::time::Instant;
        fn main() {
            let input: &str = include_str!(concat!("../input/", $input));
            $(
            let start = Instant::now();
            let result = $solver(input).unwrap();
            let duration = start.elapsed().as_micros();
            println!("{}: {} (Time: {}μs)", $part, result, duration);
            )+
        }
    };
}
