pub mod cli;
pub mod log;

use std::fmt;

use crate::src::{ Location, Message };
use crate::pretty::Colour;

/// Maintains a list of errors that occurred during compilation.
#[derive(Default)]
pub struct IssueManager {
    /// An unordered list of diagnostic info.
    pub errors : Vec<Diagnostic>,
}

impl IssueManager {
    /// Returns whether any messages occurred, regardless of their error status.
    pub fn has_messages(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Returns whether fatal errors occurred.
    pub fn has_errors(&self) -> bool {
        for error in &self.errors {
            if matches!(error.severity, Severity::Fatal | Severity::Bug) {
                return true;
            }
        }
        false
    }

    /// Returns the statistics for the number of errors that occurred.
    pub fn error_stats(&self) -> Option<IssueStats> {
        if !self.has_messages() {
            return None;
        }
        let mut max_severity = Severity::Warning;
        let mut counts = [0, 0, 0, 0];
        for error in &self.errors {
            let severity = &error.severity;
            match &severity {
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

/// Represents all information about an error encountered by Cate.
#[derive(Default)]
pub struct Diagnostic {
    /// The severity of this error.
    pub severity : Severity,
    /// A brief description of the error.
    pub message : Option<Message>,
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
    pub fn new(severity : Severity) -> Diagnostic {
        let diag = Self { severity, ..Default::default() };
        match severity {
            Severity::Bug => diag.note("\
                likely caused by a bug in the compiler, please report the issue:\n\
                https://github.com/katsaii/cosy-lang/issues"),
            _ => diag,
        }
    }

    /// Creates an empty warning message.
    pub fn warning() -> Diagnostic { Diagnostic::new(Severity::Warning) }

    /// Creates an empty error message.
    pub fn error() -> Diagnostic { Diagnostic::new(Severity::Fatal) }

    /// Creates an empty bug message.
    pub fn bug() -> Diagnostic { Diagnostic::new(Severity::Bug) }

    /// Creates an bug message with a note indicating that a feature is
    /// not yet implemented.
    pub fn unimplemented(feature : &str) -> Diagnostic {
        Self::bug().message(("'{}' is unimplemented", [feature.into()]))
    }

    /// Creates an bug message with a note indicating that unreachable
    /// code was reached when it shouldn't have been.
    pub fn unreachable() -> Diagnostic {
        Self::bug().message("encountered an unreachable compiler state")
    }

    /// Inserts a new description for this error message.
    pub fn message<M : Into<Message>>(mut self, message : M) -> Diagnostic {
        assert!(self.message.is_none(), "cannot have more than one message in diagnostic");
        self.message = Some(message.into());
        self
    }

    /// Inserts a new primary label into this error message.
    pub fn label<L : Into<Label>>(mut self, label : L) -> Diagnostic {
        let label = label.into();
        self.primary_labels.push(label);
        self
    }

    /// Inserts a new secondary label into this error message.
    pub fn label_other<L : Into<Label>>(mut self, label : L) -> Diagnostic {
        let label = label.into();
        self.secondary_labels.push(label);
        self
    }

    /// Adds a new note to the end of the error message. Should be used for
    /// additional information that doesn't have an error level, or an
    /// associated source location.
    pub fn note<N : Into<Note>>(mut self, note : N) -> Diagnostic {
        let note = note.into();
        self.notes.push(note);
        self
    }

    /// Consumes and reports this error message to the target issue manager.
    pub fn report(self, issues : &mut IssueManager) {
        issues.errors.push(self);
    }
}

/// Indicates where an error occurred with an optional annotation.
pub struct Label {
    /// The location where the error occurred in the source file.
    pub location : Location,
    /// The caption to use (if any) when displaying the error information.
    pub caption : Option<Message>,
}

impl From<Location> for Label {
    fn from(location : Location) -> Label {
        Label { location, caption : None }
    }
}

impl<M : Into<Message>> From<(Location, M)> for Label {
    fn from((location, caption) : (Location, M)) -> Label {
        Label { location, caption : Some(caption.into()) }
    }
}

/// Notes additional information as part of a diagnostic.
pub struct Note {
    /// The note caption.
    pub caption : Message,
}

impl<M : Into<Message>> From<M> for Note {
    fn from(caption : M) -> Note {
        Note { caption : caption.into() }
    }
}

/// Affects the highlighting colour of the error in the output window.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Warning,
    Fatal,
    Bug,
}

impl Default for Severity {
    fn default() -> Severity { Self::Fatal }
}

impl fmt::Display for Severity {
    fn fmt(&self, out : &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.as_str())
    }
}

impl Severity {
    /// Returns the string representation of this severity.
    pub fn as_str(&self) -> &'static str {
        match self {
            Severity::Warning => "warning",
            Severity::Fatal => "error",
            Severity::Bug => "bug",
        }
    }

    /// Returns the style of this severity.
    pub fn as_colour(&self) -> Colour {
        match self {
            Severity::Warning => Colour::Yellow,
            Severity::Fatal => Colour::BrightRed,
            Severity::Bug => Colour::Magenta,
        }
    }
}