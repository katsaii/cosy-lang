pub mod lower;
pub mod emit;

/*
use bincode::{ Encode, Decode };

use crate::{ source::Located, vfs::Manifest };

pub use crate::lower::hir;

#[derive(Debug, Default, Encode, Decode)]
pub struct Package;

/// Pretty prints Cosy ASM for debugging purposes.
pub fn debug_print_casm(_manifest : &Manifest, package : &Package) {
    println!("{:?}", Package);
}
*/