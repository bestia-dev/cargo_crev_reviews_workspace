// srv_utils_mod.rs

//! rpc methods prepare the data to respond the POST rpc requests

use crate::auto_generated_mod::cln_methods;
use crate::common_structs_mod::*;
use crate::crev_mod::*;

use function_name::named;
use unwrap::unwrap;

#[named]
pub fn srv_cargo_tree_project(_filter_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let response_data = crate::cargo_tree_mod::cargo_tree_project()?;
    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/cargo_tree.html"));

    cln_methods::cln_cargo_tree_list(response_data, &response_html)
}

/// list of all versions for one crate: from registry index with data from src cached and my_reviews
#[named]
pub fn srv_version_list(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    let filter: ReviewFilterData = unwrap!(serde_json::from_value(request_data.clone()));

    let mut vec = crev_crate_versions(&filter.crate_name)?;
    // descending order
    vec.sort_by(|a, b| {
        let a = semver::Version::parse(&a.crate_version).unwrap();
        let b = semver::Version::parse(&b.crate_version).unwrap();
        b.cmp(&a)
    });

    let response_data = VersionListData { list_of_version: vec };
    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/version_list.html"));

    cln_methods::cln_version_list(response_data, &response_html)
}

#[named]
pub fn srv_update_registry_index(_request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());
    match crate::cargo_registry_mod::update_registry_index() {
        Ok(_ret_val) => crate::response_post_mod::response_modal_message("Registry index updated."),
        Err(err) => crate::response_post_mod::response_err_message(&err),
    }
}

#[named]
pub fn srv_config_edit(_request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let response_html = crate::html_mod::process_include(&crate::auto_generated_files_mod::get_file_text("/cargo_crev_reviews/config_edit.html"));
    let response_data = crate::db_sled_mod::db_metadata_mod::get_config()?;

    cln_methods::cln_config_edit(response_data, &response_html)
}

#[named]
pub fn srv_config_save(request_data: serde_json::Value) -> anyhow::Result<String> {
    log::info!(function_name!());

    let config: ConfigData = serde_json::from_value(request_data)?;
    crate::db_sled_mod::db_metadata_mod::set_config(config);

    crate::response_post_mod::response_modal_message("Config saved.")
}
