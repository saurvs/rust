// Copyright 2015 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

// Can't use empty braced struct as enum pattern

// aux-build:empty-struct.rs

#![feature(braced_empty_structs)]

extern crate empty_struct;
use empty_struct::*;

enum E {
    Empty3 {}
}

fn main() {
    let e3 = E::Empty3 {};
    let xe3 = XE::XEmpty3 {};

    // Rejected by parser as yet
    // match e3 {
    //     E::Empty3() => () // ERROR `E::Empty3` does not name a tuple variant or a tuple struct
    // }
    // match xe3 {
    //     E::Empty3() => () // ERROR `XE::XEmpty3` does not name a tuple variant or a tuple struct
    // }
    match e3 {
        E::Empty3(..) => () //~ ERROR `E::Empty3` does not name a tuple variant or a tuple struct
    }
    match xe3 {
        XE::XEmpty3(..) => () //~ ERROR `XE::XEmpty3` does not name a tuple variant or a tuple
    }
}
