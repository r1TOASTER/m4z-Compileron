use crate::macros::MacroStruct;

pub fn pre_compile(buffer: &mut String) -> String {
    // macro list from parsing the macro lines in the start of the buffer
    let mut macro_list: Vec<MacroStruct> = vec![]; 
    // return buffer after modification
    let mut return_buffer = buffer.to_string();

    // parse the buffer into getting it's macros (at the start)

    // delete every macro after getting it to the macro list

    // replace every macro in the buffer with the macro replacement
    for single_macro in macro_list {
        return_buffer = buffer.replace(single_macro.getLiteral(), single_macro.getReplacement());
    }

    return_buffer
}