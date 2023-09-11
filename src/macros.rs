#[derive(Debug)]
pub struct MacroStruct {
    pub MacroLiteral: String,
    pub MacroReplacement: String,
    pub WholeMacro: String,
}

impl MacroStruct {
    pub fn getLiteral(&self) -> &str {
        &self.MacroLiteral.as_str()
    }
    pub fn getReplacement(&self) -> &str {
        &self.MacroReplacement.as_str()
    }
    pub fn getWholeMacro(&self) -> &str {
        &self.WholeMacro.as_str()
    }
}
