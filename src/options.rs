//! Options model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    Default,
    Header,
    Sections,
    Segments,
    Symbols,
    Dynamic,
    Relocs,
    Notes,
}

#[derive(Debug, Clone)]
pub struct ReadelfOptions {
    pub mode: Mode,
    pub wide: bool,
}

impl Default for ReadelfOptions {
    fn default() -> Self { Self { mode: Mode::Default, wide: false } }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn opts() { assert_eq!(ReadelfOptions::default().mode, Mode::Default); }
}
