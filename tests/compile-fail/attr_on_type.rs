#![feature(use_extern_macros)]

extern crate auto_impl;
use auto_impl::auto_impl;


#[auto_impl(&, &mut)]
type Baz = String;
