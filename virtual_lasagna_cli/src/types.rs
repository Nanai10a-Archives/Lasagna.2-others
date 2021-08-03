pub type Args<'a> = smallvec::SmallVec<[&'a str; 16]>;
pub type ExitStatus = Option<i32>;
pub type Date = chrono::prelude::DateTime<chrono::Local>;
