//! This is documentation for the `csv_challenge` lib crate.
//!
//! Usage:
//! ```
//!     use csv_challenge::{
//!         Opt,
//!         {load_csv, write_csv},
//!         replace_column,
//!     };
//!
mod core;
mod err;
mod opt;
pub use self::core::{
    //重新导出
    read::{load_csv, write_csv},
    write::replace_column,
};
pub use self::opt::Opt;
