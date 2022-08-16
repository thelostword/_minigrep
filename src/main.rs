/*
 * @Author: thelostword
 * @Date: 2022-08-15 17:00:10
 * @LastEditors: thelostword
 * @LastEditTime: 2022-08-16 16:29:40
 * @FilePath: \minigrep\src\main.rs
 * Copyright (c) 2022 by 东域信息, All Rights Reserved. 
 */
use std::env;
use std::process;
use minigrep::{Config, run};

fn main() {
    // let args: Vec<String> = env::args().collect();

    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments:{}", err);
        process::exit(1);
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {}", e);
        process::exit(1);
    };
}

