use std::io;

/// Assists with pretty printing tasks.
pub struct PrettyPrinter {
    column : usize,
    indent : usize,
    indent_stack : Vec<usize>,
    do_indent : bool,
    use_colour : bool,
}

impl Default for PrettyPrinter {
    fn default() -> Self {
        Self {
            column : 0,
            indent : 0,
            indent_stack : Vec::new(),
            do_indent : true,
            use_colour : false,
        }
    }
}

impl PrettyPrinter {
    pub fn new(use_colour : bool) -> Self {
        Self { use_colour, ..Default::default() }
    }

    /// Writes a string to the output stream, sanitising any escape codes.
    pub fn write<W : io::Write>(
        &mut self,
        out : &mut W,
        text : &str,
    ) -> io::Result<()> {
        // TODO :: improve this so it doesn't write character-by-character
        self.ensure_indented(out)?;
        let mut chr_prev = '\0';
        for chr in text.chars() {
            if chr == '\n' && chr_prev == '\r' {
                // skip this line break
            } else if matches!(chr, '\n' | '\r') {
                writeln!(out)?;
                self.column = 0;
                self.do_indent = true;
            } else if matches!(chr, '\'' | '"' | '\\') {
                write!(out, "{}", chr)?;
                self.column += 1;
            } else if chr.is_whitespace() {
                write!(out, " ")?;
                self.column += 1;
            } else {
                self.ensure_indented(out)?;
                for chr_escaped in chr.escape_debug() {
                    write!(out, "{}", chr_escaped)?;
                    self.column += 1;
                }
            }
            chr_prev = chr;
        }
        Ok(())
    }

    /// Sets the current decoration any following text should be written in.
    ///
    /// Does nothing if `use_colour` is false.
    pub fn write_style<W : io::Write, St : Into<Style>>(
        &mut self,
        out : &mut W,
        style : St,
    ) -> io::Result<()> {
        if !self.use_colour {
            return Ok(());
        }
        match style.into() {
            Style { decoration : Some(decoration), fg : Some(fg), bg : Some(bg) }
                => write!(out, "\x1B[{};{};{}m",
                    decoration as usize,
                    fg as usize,
                    bg as usize + 10,
                )?,
            Style { decoration, fg, bg } => {
                write!(out, "\x1B[0")?;
                if let Some(decoration) = decoration {
                    write!(out, ";{}", decoration as usize)?;
                }
                if let Some(fg) = fg {
                    write!(out, ";{}", fg as usize)?;
                }
                if let Some(bg) = bg {
                    write!(out, ";{}", bg as usize + 10)?;
                }
                write!(out, "m")?;
            }
        }
        Ok(())
    }

    fn ensure_indented<W : io::Write>(
        &mut self,
        out : &mut W,
    ) -> io::Result<()> {
        if self.do_indent {
            for _ in 0..self.indent {
                write!(out, " ")?;
            }
            self.column += self.indent;
            self.do_indent = false;
        }
        Ok(())
    }

    /// Pushes the current indentation onto the indentation stack. Affects the
    /// indentation of new lines. Returns the new indentation which was
    /// stashed.
    pub fn indent_stash(&mut self) -> usize {
        self.indent_push(self.column);
        self.indent
    }

    /// Pushes a specific indentation onto the indentation stack.
    pub fn indent_push(&mut self, n : usize) {
        self.indent_stack.push(self.indent);
        self.indent = n;
    }

    /// Pops the current indentation from the indentation stash. No effect if
    /// the stash is empty.
    pub fn indent_pop(&mut self) {
        self.indent = self.indent_stack.pop().unwrap_or_default();
    }
}

/// ANSI Terminal colours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Colour {
    Black = 30,
    Red = 31,
    Green = 32,
    Yellow = 33,
    Blue = 34,
    Magenta = 35,
    Cyan = 36,
    Grey = 37,
    BrightBlack = 90,
    BrightRed = 91,
    BrightGreen = 92,
    BrightYellow = 93,
    BrightBlue = 94,
    BrightMagenta = 95,
    BrightCyan = 96,
    BrightGrey = 97,
}

impl Colour {
    pub fn decorated(self, decoration : Decoration) -> Style {
        Style {
            decoration : Some(decoration),
            fg : Some(self),
            ..Default::default()
        }
    }

    pub fn with_bg(self, colour : Colour) -> Style {
        Style {
            fg : Some(self),
            bg : Some(colour),
            ..Default::default()
        }
    }
}

/// ANSI Terminal styles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decoration {
    Bold = 1,
    Dimmed = 2,
    Italic = 3,
    Underline = 4,
    Blink = 5,
    Reversed = 7,
    Hidden = 8,
    Strikethrough = 9,
}

impl Decoration {
    pub fn with_fg(self, colour : Colour) -> Style {
        Style {
            decoration : Some(self),
            fg : Some(colour),
            ..Default::default()
        }
    }

    pub fn with_bg(self, colour : Colour) -> Style {
        Style {
            decoration : Some(self),
            bg : Some(colour),
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
pub struct Style {
    decoration : Option<Decoration>,
    bg : Option<Colour>,
    fg : Option<Colour>,
}

impl From<Colour> for Style {
    fn from(colour : Colour) -> Style {
        Style { fg : Some(colour), ..Default::default() }
    }
}

impl From<Decoration> for Style {
    fn from(decoration : Decoration) -> Style {
        Style { decoration : Some(decoration), ..Default::default() }
    }
}

impl Style {
    pub fn decorated(mut self, decoration : Decoration) -> Style {
        self.decoration = Some(decoration);
        self
    }

    pub fn with_fg(mut self, colour : Colour) -> Style {
        self.fg = Some(colour);
        self
    }

    pub fn with_bg(mut self, colour : Colour) -> Style {
        self.bg = Some(colour);
        self
    }
}