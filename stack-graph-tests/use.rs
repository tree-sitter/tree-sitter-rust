// Don't crash when imports are present.

use std::fs::Path;

// examples from the Rust reference
use a::b::{c, d, e::f, g::h::i};
use a::b::{self, c, d::e};
use p::q::r as x;
use a::b::{self as ab, c as abc};
use a::b::*;
use a::b::{self as ab, c, d::{*, e::f}};
