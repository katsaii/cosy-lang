use std::path::Path;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
#[allow(unused_imports)] use inkwell::OptimizationLevel;

use crate::ir::casm;
use crate::error::{ IssueManager, Diagnostic };

/// Generates the LLVM code for this Cosy ASM module, and writes its bitcode to
/// `path`.
pub fn emit_llvm(
    issues : &mut IssueManager,
    bitcode_path : &Path,
    _casm : &casm::Package,
) -> bool {
    let context = Context::create();
    let module = context.create_module("main");
    let mut codegen = CodeGen {
        context : &context,
        module,
        builder : context.create_builder(),
    };
    if codegen.emit_module().is_none() {
        Diagnostic::bug()
            .message("unexpected error encountered when generating LLVM bitcode")
            .report(issues);
    }
    if !codegen.module.write_bitcode_to_path(bitcode_path) {
        Diagnostic::error()
            .message(("failed to write LLVM bitcode to path {}", [
                bitcode_path.display().into()
            ]))
            .report(issues);
    }
    codegen.module.print_to_stderr();
    false
}

struct CodeGen<'a> {
    context : &'a Context,
    module : Module<'a>,
    builder : Builder<'a>,
}

impl<'a> CodeGen<'a> {
    fn emit_module(&mut self) -> Option<()> {
        let i64_type = self.context.i64_type();
        let fn_type = i64_type.fn_type(&[i64_type.into(), i64_type.into(), i64_type.into()], false);
        let function = self.module.add_function("sum", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let x = function.get_nth_param(0)?.into_int_value();
        let y = function.get_nth_param(1)?.into_int_value();
        let z = function.get_nth_param(2)?.into_int_value();

        let sum = self.builder.build_int_add(x, y, "sum").unwrap();
        let sum = self.builder.build_int_add(sum, z, "sum").unwrap();

        self.builder.build_return(Some(&sum)).unwrap();

        let fn_type = i64_type.fn_type(&[], false);
        let function = self.module.add_function("main", fn_type, None);
        let basic_block = self.context.append_basic_block(function, "entry");

        self.builder.position_at_end(basic_block);

        let n = i64_type.const_int(2, false);

        self.builder.build_return(Some(&n)).unwrap();

        Some(())
    }
}