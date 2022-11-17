pub mod print_utils {
    pub fn run_tests(fs: Vec<fn() -> ()>, seperator: &str) {
        for f in fs {
            println!("{}\n", seperator.repeat(60));
            f();
            println!();
        }
    }
}
