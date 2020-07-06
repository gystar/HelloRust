pub mod read;
pub mod write;
use crate::err::Error;
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};
