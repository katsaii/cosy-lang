pub mod parse;

use std::io;
use bincode::{ Encode, Decode };

use crate::src::{ Location, Located, SourceMap };

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

use crate::pretty::{ PrettyPrinter, Colour, Decoration };

/// Pretty prints an AST for debugging purposes.
pub fn debug_write_ast<W : io::Write>(
    printer : &mut PrettyPrinter<W>,
    files : &SourceMap,
    ast_node : &Node,
) -> io::Result<()> {
    printer.write_style(Decoration::Bold)?;
    printer.write(ast_node.name())?;
    let style_val = Colour::Green;
    let style_loc = Colour::BrightCyan;
    let indent = 2;
    match ast_node {
        Node::NumIntegral(n) => {
            printer.write_style(style_val)?;
            printer.write(&format!(" {:?}", n.value))?;
            printer.write_style(style_loc)?;
            printer.write(&format!(" <{}>", n.loc.show_path(files)))?;
            printer.clear_style()?;
            printer.write("\n")?;
        },
        Node::NumRational(sym) => {
            printer.write_style(style_val)?;
            printer.write(&format!(" {:?}", sym.value))?;
            printer.write_style(style_loc)?;
            printer.write(&format!(" <{}>", sym.loc.show_path(files)))?;
            printer.clear_style()?;
            printer.write("\n")?;
        },
        Node::Bool(b) => {
            printer.write_style(style_val)?;
            printer.write(&format!(" {:?}", b.value))?;
            printer.write_style(style_loc)?;
            printer.write(&format!(" <{}>", b.loc.show_path(files)))?;
            printer.clear_style()?;
            printer.write("\n")?;
        },
        Node::Id(sym) => {
            printer.write_style(style_val)?;
            printer.write(&format!(" {:?}", sym.value))?;
            printer.write_style(style_loc)?;
            printer.write(&format!(" <{}>", sym.loc.show_path(files)))?;
            printer.clear_style()?;
            printer.write("\n")?;
        },
        Node::Block(blk) => {
            printer.write_style(style_loc)?;
            printer.write(&format!(" <{}>", blk.loc.show_path(files)))?;
            printer.clear_style()?;
            printer.write("\n")?;
            printer.indent_push_relative(indent);
            for node in &blk.value {
                debug_write_ast(printer, files, node)?;
            }
            printer.indent_pop();
        },
        Node::Parens(node) => {
            printer.write_style(style_loc)?;
            printer.write(&format!(" <{}>", node.loc.show_path(files)))?;
            printer.clear_style()?;
            printer.write("\n")?;
            printer.indent_push_relative(indent);
            debug_write_ast(printer, files, &node.value)?;
            printer.indent_pop();
        },
        Node::Local { name, init } => {
            printer.write_style(style_val)?;
            printer.write(&format!(" {:?}", name.value))?;
            printer.write_style(style_loc)?;
            printer.write(&format!(" <{}>", name.loc.show_path(files)))?;
            printer.clear_style()?;
            printer.write("\n")?;
            if let Some(node) = init.as_ref() {
                printer.indent_push_relative(indent);
                debug_write_ast(printer, files, &node)?;
                printer.indent_pop();
            }
        },
        Node::Fn { name, body } => {
            printer.write_style(style_val)?;
            printer.write(&format!(" {:?}", name.value))?;
            printer.write_style(style_loc)?;
            printer.write(&format!(" <{}>", name.loc.show_path(files)))?;
            printer.clear_style()?;
            printer.write("\n")?;
            printer.indent_push_relative(indent);
            debug_write_ast(printer, files, &body)?;
            printer.indent_pop();
        },
        Node::Scope { vis, node } => {
            printer.write_style(style_val)?;
            printer.write(&format!(" {:?}", vis.value))?;
            printer.write_style(style_loc)?;
            printer.write(&format!(" <{}>", vis.loc.show_path(files)))?;
            printer.clear_style()?;
            printer.write("\n")?;
            printer.indent_push_relative(indent);
            debug_write_ast(printer, files, &node)?;
            printer.indent_pop();
        },
    }
    Ok(())
}