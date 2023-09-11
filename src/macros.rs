#[derive(Debug)]
pub struct MacroStruct {
    pub macro_literal: String,
    pub macro_replacement: String,
}

impl MacroStruct {
    pub fn get_literal(&self) -> &str {
        &self.macro_literal.as_str()
    }
    pub fn get_replacement(&self) -> &str {
        &self.macro_replacement.as_str()
    }
}
