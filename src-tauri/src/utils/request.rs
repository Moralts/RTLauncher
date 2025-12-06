/*
RTLauncher, a third-party Minecraft launcher built with the newest
technology and provides innovative funtionalities
Copyright (C) 2025 lutouna

This program is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use reqwest;
use std::error::Error;

pub struct Request {
    url: String,
}

impl Request {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub fn get_url(&self) -> &str {
        &self.url
    }

    pub async fn fetch_get(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let client = reqwest::Client::new();
        let response = client.get(&self.url).send().await?;
        let text = response.text().await?;
        Ok(text)
    }
}
