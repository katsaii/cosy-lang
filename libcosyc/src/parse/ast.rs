use bincode::{ Encode, Decode };

use crate::source::{ Location, Located };
use crate::vfs::Manifest;

/// Declaration visibility level.
#[derive(Debug, Encode, Decode)]
pub enum Visibility {
    Public,
    Internal,
}

/// An owned section of source code, such as a string literal after resolving
/// escape codes.
pub type Symbol = String;

/// All AST nodes available to Cosy.
///
/// Although it's possible to construct them, any malformed ASTs will raise an
/// error during the AST -> HIR lowering step.
#[derive(Debug)]
pub enum Node {
    // expressions
    NumIntegral(Located<u128>),
    NumRational(Located<Symbol>),
    Bool(Located<bool>),
    Id(Located<Symbol>),
    Block(Located<Vec<Node>>),
    Parens(Located<Box<Node>>),
    // statments
    Local {
        name : Located<Symbol>,
        init : Option<Box<Node>>,
    },
    // declarations
    Fn {
        name : Located<Symbol>,
        body : Box<Node>,
    },
    // misc
    Scope {
        vis : Located<Visibility>,
        node : Box<Node>,
    },
}

impl Node {
    /// Returns the "primary" location of this AST node, i.e. the most important
    /// component of the node.
    pub fn primary_location(&self) -> Location {
        match self {
            Node::NumIntegral(n) => n.loc,
            Node::NumRational(sym) => sym.loc,
            Node::Bool(b) => b.loc,
            Node::Id(sym) => sym.loc,
            Node::Block(blk) => blk.loc,
            Node::Parens(node) => node.loc,
            Node::Local { name, .. } => name.loc,
            Node::Fn { name, .. } => name.loc,
            Node::Scope { vis, .. } => vis.loc,
        }
    }
    /// Returns the name of this AST node.
    pub fn name(&self) -> &'static str {
        match self {
            Node::NumIntegral(..) => "num-integral",
            Node::NumRational(..) => "num-rational",
            Node::Bool(..) => "bool",
            Node::Id(..) => "id",
            Node::Block(..) => "block",
            Node::Parens(..) => "parens",
            Node::Local { .. } => "local",
            Node::Fn { .. } => "fn",
            Node::Scope { .. } => "scope",
        }
    }
}

/// Pretty prints an AST for debugging purposes.
pub fn debug_print_ast(manifest : &Manifest, ast : &Node) {
    debug_print_ast_(manifest, ast, 0);
}

fn debug_print_ast_(manifest : &Manifest, node : &Node, indent : usize) {
    print!("{}{}", "  ".repeat(indent), node.name());
    match node {
        Node::NumIntegral(n) => {
            println!(" {:?} <{}>",
                n.value, n.loc.show_path(manifest)
            );
        },
        Node::NumRational(sym) => {
            println!(" {:?} <{}>",
                sym.value, sym.loc.show_path(manifest)
            );
        },
        Node::Bool(b) => {
            println!(" {:?} <{}>",
                b.value, b.loc.show_path(manifest)
            );
        },
        Node::Id(sym) => {
            println!(" {:?} <{}>",
                sym.value, sym.loc.show_path(manifest)
            );
        },
        Node::Block(blk) => {
            println!(" <{}>", blk.loc.show_path(manifest));
            for node in &blk.value {
                debug_print_ast_(manifest, node, indent + 1);
            }
        },
        Node::Parens(node) => {
            println!(" <{}>", node.loc.show_path(manifest));
            debug_print_ast_(manifest, &node.value, indent + 1);
        },
        Node::Local { name, init } => {
            println!(" {:?} <{}>",
                name.value, name.loc.show_path(manifest)
            );
            if let Some(node) = init.as_ref() {
                debug_print_ast_(manifest, &node, indent + 1);
            }
        },
        Node::Fn { name, body } => {
            println!(" {:?} <{}>",
                name.value, name.loc.show_path(manifest)
            );
            debug_print_ast_(manifest, &body, indent + 1);
        },
        Node::Scope { vis, node } => {
            println!(" {:?} <{}>",
                vis.value, vis.loc.show_path(manifest)
            );
            debug_print_ast_(manifest, &node, indent + 1);
        },
    }
}