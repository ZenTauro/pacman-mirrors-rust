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

use std::cmp::Ordering::Equal;
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

use std::process::exit;

#[async_std::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    logger::init();

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
        v.parse::<usize>().expect("Failed to parse number of fasttrack results")
    });
    let timeout = args.value_of("timeout").map(|v| {
        let val: u64 = v.parse().expect("Failed to parse timeout");
        std::time::Duration::from_secs(val)
    });

    let uid = geteuid();

    if uid != 0 {
        error!("This process must be run as root");
        exit(1);
    }

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
    let mut ret = Vec::new();

    while let Ok((val, timestamp)) = received.recv().await {
        match take {
            Some(0) => break,
            Some(_) => take = take.map(|v| v - 1),
            None    => (),
        };

        let col = if timestamp < 2.0 {
            format!("{:.5}", timestamp).green()
        } else {
            format!("{:.5}", timestamp).red()
        };

        eprintln!("    {} - {}", col, val);
        ret.push((timestamp, val));
    }

    ret.sort_by(|(k1, _), (k2, _)| {
        k1.partial_cmp(k2).unwrap_or(Equal)
    });

    for (_, v) in ret {
        res.push(v);
    }

    let file_string =  build_filestring(res);
    let mut mirror_file = OpenOptions::new()
        .write(true)
        .open(config::MIRROR_LIST)
        .await?;

    mirror_file
        .write_all(file_string.as_ref()).await
        .expect("Writing to /etc/pacman.d/mirrorlist shouldn't fail");

    Ok(())
}
