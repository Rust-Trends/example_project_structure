use mylib::really_complicated_code;

mod additional;
mod combined;

/// Including utils is different because there is no utils file on
/// the same level as this main.rs
#[path = "./utils/utils.rs"]
mod utils;

/// This project shows how to structure a project with multiple modules
/// files and directories.
fn main() {
    println!("Hello, world!");
    additional::hello_from_additional();
    utils::hello_from_utilsl();
    combined::combined();
    combined::mod1::module1();
    combined::mod2::module2();

    // Using the library
    println!("Using the library {:?}", really_complicated_code(1, 2));
}

pub fn add(x: i64, y: i64) -> i64 {
    x + y
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        assert_eq!(add(1, 2), 3);
    }
}
