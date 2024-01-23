#![cfg_attr(windows, feature(abi_vectorcall))]

use ext_php_rs::prelude::*;
use ext_php_rs::types::ZendClassObject;
use std::env;

//pub mod assertions;
//use crate::assertions::Action;
//use c2pa::assertions::Action;


#[php_class]
#[derive(Debug)]
pub struct C2PA {
  pub certificate: String,
  pub key: String
}

#[php_impl]
impl C2PA {

  pub fn __construct() -> Self {
    Self { certificate: String::new(), key: String::new() }
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

  #[setter]
  pub fn set_certificate(&mut self, certificate: String) {
    // validate the certificate

    // set the certificate
    self.certificate = certificate;
  }

  #[setter]
  pub fn set_key(&mut self, key: String) {
    // validate the key

    // set the key
    self.key = key;
  }

  pub fn get_raw_obj(#[this] this: &mut ZendClassObject<C2PA>) {
    dbg!(this);
  }
}





#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
