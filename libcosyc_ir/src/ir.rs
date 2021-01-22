use libcosyc_diagnostic::source::Span;
use std::fmt;

/// Represents the different kinds of binary operation.
#[derive(Debug)]
pub enum BinaryOpKind {
    Add,
    Subtract
}

/// Represents the different kinds of binary operation.
#[derive(Debug)]
pub enum UnaryOpKind {
    Negate
}

/// Represents the different kinds of value.
#[derive(Debug)]
pub enum ValueKind {
    Integral,
    TypeI8,
    TypeUniverse(usize)
}

impl ValueKind {
    /// Returns whether this value is runtime-known.
    pub fn is_runtime_known(&self) -> bool {
        !matches!(self,
                Self::TypeI8
                | Self::TypeUniverse(..))
    }
}

/// Represents the different kinds of types.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeKind {
    I8,
    TypeUniverse(usize),
    Unknown
}

impl fmt::Display for TypeKind {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::I8 => write!(out, "i8"),
            Self::TypeUniverse(n) => {
                write!(out, "type")?;
                if *n > 0 {
                    write!(out, "#{}", *n + 1)?;
                }
                Ok(())
            },
            Self::Unknown => write!(out, "<unknown>")
        }
    }
}

/// Represents a kind of expression.
#[derive(Debug)]
pub enum InstKind {
    Value(ValueKind),
    TypeAnno {
        value : Box<Inst>,
        ty : Box<Inst>
    },
    BinaryOp {
        kind : BinaryOpKind,
        left : Box<Inst>,
        right : Box<Inst>
    },
    UnaryOp {
        kind : UnaryOpKind,
        value : Box<Inst>
    }
}

/// Represents a node for the typed intermediate representation of a program.
#[derive(Debug)]
pub struct Inst {
    pub span : Span,
    pub datatype : TypeKind,
    pub kind : InstKind
}

impl Inst {
    /// Creates a new typed instruction.
    pub fn new_typed(span : Span, kind : InstKind, datatype : TypeKind) -> Self {
        Self { span, datatype, kind }
    }

    /// Creates a new untyped instruction.
    pub fn new(span : Span, kind : InstKind) -> Self {
        Self::new_typed(span, kind, TypeKind::Unknown)
    }
}

/// Infers the types of trivial values.
pub fn infer_value_type(value : &ValueKind) -> TypeKind {
    match value {
        ValueKind::TypeI8 => TypeKind::TypeUniverse(0),
        ValueKind::TypeUniverse(n) => TypeKind::TypeUniverse(*n + 1),
        _ => TypeKind::Unknown
    }
}

/// Converts a type value into a concrete type.
pub fn value_to_type(value : &ValueKind) -> Option<TypeKind> {
    let ty = match value {
        ValueKind::TypeI8 => TypeKind::I8,
        ValueKind::TypeUniverse(n) => TypeKind::TypeUniverse(*n),
        _ => return None
    };
    Some(ty)
}
