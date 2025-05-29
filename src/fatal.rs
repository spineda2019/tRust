slint::include_modules!();

use std::{
    error::Error,
    fmt::{Debug, Display},
    ops::Deref,
};

use native_dialog::DialogBuilder;

/// This function will only be run in situations when an unrecoverable UI error
/// occurs. For example, if a config file is missing and attempts to load a
/// new or user-selected file also fail. This function displays a visual error
/// and exits the whole process.
pub fn fatal_panic(error: Box<dyn Error>) -> ! {
    let message: String = format!("Error: {}", error.deref());
    let _ = DialogBuilder::message()
        .set_level(native_dialog::MessageLevel::Error)
        .set_title("Fatal Error")
        .set_text(message)
        .alert()
        .show();
    std::process::exit(1);
}

#[derive(Debug)]
pub struct SourceLocation {
    file: &'static str,
    line: u32,
    column: u32,
}

impl SourceLocation {
    pub fn new(file: &'static str, line: u32, column: u32) -> SourceLocation {
        SourceLocation { file, line, column }
    }
}

impl Display for SourceLocation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "File: {}\nLine: {}\nColumn: {}\n",
            self.file, self.line, self.column
        )
    }
}

#[derive(Debug)]
pub struct TrustError {
    description: String,
    source_location: SourceLocation,
}

impl TrustError {
    pub fn new(description: String, source_location: SourceLocation) -> TrustError {
        TrustError {
            description,
            source_location,
        }
    }
}

impl Display for TrustError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}\nat location\n{}",
            self.description, self.source_location
        )
    }
}

impl Error for TrustError {}
