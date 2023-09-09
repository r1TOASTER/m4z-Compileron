use crate::macros::MacroStruct;

pub fn pre_compile(buffer: &mut String) -> String {
    // macro list from parsing the macro lines in the start of the buffer
    let mut macro_list: Vec<MacroStruct> = vec![]; 
    // return buffer after modification
    let mut return_buffer = buffer.to_string();
    // parse the buffer into getting it's macros (at the start)
    let macro_lines: Vec<&str> = buffer.rsplit("\r\n").collect();

    for maybe_macro_line in macro_lines {

        let mut current_macro = MacroStruct { MacroLiteral: String::new(), MacroReplacement: String::new() };
        // get the line splitted by space
        let macro_line: Vec<&str> = maybe_macro_line.rsplit(' ').rev().collect();

        // if the first keyword isn't macro
        if macro_line[0].ne("macro") {
            continue;
        }
        // is a macro line
        else {
            // index 0 = keyword "macro"
            let mut index = 1;

            while index < macro_line.len() {

                while index < macro_line.len() && !macro_line[index].eq("with") {
                    current_macro.MacroLiteral += macro_line[index];
                    current_macro.MacroLiteral += " ";
                    index += 1;
                }

                current_macro.MacroLiteral = current_macro.MacroLiteral.trim_end().to_string();

                // if last index - no "with" keyword
                if index == macro_line.len() {
                    panic!("'with' keyword missing");
                }

                index += 1;

                // found, now search for the replacement
                while index < macro_line.len() && !macro_line[index].eq("end") {
                    current_macro.MacroReplacement += macro_line[index];
                    current_macro.MacroReplacement += " ";
                    index += 1;
                }

                current_macro.MacroReplacement = current_macro.MacroReplacement.trim_end().to_string();

                if index == macro_line.len() {
                    panic!("'end' keyword missing");
                }

                // continue this loop
                index += 1;
            }

            let mut macro_full_line = String::from("macro ");
            macro_full_line += current_macro.getLiteral();
            macro_full_line += current_macro.getReplacement();
            macro_full_line += "end\r\n";
            // delete every macro after getting it to the macro list
            return_buffer = return_buffer.replace(&macro_full_line, "");
            
            dbg!(&return_buffer);
            
            macro_list.push(current_macro);
        }
    }

    // replace every macro in the buffer with the macro replacement
    for single_macro in macro_list {
        return_buffer = return_buffer.replace(single_macro.getLiteral(), single_macro.getReplacement());
    }

    return_buffer
}