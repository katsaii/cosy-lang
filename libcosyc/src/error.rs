use std::{ fmt, io };
use crate::source::Location;

/// Represents a complete message for an error.
pub struct Message {
    /// The format string for this message.
    pub fmt : &'static str,
    /// The arguments that will be inserted into the message template.
    pub args : Vec<TextFragment>,
}

impl Message {
    /// Create a new message from a given format string.
    pub fn new(fmt : &'static str) -> Self {
        Self { fmt, args : Vec::new() }
    }

    /// Consumes this message, returning a new message with the given arg
    /// appended to its `args` field.
    pub fn with_arg<A : Into<TextFragment>>(mut self, arg : A) -> Self {
        let arg = arg.into();
        self.args.push(arg);
        self
    }
}

impl From<&'static str> for Message {
    fn from(fmt : &'static str) -> Self {
        Message::new(fmt)
    }
}

impl<I : IntoIterator<Item=TextFragment>> From<(&'static str, I)> for Message {
    fn from((fmt, args) : (&'static str, I)) -> Self {
        let mut message = Message::new(fmt);
        message.args.extend(args);
        message
    }
}

/// Represents a string or piece of source code which can be 
#[derive(PartialEq, Eq)]
pub enum TextFragment {
    Locale(&'static str),
    Text(String),
    Code(Location),
}

impl From<&'static str> for TextFragment {
    fn from(locale_str : &'static str) -> Self {
        TextFragment::Locale(locale_str)
    }
}

impl From<String> for TextFragment {
    fn from(string : String) -> Self {
        Self::Text(string)
    }
}

impl From<Location> for TextFragment {
    fn from(location : Location) -> Self {
        TextFragment::Code(location)
    }
}

/// Indicates where an error occurred with an optional annotation.
pub struct Label {
    /// The location where the error occurred in the source file.
    pub location : Location,
    /// The captions to use (if any) when displaying the error information.
    pub captions : Vec<Message>,
}

impl From<Location> for Label {
    fn from(location : Location) -> Self {
        Label::new(location)
    }
}

impl Label {
    /// Create an new label from this source location and caption.
    pub fn new(location : Location) -> Self {
        Self { location, captions : Vec::new() }
    }

    /// Consumes this label, returning a new label with the given caption
    /// appended to its `captions` field.
    pub fn with_caption<M : Into<Message>>(mut self, caption : M) -> Self {
        let caption = caption.into();
        self.captions.push(caption);
        self
    }
}

/// Notes additional information as part of a diagnostic.
pub struct Note {
    /// The note caption.
    pub captions : Vec<Message>,
}

impl Note {
    /// Create an empty note.
    pub fn new() -> Self {
        Self { captions : Vec::new() }
    }

    /// Consumes this label, returning a new label with the given caption
    /// appended to its `captions` field.
    pub fn with_caption<M : Into<Message>>(mut self, caption : M) -> Self {
        let caption = caption.into();
        self.captions.push(caption);
        self
    }
}

impl<M : Into<Message>> From<M> for Note {
    fn from(caption : M) -> Self {
        let caption = caption.into();
        Note::new().with_caption(caption)
    }
}

/// Affects the highlighting colour of the error in the output window.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Info,
    Warning,
    Fatal,
    Bug,
}

impl fmt::Display for Severity {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        let kind = match self {
            Self::Info => "info",
            Self::Warning => "warning",
            Self::Fatal => "error",
            Self::Bug => "bug",
        };
        write!(out, "{}", kind)
    }
}

/// Represents all information about an error encountered by Cate.
pub struct Diagnostic {
    /// The severity of this error.
    pub severity : Severity,
    /// A brief description of the error.
    pub messages : Vec<Message>,
    /// Used to discover the line and column numbers of code which directly
    /// caused the error.
    pub primary_labels : Vec<Label>,
    /// Used to discover the line and column numbers of code which may be
    /// relevant to the error. E.g. somewhere else where a variable is used.
    pub secondary_labels : Vec<Label>,
    /// Additional notes about the error, displayed at the bottom of the error
    /// message. These may describe general fixes, or other reasons why an
    /// error occurred (e.g. "known bug" or "unsupported").
    pub notes : Vec<Note>,
}

