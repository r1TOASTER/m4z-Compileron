use crate::macros::MacroStruct;

pub fn pre_compile(buffer: &mut String) -> (String, Vec<String>) {
    // macro list from parsing the macro lines in the start of the buffer
    let mut macro_list: Vec<MacroStruct> = vec![];
    // include list to return
    let mut include_list: Vec<String> = vec![]; 
    // return buffer after modification
    let mut return_buffer = buffer.to_string();
    // parse the buffer into getting it's macros (at the start)
    let macro_lines: Vec<&str> = buffer.rsplit("\r\n").collect();

    for maybe_macro_line in macro_lines {

        let mut current_macro = MacroStruct { macro_literal: String::new(), macro_replacement: String::new() };
        // get the line splitted by space
        let macro_line: Vec<&str> = maybe_macro_line.rsplit(' ').rev().collect();

        // is a macro line
        if macro_line.get(0).unwrap().eq(&"#macro") {
            // index 0 = keyword "macro"
            let mut index = 1;

            while index < macro_line.len() {

                while index < macro_line.len() && !macro_line.get(index).unwrap().eq(&"with") {
                    current_macro.macro_literal += macro_line.get(index).unwrap();
                    current_macro.macro_literal += " ";
                    index += 1;
                }

                // if last index - no "with" keyword
                if index == macro_line.len() {
                    panic!("'with' keyword missing");
                }

                index += 1;

                // found, now search for the replacement
                while index < macro_line.len() && !macro_line.get(index).unwrap().eq(&"end") {
                    current_macro.macro_replacement += macro_line.get(index).unwrap();
                    current_macro.macro_replacement += " ";
                    index += 1;
                }


                if index == macro_line.len() {
                    panic!("'end' keyword missing");
                }

                // continue this loop
                index += 1;
            }

            // get the entire macro line to remove it from the original buffer
            let mut macro_full_line_to_delete = String::from("#macro ");
            macro_full_line_to_delete += current_macro.get_literal();
            macro_full_line_to_delete += "with ";
            macro_full_line_to_delete += current_macro.get_replacement();
            macro_full_line_to_delete += macro_line.get(macro_line.len() - 1).unwrap();

            // delete every macro line after getting it to the macro list
            return_buffer = return_buffer.replace(&macro_full_line_to_delete, "");
            
            // trim the extra spaces at the end of the macro literal and replacement
            current_macro.macro_literal = current_macro.macro_literal.trim_end().to_string();
            current_macro.macro_replacement = current_macro.macro_replacement.trim_end().to_string();

            // push the current macro to the list
            macro_list.push(current_macro);
        }

        // an include instead
        else if macro_line.get(0).unwrap().eq(&"#get") {
            // add the include name to the vec of includes
            include_list.push(macro_line.get(1).expect("No include found").to_string());
            
            let mut include_full_line_to_delete = String::from("#get ");
            include_full_line_to_delete += macro_line.get(1).unwrap();
            
            // remove the include from the text
            return_buffer = return_buffer.replace(&include_full_line_to_delete, "");
        }

    }

    // replace every macro in the buffer with the macro replacement
    for single_macro in macro_list.into_iter().rev().collect::<Vec<MacroStruct>>() {
        return_buffer = return_buffer.replace(single_macro.get_literal(), single_macro.get_replacement());
    }

    (return_buffer, include_list)
}