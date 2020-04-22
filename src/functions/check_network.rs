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

use crate::config;
use async_std::task;

pub async fn is_networkup() -> bool {
    let mut tasks = Vec::with_capacity(config::INET_CONN_CHECK_URLS.len());
    for url in &config::INET_CONN_CHECK_URLS {
        let url = (*url).to_string();
        tasks.push(task::spawn(async move {
            surf::get(url).recv_string().await
        }));
    }

    let mut res = Vec::with_capacity(config::INET_CONN_CHECK_URLS.len());

    task::block_on(async {
        for t in tasks {
            match t.await {
                Ok(_) => { res.push(true) },
                Err(_) => { res.push(false) },
            }
        }
    });

    for v in res {
        if v {
            return true;
        }
    };

    false
}
