/*
    Copyright 2021 Volt Contributors

    Licensed under the Apache License, Version 2.0 (the "License");
    you may not use this file except in compliance with the License.
    You may obtain a copy of the License at

        http://www.apache.org/licenses/LICENSE-2.0

    Unless required by applicable law or agreed to in writing, software
    distributed under the License is distributed on an "AS IS" BASIS,
    WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
    See the License for the specific language governing permissions and
    limitations under the License.
*/

use std::io;

use thiserror::Error;

use crate::classes::package::Package;

#[derive(Error, Debug)]
pub enum GetPackageError {
    #[error("network request failed with registry")]
    Request(chttp::Error),
    #[error("unable to read network response")]
    IO(io::Error),
    #[error("unable to deserialize network response: {0:?}")]
    JSON(serde_json::Error),
}

pub async fn get_package(name: &str) -> Result<Option<Package>, GetPackageError> {
    /*

    */
    let resp = chttp::get_async(format!("http://registry.yarnpkg.com/{}", name))
        .await
        .map_err(GetPackageError::Request)?;

    if resp.status().is_client_error() {
        return Ok(None);
    }

    let mut body = resp.into_body();
    let body_string = body.text().map_err(GetPackageError::IO)?;

    let package: Package = serde_json::from_str(&body_string).map_err(GetPackageError::JSON)?;

    Ok(Some(package))
}
