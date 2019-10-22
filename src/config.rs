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
/// Constant function

// http constants
pub static TEST_FILE: &str = "core.db.tar.gz";
pub static USER_AGENT: &str = "Pacman-Mirrors/";
pub static URL_MIRROR_JSON: &str =
    "https://gitlab.manjaro.org/tools/maintenance-tools/manjaro-web-repo/raw/master/mirrors.json";
pub static URL_STATUS_JSON: &str = "http://repo.manjaro.org/status.json";
pub static INET_CONN_CHECK_URLS: [&str; 3] =
    ["https://gitlab.manjaro.org",
    "https://wikipedia.org",
    "https://bitbucket.org"];

// etc files
pub static CONFIG_FILE: &str = "/etc/pacman-mirrors.conf";
pub static MIRROR_LIST: &str = "/etc/pacman.d/mirrorlist";
// pacman-mirrors dir/files
pub static WORK_DIR: &str = "/var/lib/pacman-mirrors/";
pub static USR_DIR: &str = "/usr/share/pacman-mirrors";
pub static CUSTOM_FILE: &str = "/var/lib/pacman-mirrors/custom-mirrors.json";
pub static MIRROR_FILE: &str = "/usr/share/pacman-mirrors/mirrors.json";
pub static STATUS_FILE: &str = "/var/lib/pacman-mirrors/status.json";
// repo constants
pub static BRANCHES: (&str, &str, &str) = ("stable", "testing", "unstable");
pub static X32_BRANCHES: (&str, &str, &str) = ("x32-stable", "x32-testing", "x32-unstable");
pub static PROTOCOLS: (&str, &str, &str, &str) = ("https", "http", "ftp", "ftps");
pub static METHODS: (&str, &str) = ("rank", "random");
pub static SSL: (&str, &str) = ("True", "False");
pub static REPO_ARCH: &str = "/$repo/$arch";