impl Diagnostic {
    /// Creates an empty diagnostic struct with this severity. Builder pattern
    /// is used to insert information into the error message.
    pub fn new(severity : Severity) -> Self {
        Self {
            severity,
            messages : Vec::new(),
            primary_labels : Vec::new(),
            secondary_labels : Vec::new(),
            notes : Vec::new(),
        }
    }

    /// Creates an empty warning message.
    pub fn warning() -> Self {
        Self::new(Severity::Warning)
    }

    /// Creates an empty error message.
    pub fn error() -> Self {
        Self::new(Severity::Fatal)
    }

    /// Creates an empty bug message.
    pub fn bug() -> Self {
        Self::new(Severity::Bug)
            .note("\
                likely caused by a bug in the compiler\n\
                please report the issue:\n\
                  https://github.com/katsaii/cosy-lang/issues")
    }

    /// Creates an bug message with a note indicating that a feature is
    /// not yet implemented.
    pub fn unimplemented() -> Self {
        Self::bug()
            .message("this feature is unimplemented")
    }

    /// Creates an bug message with a note indicating that unreachable
    /// code was reached when it shouldn't have been.
    pub fn unreachable() -> Self {
        Self::bug()
            .message("encountered an unreachable compiler state")
    }

    /// Inserts a new description for this error message.
    pub fn message<M : Into<Message>>(mut self, message : M) -> Self {
        let message = message.into();
        self.messages.push(message);
        self
    }

    /// Inserts a new primary label into this error message.
    pub fn label<L : Into<Label>>(mut self, label : L) -> Self {
        let label = label.into();
        self.primary_labels.push(label);
        self
    }

    /// Inserts a new secondary label into this error message.
    pub fn label_other<L : Into<Label>>(mut self, label : L) -> Self {
        let label = label.into();
        self.secondary_labels.push(label);
        self
    }

    /// Adds a new note to the end of the error message. Should be used for
    /// additional information that doesn't have an error level, or an
    /// associated source location.
    pub fn note<N : Into<Note>>(mut self, note : N) -> Self {
        let note = note.into();
        self.notes.push(note);
        self
    }

    /// Consumes and reports this error message to the target issue tracker.
    pub fn report(self, issues : &mut IssueManager) {
        issues.errors.push(self);
    }
}

impl From<io::Error> for Diagnostic {
    fn from(err : io::Error) -> Self {
        Diagnostic::error()
            .message(("{}", [err.to_string().into()]))
    }
}

/// Maintains a list of errors that occurred during compilation.
#[derive(Default)]
pub struct IssueManager {
    /// An unordered list of diagnostic info.
    pub errors : Vec<Diagnostic>,
}

impl IssueManager {
    /// Returns whether errors occurred.
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns the statistics for the number of errors that occurred.
    pub fn error_stats(&self) -> Option<IssueStats> {
        if !self.has_errors() {
            return None;
        }
        let mut max_severity = Severity::Warning;
        let mut counts = [0, 0, 0, 0];
        for error in &self.errors {
            let severity = &error.severity;
            match &severity {
                Severity::Info => counts[0] += 1,
                Severity::Warning => counts[1] += 1,
                Severity::Fatal => counts[2] += 1,
                Severity::Bug => counts[3] += 1,
            }
            if *severity > max_severity {
                max_severity = severity.clone();
            }
        }
        Some(IssueStats {
            max_severity,
            infos : counts[0],
            warnings : counts[1],
            errors : counts[2],
            bugs : counts[3],
        })
    }
}

/// Stores error statistics, such as error frequency.
#[derive(Debug)]
pub struct IssueStats {
    /// The max error class reached by the compiler.
    pub max_severity : Severity,
    /// The number of infos encountered.
    pub infos : usize,
    /// The number of warnings encountered.
    pub warnings : usize,
    /// The number of fatal errors encountered.
    pub errors : usize,
    /// The number of compiler bugs encountered.
    pub bugs : usize,
}

impl IssueStats {
    /// Returns the total number of messages that occurred.
    pub fn total(&self) -> usize {
        self.warnings + self.errors + self.bugs
    }
}