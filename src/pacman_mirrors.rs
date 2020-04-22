// This file is part of pacman-mirrors-rust
//
// pacman-mirrors-rust is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// pacman-mirrors-rust is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with pacman-mirrors-rust.  If not, see <http://www.gnu.org/licenses/>.
//
// Authors: ZenTauro <zentauro@riseup.net>

use crate::mirrors::Mirror;

/// The different modes available to be run
#[derive(Debug)]
pub enum Mode {
    /// Probe the mirrors and order by response time
    Fasttrack,
    /// Add mirrors by country nearest to geoip
    Geoip,
    /// Prompt for mirrors to add
    Interactive,
}

/// The object representing the current state of the application
#[derive(Debug)]
pub struct PacmanMirrors {
    /// Whether we are using a custom mirror list
    pub custom: bool,
    /// Whether it is default
    pub default: bool,
    /// The mode selected to run
    pub mode: Mode,
    /// The timeout in seconds
    pub max_wait_time: Option<u64>,
    /// An array of the mirrors to probe
    pub mirrors: Vec<Mirror>,
    /// A field representing network connectivity
    pub network: bool,
    /// A field representing if there is a display
    pub no_display: bool,
    /// A field representing if there is a mirror list
    pub no_mirrorlist: bool,
    /// Report status
    pub no_status: bool,
    /// Run in quiet mode
    pub quiet: bool,
    /// A vector containing a list of countries to probe
    pub selected_countries: Vec<String>,
    /// A field representing if it is connected to a tty
    pub tty: bool,
}

impl PacmanMirrors {
    /// Returns a `PacmanMirrors` object with the default
    /// settings
    pub fn default() -> PacmanMirrors {
        PacmanMirrors {
            custom: false,
            default: false,
            mode: Mode::Fasttrack,
            max_wait_time: Some(2),
            mirrors: Vec::default(),
            network: true,
            no_display: false,
            no_mirrorlist: false,
            no_status: false,
            quiet: false,
            selected_countries: Vec::default(),
            tty: false,
        }
    }

    ///
    pub fn gen_mirr(&mut self) {
        self.add_mirrors();
    }

    /// Adds a mirror to the mirror list
    fn add_mirrors(&mut self) {
        if self.custom {
            unimplemented!();
        } else {
            unimplemented!();
        }
    }
}
