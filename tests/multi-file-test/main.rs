use mylib;
mod test_module;

#[test]
fn my_test_multi_file() {
    assert_eq!(mylib::really_complicated_code(8, 2).unwrap(), 10);
    assert_eq!(test_module::show_test_module(), "show_test_module");
}