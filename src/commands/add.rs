use crate::{
    classes::package::{Package, Version},
    utils::download_tarbal,
};
use crate::{model::http_manager, utils::App};
use async_trait::async_trait;
use colored::Colorize;
use sha1;
use std::process;

use crate::__VERSION__;

use super::Command;

pub struct Add;

#[async_trait]
impl Command for Add {
    fn help(&self) -> String {
        format!(
            r#"volt {}
    
    Add a package to your dependencies for your project.
    
    Usage: {} {} {} {}
    
    Options: 
        
      {} {} Output the version number.
      {} {} Output verbose messages on internal operations.
      {} {} Disable progress bar."#,
            __VERSION__.bright_green().bold(),
            "volt".bright_green().bold(),
            "add".bright_purple(),
            "[packages]".white(),
            "[flags]".white(),
            "--version".blue(),
            "(-ver)".yellow(),
            "--verbose".blue(),
            "(-v)".yellow(),
            "--no-progress".blue(),
            "(-np)".yellow()
        )
    }

    async fn exec(&self, app: App, packages: &Vec<String>, _flags: &Vec<String>) {
        for package_name in packages {
            let response = match http_manager::get_package(package_name) {
                Ok(text) => text,
                Err(e) => {
                    eprintln!(
                        "{}: An Error Occured While Requesting {}.json - {}",
                        "error".bright_red().bold(),
                        package_name,
                        e.to_string().bright_yellow()
                    );
                    process::exit(1);
                }
            };

            let package: Package = serde_json::from_str(&response).unwrap();

            // Cache deps
            {
                let db = app.db.lock().await;
                let mut query = db
                    .prepare("INSERT OR IGNORE INTO deps(id) VALUES (?)")
                    .unwrap();
                for (name, version) in &package
                    .versions
                    .get(&package.dist_tags.latest)
                    .unwrap()
                    .dependencies
                {
                    query
                        .execute([&format!(
                            "{}_{}",
                            name.replace("_", "-"),
                            version.replace("^", "")
                        )])
                        .unwrap();
                }
            }

            // println!("package: {:?}", package);

            let version: Version = package
                .versions
                .get_key_value(&package.dist_tags.latest)
                .unwrap()
                .1
                .clone();

            // TODO: Handle Dependencies

            // TODO: Download File
            download_tarbal(package).await;

            // TODO: Verify Checksum
            let dl = sha1::Sha1::from("").digest(); // TODO: Change this to a real checksum

            if dl.to_string() == version.dist.shasum {
                // Verified Checksum
            }
        }
    }
}
