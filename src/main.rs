// This file is part of pacman-mirrors-rust
//
// pacman-mirrors is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// pacman-mirrors is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with pacman-mirrors.  If not, see <http://www.gnu.org/licenses/>.
//
// Authors: ZenTauro <zentauro@riseup.net>

#[macro_use]
extern crate log;

mod builder;
mod config;
mod functions;
mod mirrors;
mod pacman_mirrors;

use crate::builder::build_initial_list;
use crate::builder::build_mirror_list;
use crate::functions::*;

use clap::{App, Arg};
use colored::*;
use pacman_mirrors::PacmanMirrors;
use pretty_env_logger as logger;

use async_std::io;
use async_std::prelude::*;
use async_std::fs::OpenOptions;

use std::process::{exit, Command};

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    logger::init();

    let uid = geteuid();
    //     : usize = std::str::from_utf8(
    //     Command::new("id")
    //         .arg("-u")
    //         .output()?
    //         .stdout
    //         .as_slice()
    // )?.parse()?;

    if uid != 0 {
        error!("This process must be run as root");
        exit(1);
    }

    let app = App::new("pacman-mirrors")
        .version("0.1.0")
        .author("ZenTauro <zentauro@riseup.net>")
        .about("Creates ranked mirrors lists for pacman")
        .after_help(concat!(
            "Copyright (C) 2018  zentauro\n",
            "This program comes with ABSOLUTELY NO WARRANTY\n",
            "This is free software, and you are welcome to redistribute it\n",
            "under certain conditions;\n\n",
            "See the GNU General Public License v3 for details."
        ))
        .arg(
            Arg::with_name("timeout")
                .short("t")
                .long("timeout")
                .help("Timeout in seconds for each request")
                .takes_value(true)
                .value_name("TIMEOUT")
        )
        .arg(
            Arg::with_name("fasttrack")
                .short("f")
                .long("fasttrack")
                .takes_value(true)
                .value_name("NUMBER")
                .help("Generates a mirror according to speed")
        )
        .arg(
            Arg::with_name("country")
                .short("c")
                .long("country")
                .takes_value(true)
                .value_name("COUNTRIES")
                .help("Generates a mirror list with the mirrors in the specified countries, (comma separated list)")
        )
        .arg(
            Arg::with_name("interactive")
                .short("i")
                .long("interactive")
                .help("Generates personalized mirror list")
        );

    let args = &app.get_matches();
    let mut take = args.value_of("fasttrack").map(|v| {
        v.parse::<usize>().expect("Failed to parse")
    });
    let mut timeout = args.value_of("timeout").map(|v| {
        let val = v.parse::<u64>().expect("Failed to parse");
        std::time::Duration::from_secs(val)
    });

    let _pacman_mirrors = PacmanMirrors::default();
    if is_networkup().await {
        info!("Network connection appears to be up")
    } else {
        error!("Network connection appears to be down");
        return Err(Box::new(io::Error::new(
            io::ErrorKind::Other,
            "Network down",
        )));
    }

    let mirs = mirrors::fetch_mirs().await;
    let received = build_mirror_list(&mirs, timeout).await;
    let mut res = Vec::new();

    while let Some((val, timestamp)) = received.recv().await {
        match take {
            Some(0) => break,
            Some(v) => take = take.map(|v| v - 1),
            None    => (),
        };

        let col = if timestamp < 2.0 {
            format!("{:.5}", timestamp).green()
        } else {
            format!("{:.5}", timestamp).red()
        };

        eprintln!("    {} - {}", col, val);
        res.push(val);
    }

    let file_string =  build_filestring(res);
    let mut mirror_file = OpenOptions::new()
        .write(true)
        .open(config::MIRROR_FILE)
        .await?;

    mirror_file.write_all(file_string.as_ref());

    Ok(())
}
