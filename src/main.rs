mod functional;
mod heap;
mod lifecycle;
mod option;
mod traits;
mod utils;

use functional::{closure, iterator};
use heap::{map, vector};
use lifecycle::{advanced, basic, struct_in};
use option::file;
use traits::post;
use utils::print_utils;

fn main() {
    print_utils::run_tests(
        vec![
            post::test,
            map::test,
            vector::test,
            file::test,
            basic::test,
            struct_in::test,
            advanced::test,
            closure::test,
            iterator::test,
        ],
        "=",
    );
}
