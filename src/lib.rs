#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
#![warn(trivial_casts)]
#![warn(trivial_numeric_casts)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_qualifications)]
#![warn(unused_results)]

#[macro_use]
extern crate failure;

pub use line_filter::{Filter as LineFilter, Range as LineRange};
pub use reader::{Error as ReadError, Reader};
pub use record::{ParseRecordError, Record, RecordKind};
pub use report::{MergeError, Report};

mod line_filter;
mod report;
mod record;
mod reader;
