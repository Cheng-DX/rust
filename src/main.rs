mod heap;
mod lifecycle;
mod option;
mod traits;

use heap::{map, vector};
use lifecycle::{advanced, basic, struct_in};
use option::file;
use traits::post;

fn main() {
    run_tests(vec![
        post::test,
        map::test,
        vector::test,
        file::test,
        basic::test,
        struct_in::test,
        advanced::test,
    ]);
}

fn run_tests(fns: Vec<fn() -> ()>) {
    for f in fns {
        println!("{}", "=".repeat(60));
        f();
    }
}
