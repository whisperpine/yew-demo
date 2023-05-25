pub mod comp;
pub mod route;

#[derive(Clone, PartialEq, Debug)]
pub struct Theme {
    pub foreground: String,
    pub background: String,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            foreground: String::from("foreground"),
            background: String::from("background"),
        }
    }
}
