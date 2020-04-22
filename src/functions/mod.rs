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

mod config_fn;
mod http_fn;
mod check_network;
mod mirror_file;

pub use self::config_fn::*;
pub use self::http_fn::*;
pub use self::check_network::*;
pub use self::mirror_file::*;

pub fn geteuid() -> u16 {
    extern { fn geteuid() -> u16; }
    unsafe { geteuid() }
}
