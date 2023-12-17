#![feature(ascii_char, string_remove_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::comparison_chain)]

pub use core::ops::Range;
pub use itertools::Itertools;
pub use num::integer::lcm;
pub use pathfinding::prelude::*;
pub use rayon::prelude::*;
pub use std::{
    array,
    cmp::Ordering,
    collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque},
    fs,
    mem::take,
    thread,
};
pub use text_io::scan;
pub use vek::*;

pub mod days;
pub mod util;

pub use util::*;
