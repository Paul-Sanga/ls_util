#![allow(dead_code)]

extern crate clap;
use std::path::PathBuf;

use clap::Parser;

#[cfg(test)]
mod tests;

#[derive(Debug, Parser)]
pub struct List {
    #[arg(short, long, value_name = "FILE PATH")]
    dir_path: Option<PathBuf>,
}

#[derive(Debug)]
pub struct ListImpl {
    list_instance: List,
}

impl ListImpl {
    pub fn new(list_instance: List) -> Self {
        ListImpl { list_instance }
    }

    pub fn list_dir_contents(&self) {
        if let Some(path) = &self.list_instance.dir_path {
            if !path.exists() {
                println!("\x1b[31m Path does not exist \x1b[0m")
            } else if !path.is_dir() {
                println!("\x1b[31m Invalid directory path \x1b[0m")
            }

            if path.exists() && path.is_dir() {
                match std::fs::read_dir(path) {
                    Ok(dir) => dir.into_iter().for_each(|el| match el {
                        Ok(entry) => {
                            if let Some(path_str) = entry.file_name().as_os_str().to_str() {
                                println!("\x1b[32m {path_str} \x1b[0m",)
                            }
                        }
                        Err(error) => {
                            println!("\x1b[31m {error} \x1b[0m")
                        }
                    }),
                    Err(error) => {
                        println!("\x1b[31m {error} \x1b[0m")
                    }
                }
            }
        } else {
            println!("\x1b[31m Directory path not specified \x1b[0m")
        }
    }
}
