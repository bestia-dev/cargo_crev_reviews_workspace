// cargo_tree_mod.rs

// use crate::auto_generated_mod::cln_methods;
use crate::common_structs_mod::*;
// use crate::crev_mod::*;

// use anyhow::Context;
// use function_name::named;
use std::env;
// use std::str::FromStr;
// use std::time::Duration;
use unwrap::unwrap;

/// cargo_tree
pub fn cargo_tree_project() -> anyhow::Result<CargoTreeListData> {
    //let ns_started = crate::utils_mod::ns_start("cargo_tree_project");

    let vec_publisher = crate::db_sled_mod::db_publisher_mod::list()?;
    let cargo_audit = crate::cargo_audit_mod::run_cargo_audit()?;

    let output = std::process::Command::new("cargo").arg("tree").output().unwrap();
    let output = format!("{} {}", String::from_utf8_lossy(&output.stdout), String::from_utf8_lossy(&output.stderr));

    let mut list_of_cargo_tree = vec![];
    for line in output.lines() {
        if !line.trim().starts_with("warning:") && !line.starts_with("package:") && !line.starts_with("workspace:") && !line.starts_with("Blocking ") {
            //  └── wasm-bindgen v0.2.78 (*)
            let regex = unwrap!(regex::Regex::new(r#"([a-z0-9-_]+) v([0-9]+.[0-9]+.[0-9]+)"#));
            match regex.captures(line) {
                None => list_of_cargo_tree.push(CargoTreeItemData {
                    cargo_tree_line: line.to_string(),
                    ..Default::default()
                }),
                Some(caps) => {
                    let crate_name = caps[1].to_string();
                    let crate_version = caps[2].to_string();
                    let crate_name_version = crate::utils_mod::crate_version_join(&crate_name, &crate_version);
                    // my rating from my review
                    let my_rating =
                        // result, option
                        match crate::db_sled_mod::db_review_mod::read(&crate_name_version) {
                            Err(_err) => None,
                            Ok(my_review_opt) =>{
                                match my_review_opt{
                                    None => None,
                                    Some(my_review) => Some(my_review.rating),
                                }
                            }
                    };
                    // crate description
                    let crate_description =
                        // result, option
                        match crate::db_sled_mod::db_crate_mod::read(&crate_name) {
                            Err(_err) => None,
                            Ok(crate_data_opt) =>{
                                match crate_data_opt{
                                    None => None,
                                    Some(crate_data) => Some(crate_data.description),
                                }
                            }
                        };

                    let (published_by_url,trusted_publisher) =
                    // result, option
                    match crate::db_sled_mod::db_verify_mod::read(&crate_name_version) {
                        Err(_err) => (None,None),
                        Ok(verify_data_opt) =>{
                            match verify_data_opt{
                                None => (None,None),
                                Some(verify_data) => {
                                    let published_by_url = verify_data.published_by_url;
                                    let trusted_publisher = crate::crev_mod::is_trusted_publisher(&vec_publisher, &published_by_url);
                                    (Some(published_by_url), Some(trusted_publisher))
                                }
                            }
                        }
                    };

                    let status =
                    // result, option
                    match crate::db_sled_mod::db_verify_mod::read(&crate_name_version) {
                        Err(_err) => None,
                        Ok(verify_data_opt) =>{
                            match verify_data_opt{
                                None => None,
                                Some(verify_data) => Some(verify_data.status),
                            }
                        }
                    };
                    let audit_id = crate::cargo_audit_mod::get_audit_id_for_crate_version(&cargo_audit, &crate_name, &crate_version);

                    list_of_cargo_tree.push(CargoTreeItemData {
                        cargo_tree_line: line.to_string(),
                        crate_name_version: Some(crate_name_version),
                        my_rating,
                        crate_description,
                        published_by_url,
                        trusted_publisher,
                        status,
                        audit_id,
                    })
                }
            }
        }
    }

    //crate::utils_mod::ns_print_ms("cargo_tree_project", ns_started);

    Ok(CargoTreeListData {
        project_dir: env::current_dir()?.to_string_lossy().to_string(),
        list_of_cargo_tree,
    })
}
