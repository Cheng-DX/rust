pub mod print_utils {
    pub fn run_tests(fs: Vec<fn() -> ()>, seperator: &str) {
        for f in fs {
            println!("{}", seperator.repeat(60));
            f();
        }
    }
}
