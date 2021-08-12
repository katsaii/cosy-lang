use crate::ir;
use libcosyc_diagnostic::{
    error::{ CompilerError, IssueTracker, Failable },
    source::Renderable
};

/// Manages the validation of IR.
pub struct TypeChecker<'a> {
    src : &'a str,
    issues : &'a mut IssueTracker
}

impl Failable for TypeChecker<'_> {
    fn issues(&mut self) -> &mut IssueTracker {
        self.issues
    }
}

impl Renderable for TypeChecker<'_> {
    fn src(&self) -> &str {
        self.src
    }
}

impl<'a> TypeChecker<'a> {
    /// Creates a new instance from this issue tracker and source file.
    pub fn new(src : &'a str, issues : &'a mut IssueTracker) -> Self {
        Self { src, issues }
    }

    /// Asserts whether this instruction has one of the following types.
    pub fn expect_type(&mut self, inst : &ir::Inst, expect : &[ir::TypeKind]) -> Option<()> {
        let span = &inst.span;
        let datatype = &inst.datatype;
        for ty_kind in expect {
            if datatype.kind == *ty_kind {
                return Some(());
            }
        }
        let mut types = String::new();
        let count = expect.len();
        for (i, ty_kind) in expect.iter().enumerate() {
            if i != 0 {
                types.push_str(if i + 1 == count { " or" } else { "," })
            }
            types.push_str(" `");
            types.push_str(&ty_kind.to_string());
            types.push_str("`");
        }
        let mut err = CompilerError::new()
                .span(&span.join(&datatype.span))
                .reason(format!("expected a value of type{} (got `{}`)", types, datatype.kind));
        if matches!(datatype.kind, ir::TypeKind::Unknown) {
            err = err.note("consider adding a type annotation");
        }
        self.report(err)
    }

    /// Asserts whether these two instructions have equivalent types.
    pub fn expect_equal_types(&mut self, a : &ir::Inst, b : &ir::Inst) -> Option<()> {
        let mut ty_a = &a.datatype;
        let mut ty_b = &b.datatype;
        if ty_a.kind == ty_b.kind {
            return Some(());
        }
        if matches!(ty_a.kind, ir::TypeKind::Unknown) {
            let tmp = ty_a;
            ty_a = ty_b;
            ty_b = tmp;
        }
        let mut err = CompilerError::new()
                .span(&b.span)
                .reason(format!("expected a value of type `{}` (got `{}`)", ty_a.kind, ty_b.kind));
        if matches!(ty_a.kind, ir::TypeKind::Unknown) ||
                matches!(ty_b.kind, ir::TypeKind::Unknown) {
            err = err.note("consider adding a type annotation");
        }
        self.report(err)
    }

    /// Performs type checking on this instruction and returns whether it is well-typed.
    pub fn type_check(&mut self, inst : &mut ir::Inst) -> Option<()> {
        let span = &inst.span;
        match &inst.kind {
            ir::InstKind::Variable => self.report(
                    CompilerError::unimplemented("type checking variables").span(&span))?,
            ir::InstKind::Integral { .. } => {
                use ir::TypeKind as TK;
                self.expect_type(inst, &[
                    TK::Int8,
                    TK::Int16,
                    TK::Int32,
                    TK::Int64,
                    TK::UInt8,
                    TK::UInt16,
                    TK::UInt32,
                    TK::UInt64,
                ])?;
            },
            ir::InstKind::FunctionApp { .. } => self.report(
                    CompilerError::unimplemented("type checking function application").span(&span))?,
        }
        Some(())
    }
}

/// Performs type checking on this IR. Returns validated IR.
pub fn check(mut inst : ir::Inst, src : &str, issues : &mut IssueTracker) -> Option<ir::Inst> {
    let mut man = TypeChecker::new(src, issues);
    man.type_check(&mut inst);
    Some(inst)
}
