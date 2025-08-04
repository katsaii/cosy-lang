pub mod source;
pub mod error;
pub mod parse;
pub mod package;

/// Cosy language file extension.
pub const EXT_SRC : &'static str = "cy";

/// Cosy IR file extension.
pub const EXT_IR : &'static str = "casm";

/// Cosy IDL file extension.
pub const EXT_IDL : &'static str = "cidl";

/// Common info used throughout many parts of the compiler.
#[derive(Default)]
pub struct Session {
    /// A store of all files managed by a compiler session.
    pub files : source::FileManager,
    //pub issues : IssueManager,
}