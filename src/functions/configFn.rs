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

use config;

enum Method {
    rank,
    random,
}

struct Config {
    branch:           String ,     // "stable",
    branches:         Vec<String>, // conf.BRANCHES,
    config_file:      String,      // conf.CONFIG_FILE,
    country_pool:     Vec<String>, // [],
    custom_file:      String,      // conf.CUSTOM_FILE,
    method:           Method,      // "rank",
    work_dir:         String,      // conf.WORK_DIR,
    mirror_file:      String,      // conf.MIRROR_FILE,
    mirror_list:      String,      // conf.MIRROR_LIST,
    no_update:        bool,        // False,
    protocols:        Vec<String>, // [],
    repo_arch:        String,      // conf.REPO_ARCH,
    ssl_verify:       bool,        // True,
    status_file:      String,      // conf.STATUS_FILE,
    test_file:        String,      // conf.TEST_FILE,
    url_mirrors_json: String,      // conf.URL_MIRROR_JSON,
    url_status_json:  String,      // conf.URL_STATUS_JSON,
    x32:              bool,        // False
}

impl Config {
    pub fn default() -> Config {
        Config {
            branch: "stable".to_owned(),
            branches: ,
        }
    }
}

pub fn setup_config() -> ()
