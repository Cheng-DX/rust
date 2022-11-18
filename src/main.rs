mod functional;
mod heap;
mod lifecycle;
mod option;
mod refs;
mod smart_pointer;
mod thread;
mod traits;
mod types;
mod utils;

use functional::{closure, iterator};
use heap::{map, vector};
use lifecycle::{advanced, basic, struct_in};
use option::file;
use refs::{self_refed_struct, weak};
use smart_pointer::{box_pointer, cell, deref, drop, rc_arc};
use thread::use_thread;
use traits::post;
use types::{enum_int, new_type, sized_dst};
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
            new_type::test,
            sized_dst::test,
            enum_int::test,
            box_pointer::test,
            deref::test,
            drop::test,
            rc_arc::test,
            cell::test,
            weak::test,
            self_refed_struct::test,
            use_thread::test,
        ],
        "=",
    );
}
