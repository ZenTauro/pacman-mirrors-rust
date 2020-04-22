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

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MirrorJSON {
    pub country: String,
    pub url: String,
    pub protocols: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Protocols {
    pub https: bool,
    pub http: bool,
    pub ftp: bool,
}

impl Protocols {
    pub fn new() -> Protocols {
        Protocols {
            https: false,
            http: false,
            ftp: false,
        }
    }
}

/// Represents the mirror and response time
#[derive(Debug)]
pub struct Mirror {
    /// Self explanatory
    pub country: String,
    /// The url to the mirror
    pub url: String,
    /// An array of the different protocols available to access
    /// the mirror
    pub protocols: Protocols,
    /// The branches available, stable, testing or unstable
    pub branches: Vec<Option<String>>,
    /// Time since last sync
    pub last_sync: Option<String>,
    /// Response time
    pub resp_time: Option<u64>,
}

impl Mirror {
    pub fn new(
        country: String,
        url: String,
        protocols: Protocols,
        branches: Vec<Option<String>>,
        last_sync: Option<String>,
        resp_time: Option<u64>,
    ) -> Mirror {
        Mirror {
            country,
            url,
            protocols,
            branches,
            last_sync,
            resp_time,
        }
    }

    pub fn from_mirror(mir: &MirrorJSON) -> Mirror {
        let mut prots = Protocols::new();

        for prot in &mir.protocols[..] {
            match &prot[..] {
                "https" => prots.https = true,
                "http" => prots.http = true,
                "ftp" => prots.ftp = true,
                protocol => panic!("Unknown protocol {}", protocol),
            }
        }

        Mirror {
            country: mir.country.clone(),
            url: mir.url.clone(),
            protocols: prots,
            branches: Vec::new(),
            last_sync: None,
            resp_time: None,
        }
    }
}

pub async fn fetch_mirrors() -> Vec<Mirror> {
    let mirrors: Vec<MirrorJSON> = serde_json::from_str(
        &surf::get(crate::config::URL_MIRROR_JSON)
            .await
            .unwrap()
            .body_string()
            .await
            .unwrap(),
    )
    .unwrap();

    mirrors.iter().map(|m| Mirror::from_mirror(m)).collect()
}

pub async fn fetch_mirs() -> Vec<String> {
    let mirrors: Vec<MirrorJSON> = serde_json::from_str(
        &surf::get(crate::config::URL_MIRROR_JSON)
            .await
            .unwrap()
            .body_string()
            .await
            .unwrap(),
    )
    .unwrap();

    mirrors
        .iter()
        .filter(|m| {
            let mut has_https = false;

            for prot in &m.protocols[..] {
                match &prot[..] {
                    "https" => {
                        has_https = true;
                        break;
                    }
                    _ => continue,
                }
            }

            has_https
        })
        .map(|m| m.url.clone())
        .collect()
}
