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

/// Represents the mirror and response time
#[derive(Debug)]
pub struct Mirror {
    /// Self explanatory
    pub country: String,
    /// The url to the mirror
    pub url: String,
    /// An array of the different protocols available to access
    /// the mirror
    pub protocols: Vec<String>,
    /// The branches available, stable, testing or unstable
    pub branches: Vec<Option<String>>,
    /// Time since last sync
    pub last_sync: Option<String>,
    /// Response time
    pub resp_time: Option<u64>,
}

impl Mirror {
    fn new(
        country: String,
        url: String,
        protocols: Vec<String>,
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
}
