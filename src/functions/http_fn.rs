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

use std::time::{Duration, Instant};
use async_std::io;
use surf::Exception;

/// Fetches the country associated to the connecting IP
/// from `geojs.io`
pub async fn get_ip_country() -> String {
    surf::get("https://get.geojs.io/v1/ip/country/full")
        .recv_string()
        .await
        .unwrap()
}

/// Measures the response time for a given url
pub async fn mirror_response(url: &str, timeout: u64) -> Result<f32, Exception> {
    let t0 = Instant::now();

    io::timeout(Duration::from_secs(timeout), async {
        surf::get(url)
            .recv_string()
            .await
            .map_err(|e| {
                io::Error::new(io::ErrorKind::Other, (*e).to_string())
            })
    }).await?;

    Ok(Instant::now().duration_since(t0).as_secs_f32())
}
