use std::env::current_exe;
use std::fs::{create_dir_all, read_to_string};
use std::path::PathBuf;
use std::process::exit;

use argh::FromArgs;
use once_cell::sync::Lazy;
use ron::{
    de::from_str,
    error::{
        SpannedError,
        Error as ron_error,
        Position,
    },
};
use serde::Deserialize;

use crate::error::MyError;

/// global para
pub static PARAS: Lazy<ParsedParas> = Lazy::new(|| {
    match parse_para() {
        Ok(p) => p,
        Err(e) => {
            println!("{}", e);
            exit(1);
        },
    }
});

#[derive(FromArgs)]
#[argh(help_triggers("-h", "--help"))] // https://github.com/google/argh/pull/106
/// mindmap server, based on mind-elixir v5.1.1
struct Paras {
    /// ip address, default: 127.0.0.1
    #[argh(option, short = 'a')]
    addr: Option<String>,

    /// port, default: 8081
    #[argh(option, short = 'p')]
    port: Option<u16>,

    /// language, support: zh_CN, zh_TW, en, ja, pt, ru, default: en
    #[argh(option, short = 'l')]
    language: Option<String>,

    /// output path, default: ./mindmap
    #[argh(option, short = 'o')]
    outpath: Option<String>,

    /// config file, the priority of -a/-p/-l/-o is higher than -c, default: mindmap_config.txt in current path or binary executable file path
    #[argh(option, short = 'c')]
    config: Option<String>,
}

/// mindmap_config.txt
#[derive(Deserialize)]
struct Config {
    addr:     String,
    port:     u16,
    language: String,
    outpath:  String,
}

/// parsed para
///#[derive(Debug, Default)]
pub struct ParsedParas {
    pub addr:      [u8; 4], // addr, default: 127.0.0.1
    pub addr_str:  String,  // addr string, default: "127.0.0.1"
    pub port:      u16,     // port, default: 8081
    pub language:  String,  // language, support: zh_CN, zh_TW, en, ja, pt, ru, default: en
    pub outpath:   PathBuf, // output path, default: ./mindmap
}

/// 解析参数
pub fn parse_para() -> Result<ParsedParas, MyError> {
    let para: Paras = argh::from_env();
    // get config file
    let config: Option<Config> = if para.addr.is_some() && para.port.is_some() && para.language.is_some() && para.outpath.is_some() {
        if para.config.is_some() {
            println!("Warning - the priority of -a/-p/-l/-o is higher than -c, your -c is useless.");
        }
        None
    } else {
        let config_file: Option<PathBuf> = match para.config {
            Some(c) => {
                let tmp_config = PathBuf::from(&c);
                if !(tmp_config.exists() && tmp_config.is_file()) {
                    return Err(MyError::FileNotExistError{file: c})
                }
                Some(tmp_config)
            },
            None => find_config_file(),
        };
        if let Some(f) = config_file {
            match from_str::<Config>(&read_to_string(&f)?) {
                Ok(c) => Some(c),
                Err(e) => if let SpannedError{code: ron_error::Message(m), position: Position{line, col}} = e {
                    return Err(MyError::ParaError{para: format!("{} position: line={}, column={}", m, line, col)})
                } else {
                    return Err(MyError::ParaError{para: format!("parse {} error: {:?}", f.display(), e)})
                },
            }
        } else {
            None
        }
    };
    // parse -a, -p, -o
    let out: ParsedParas = ParsedParas{
        addr: match &para.addr {
            Some(a) => get_addr(a)?,
            None => {
                if let Some(c) = &config {
                    get_addr(&c.addr)?
                } else {
                    [127, 0, 0, 1]
                }
            },
        },
        addr_str: match para.addr {
            Some(a) => a,
            None => {
                if let Some(c) = &config {
                    c.addr.clone()
                } else {
                    "127.0.0.1".to_string()
                }
            },
        },
        port: match para.port {
            Some(p) => p,
            None => {
                if let Some(c) = &config {
                    c.port
                } else {
                    8081
                }
            },
        },
        language: match para.language {
            Some(l) => l,
            None => {
                if let Some(c) = &config {
                    c.language.clone()
                } else {
                    "en".to_string()
                }
            },
        },
        outpath: match para.outpath {
            Some(o) => PathBuf::from(&o),
            None => {
                if let Some(c) = &config {
                    PathBuf::from(&c.outpath)
                } else {
                    PathBuf::from("./mindmap")
                }
            },
        },
    };
    // check language
    if !["zh_CN", "zh_TW", "en", "ja", "pt", "ru"].iter().any(|l| l == &out.language) {
        return Err(MyError::ParaError{para: format!("-l only support zh_CN, zh_TW, en, ja, pt, ru, not {}", out.language)})
    }
    // check outpath exist
    if !(out.outpath.exists() && out.outpath.is_dir()) {
        if let Err(err) = create_dir_all(&out.outpath) {
            return Err(MyError::CreateDirAllError{dir_name: out.outpath.to_str().unwrap().to_string(), error: err})
        }
    }
    Ok(out)
}

/// parse ip addr
fn get_addr(addr: &str) -> Result<[u8;4], MyError> {
    let tmp_addr_vec: Vec<&str> = addr.split(".").collect();
    if tmp_addr_vec.len() == 4 { // `x.x.x.x`, e.g. `127.0.0.1`
        let mut tmp_addr: [u8; 4] = [0; 4];
        for (i, num) in tmp_addr_vec.iter().enumerate() { // convert each number to u8
            match num.parse::<u8>() {
                Ok(n) => tmp_addr[i] = n,
                Err(e) => return Err(MyError::ParseStringError{from: num.to_string(), to: "u8".to_string(), error: e}),
            }
        }
        Ok(tmp_addr)
    } else {
        return Err(MyError::ParaError{para: format!("-a ip address must be x.x.x.x format, not {}", addr)})
    }
}

/// find config file from ./ and binary executable file path
fn find_config_file() -> Option<PathBuf> {
    // get config file from ./
    let tmp_config = PathBuf::from("./mindmap_config.txt");
    if tmp_config.exists() && tmp_config.is_file() {
        Some(tmp_config)
    } else {
        // get config file from path of the current running executable
        if let Ok(mut binary_path) = current_exe() {
            if binary_path.pop() { // Truncates binary_path to parent
                let tmp_config = binary_path.join("mindmap_config.txt");
                if tmp_config.exists() && tmp_config.is_file() {
                    Some(tmp_config)
                } else {
                    None
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}
