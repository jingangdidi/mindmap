use std::io;
use std::num::ParseIntError;

use thiserror::Error;

/// https://github.com/dtolnay/thiserror
/// https://crates.io/crates/thiserror
/// https://juejin.cn/post/7272005801081126968
/// https://www.shakacode.com/blog/thiserror-anyhow-or-how-i-handle-errors-in-rust-apps/
/// https://rustcc.cn/article?id=1e20f814-c7d5-4aca-bb67-45dcfb65d9f9
#[derive(Debug, Error)]
pub enum MyError {
    // file not exist error
    #[error("Error - {file} does not exist")]
    FileNotExistError{file: String},

    // create_dir_all error
    #[error("Error - fs::create_dir_all {dir_name}: {error}")]
    CreateDirAllError{dir_name: String, error: io::Error},

    // string to int error
    #[error("Error - parse {from} -> {to}: {error}")]
    ParseStringError{from: String, to: String, error: ParseIntError},

    // para error
    #[error("Error - {para}")]
    ParaError{para: String},

    // io::Error
    #[error("I/O error occurred")]
    IoError(#[from] io::Error),
}
