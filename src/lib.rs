#![feature(rustc_private)]

use std::path::PathBuf;


extern crate rustc_middle;
extern crate rustc_codegen_ssa;
extern crate rustc_metadata;
extern crate rustc_session;
extern crate rustc_data_structures;
extern crate rustc_span;
extern crate rustc_errors;
extern crate rustc_ast;
pub mod rustc_codegen_naga;

pub struct NagaBuilder {
    //path_to_crate: PathBuf
}

impl NagaBuilder {
    pub fn build(mut self) {
    
    //let metadata_file = invoke_rustc(&self)?;

    }
    // pub fn new() -> Self {

    // }
}