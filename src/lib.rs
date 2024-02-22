#![cfg_attr(windows, feature(abi_vectorcall))]
#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]

use ext_php_rs::types::ZendClassObject;
use ext_php_rs::{
    info_table_end, info_table_row, info_table_start, prelude::*, zend::{ModuleEntry},
};
use ext_php_rs::prelude::*;

//use c2pa::{Error, Ingredient, Manifest, ManifestStore, ManifestStoreReport};
use c2pa::{ Manifest };
use std::env;
//use serde::Deserialize;
//use std::path::PathBuf;

//pub mod assertions;
//use crate::assertions::Action;
//use c2pa::assertions::Action;


#[php_class]
#[derive(Debug, Default)]
pub struct C2PA {}

#[php_impl]
impl C2PA {

  pub fn __construct() -> Self {
    Self { }
  }

  /**
   * returnd th version of the
   */
  pub fn php_extension_version() -> String {
    String::from(env!("CARGO_PKG_VERSION"))
  }

  /**
   * returns the version of the Rust SDK library we are using
   */
  pub fn rust_library_version() -> String {
    String::from(c2pa::VERSION)
  }

  pub fn get_raw_obj(#[this] this: &mut ZendClassObject<C2PA>) {
    dbg!(this);
  }
}

#[no_mangle]
pub extern "C" fn php_module_info(_module: *mut ModuleEntry) {
    info_table_start!();
    info_table_row!("C2PA", "enabled");
    info_table_row!("C2PA library version", String::from(env!("CARGO_PKG_VERSION")));
    info_table_row!("C2PA Rust SDK version", String::from(c2pa::VERSION));
    info_table_end!();
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module.info_function(php_module_info)
}
