#![no_main]
#[macro_use] extern crate libfuzzer_sys;
extern crate rustfest;
use rustfest::combine::*;
use std::str;

fuzz_target!(|data: &[u8]| {
    if let Ok(s) = str::from_utf8(data) {
        if let Ok((parsed, _rest)) = rustfest::expr().easy_parse(s) {
            let mut variables = rustfest::HashMap::new();
            rustfest::eval(&parsed, &mut variables);
        }
    }
});
