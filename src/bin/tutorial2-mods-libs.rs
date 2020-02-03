extern crate rust_tutorials;

use rust_tutorials::test_mod::test_funcs;
use rust_tutorials::test_mod2::test_funcs as test_funcs2;

fn main() {
    test_funcs::print_hello();
    test_funcs2::print_hello();
}
