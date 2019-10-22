#![feature(async_await)]

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

extern crate pacman_mirrors_lib;
extern crate reqwest;

use pacman_mirrors_lib::{mirrors::*, PacmanMirrors, config};
use reqwest::Client;

fn main() {
    let pacman_mirrors = PacmanMirrors::default();
    let client = Client::new();
    for host in &config::INET_CONN_CHECK_URLS {
        let _ = client.get(*host)
            .send()
            .map_err(|_| {
                panic!("It failed");
            });
    }
    println!("{:?}", pacman_mirrors);
}
