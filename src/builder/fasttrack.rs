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

use crate::functions::mirror_response;
use async_std::io;
use async_std::sync::{channel, Receiver};
use async_std::task;
use std::time::Duration;

pub async fn build_mirror_list(
    mirror_list: &Vec<String>,
    timeout: Option<Duration>,
) -> Receiver<(String, f32)> {
    let (s, r) = channel(mirror_list.len());

    for mirror in mirror_list {
        let m = mirror.clone();
        let sc = s.clone();

        info!("Timeout is: {:?}", timeout);

        if let Some(t) = timeout {
            info!("Spawining with timeout {:?}", t);
            task::spawn(io::timeout(t, async move {
                match mirror_response(&m, 10).await {
                    Ok(res) => {
                        sc.send((m.clone(), res)).await;
                    }
                    Err(_) => (),
                }
                Ok(())
            }));
        } else {
            task::spawn(async move {
                match mirror_response(&m, 10).await {
                    Ok(res) => {
                        sc.send((m.clone(), res)).await;
                    }
                    Err(_) => (),
                }
            });
        }
    }

    r
}
