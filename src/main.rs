#![feature(plugin)]
#![feature(core)]
#![feature(collections)]
#![allow(dead_code)]

#[plugin] #[no_link]
extern crate regex_macros;
extern crate regex;

mod filter;
