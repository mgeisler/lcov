pub use self::parse::ParseRecordError;
use std::path::PathBuf;

mod parse;
mod display;
#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Record {
    /// Represents a `TN` record.
    TestName {
        /// test name
        name: String,
    },
    /// Represents a `SF` record.
    SourceFile {
        /// Absolute path to the source file.
        path: PathBuf,
    },

    /// Represents a `FN` record.
    FunctionName {
        /// Function name.
        name: String,
        /// Line number of function start.
        start_line: u32,
    },
    /// Represents a `FNDA` record.
    FunctionData {
        /// Function name.
        name: String,
        /// Execution count.
        count: u64,
    },
    /// Represents a `FNF` record.
    FunctionsFound {
        /// Number of functions found.
        found: u64,
    },
    /// Represents a `FNH` record.
    FunctionsHit {
        /// Number of functions hit.
        hit: u64,
    },

    /// Represents a `BRDA` record.
    ///
    /// `block` and `branch` are gcc internal IDs for the branch.
    BranchData {
        /// Line number.
        line: u32,
        /// Block number.
        block: u32,
        /// Branch number.
        branch: u32,
        /// A number indicating how often that branch was taken.
        taken: Option<u64>,
    },
    /// Represents a `BRF` record.
    BranchesFound {
        /// Number of branches found.
        found: u64,
    },
    /// Represents a `BRH` record.
    BranchesHit {
        /// Number of branches hit.
        hit: u64,
    },

    /// Represents a `DA` record.
    LineData {
        /// Line number.
        line: u32,
        /// Execution count.
        count: u64,
        /// Checksum for each instrumented line.
        checksum: Option<String>,
    },
    /// Represents a `LF` record.
    LinesFound {
        /// Number of instrumented line.
        found: u64,
    },
    /// Represents a `LH` record.
    LinesHit {
        /// Number of lines with a non-zero execution count.
        hit: u64,
    },

    /// Represents a `end_of_record` record.
    EndOfRecord,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum RecordKind {
    TestName,
    SourceFile,
    FunctionName,
    FunctionData,
    FunctionsFound,
    FunctionsHit,
    BranchData,
    BranchesFound,
    BranchesHit,
    LineData,
    LinesFound,
    LinesHit,
    EndOfRecord,
}

macro_rules! kind_impl {
    ($rec:expr; $($kind:ident),*) => {
        match $rec {
            $(&Record::$kind { .. } => RecordKind::$kind),*
        }
    }
}

impl Record {
    pub fn kind(&self) -> RecordKind {
        kind_impl!{
            self;
            TestName, SourceFile,
            FunctionName, FunctionData, FunctionsFound, FunctionsHit,
            BranchData, BranchesFound, BranchesHit,
            LineData, LinesFound, LinesHit,
            EndOfRecord
        }
    }
}

impl RecordKind {
    pub fn as_str(&self) -> &'static str {
        use RecordKind::*;

        match *self {
            TestName => "TN",
            SourceFile => "SF",
            FunctionName => "FN",
            FunctionData => "FNDA",
            FunctionsFound => "FNF",
            FunctionsHit => "FNH",
            BranchData => "BRDA",
            BranchesFound => "BRF",
            BranchesHit => "BRH",
            LineData => "DA",
            LinesFound => "LF",
            LinesHit => "LH",
            EndOfRecord => "end_of_record",
        }
    }
}
