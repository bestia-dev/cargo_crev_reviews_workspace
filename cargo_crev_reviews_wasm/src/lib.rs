// cargo_crev_reviews_wasm lib.rs

// region: auto_md_to_doc_comments include README.md A //!
//! # cargo_crev_reviews_wasm
//!
//! **Wasm web app that is the frontend of the application cargo_crev_reviews**  
//! ***[repository](https://github.com/LucianoBestia/cargo_crev_reviews_workspace); version: 2021.918.1526  date: 2021-09-18 authors: Luciano Bestia***  
//!
//! [![Lines in Rust code](https://img.shields.io/badge/Lines_in_Rust-1009-green.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in Doc comments](https://img.shields.io/badge/Lines_in_Doc_comments-108-blue.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in Comments](https://img.shields.io/badge/Lines_in_comments-88-purple.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in examples](https://img.shields.io/badge/Lines_in_examples-0-yellow.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//! [![Lines in tests](https://img.shields.io/badge/Lines_in_tests-0-orange.svg)](https://github.com/LucianoBestia/cargo_crev_reviews_workspace/)
//!
//! This is a member of the workspace.
//! It is the wasm frontend for the GUI application.  
//!
// endregion: auto_md_to_doc_comments include README.md A //!

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsValue;

mod common_mod;
mod page_review_mod;
mod pages_mod;
mod utils_mod;
mod web_sys_mod;

use crate::web_sys_mod as w;

#[wasm_bindgen(start)]
/// To start the Wasm application, wasm_bindgen runs this function
pub fn wasm_bindgen_start() -> Result<(), JsValue> {
    // Initialize debugging for when/if something goes wrong.
    console_error_panic_hook::set_once();
    // write the app version just for debug purposes
    w::debug_write(&format!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")));

    crate::page_review_mod::request_review_list("");

    // return
    Ok(())
}
