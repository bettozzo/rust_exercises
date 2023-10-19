#![allow(dead_code, unused_imports)]
pub mod es1;
use crate::es1::main_es1;

pub mod es2;
use crate::es2::main_es2;

pub mod es3;
use crate::es3::*;

pub mod es4;
use crate::es4::*;

pub mod es5;
use crate::es5::main_es5;

pub mod es6;
use crate::es6::main_es6;

pub mod es7;
use crate::es7::main_es7;

pub mod es8;
use crate::es8::main_es8;

fn main() {
    // main_es1();
    // main_es2();
    // main_es5();
    main_es6();
    // main_es7();
    // main_es8();
}
