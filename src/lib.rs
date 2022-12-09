#![feature(string_remove_matches)]
#![allow(clippy::type_complexity)]
#![allow(clippy::needless_range_loop)]
#![allow(clippy::wrong_self_convention)]
#![allow(clippy::comparison_chain)]

pub use itertools::Itertools;
pub use std::{collections::{BTreeMap, BTreeSet, HashMap, HashSet, VecDeque}, fs};
pub use vek::*;

pub mod days;
pub mod util;

pub use util::read_file;
