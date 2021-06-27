use std::fmt;
use std::path::Path;
use colored::*;

pub trait Prettify {
    type Out: fmt::Display;
    fn pretty(&self) -> Self::Out;
}

impl<P: AsRef<Path>> Prettify for P {
    type Out = ColoredString;
    fn pretty(&self) -> Self::Out {
        self.as_ref().to_string_lossy().bright_blue().bold()
    }
}
