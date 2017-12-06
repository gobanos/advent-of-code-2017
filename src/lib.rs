pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;
pub mod day6;

#[macro_export]
macro_rules! load_input {
    ($file:expr) => ({ include_str!(concat!("../../resources/", $file, ".txt"))})
}
