mod gen_llvm;

pub mod lower;

use std::io;
use bincode;

use crate::src::SourceMap;
#[allow(unused_imports)] use crate::pretty::{ PrettyPrinter, Colour, Decoration };

pub use gen_llvm::emit_llvm;

#[derive(Debug, Default, bincode::Encode, bincode::Decode)]
pub struct Package;

/// Pretty prints Cosy ASM for debugging purposes.
pub fn debug_write_casm<W : io::Write>(
    printer : &mut PrettyPrinter<W>,
    _files : &SourceMap,
    package : &Package,
) -> io::Result<()> {
    printer.write(&format!("{:?}", package))?;
    Ok(())
}