macro_rules! ntval {
    ($($arg:expr),*) => {{
        let mut inner_data = String::new();
        let _ = String::from(inner_data);
        let mut output_lengths_vector: Vec<usize> = Vec::new();
    // color implementation
    enum TerminalColors {
        Variable,
        Type,
        Value,
        Static
    }
    impl TerminalColors {
        fn as_str(&self) -> &'static str {
            match self {
                TerminalColors::Variable => "#89CFF0", // default: #89CFF0
                TerminalColors::Type => "#c4b2f4", // default: #afd6d2" (light purple #b39cf1)
                TerminalColors::Value => "#F1EB9C",
                TerminalColors::Static => "#dba4a4" // #c8a4c6 dba4a4
            }
        }
    }
    // triadic: (9cdaf1, b39cf1, f1b39c, daf19c)
    trait TColor {
        fn color_str_from_hex<S: AsRef<str>>(self, hex:S, bold:bool) -> String;
    }
    impl TColor for String {
        fn color_str_from_hex<S: AsRef<str>>(self, hex:S, bold:bool) -> String {
            let hex: String = String::from(hex.as_ref());
            let rgb = rgb_from_hex_str(hex);
            if bold {
                format!("\x1b[1m\x1b[38;2;{};{};{}m{}\x1b[0m",rgb.0,rgb.1,rgb.2,self)
            } else {
                format!("\x1b[38;2;{};{};{}m{}\x1b[0m",rgb.0,rgb.1,rgb.2,self)
            }
        }
    }

    impl TColor for &str {
        fn color_str_from_hex<S: AsRef<str>>(self, hex:S, bold:bool) -> String {
            let hex: String = String::from(hex.as_ref());
            let rgb = rgb_from_hex_str(hex);
            if bold {
                format!("\x1b[1m\x1b[38;2;{};{};{}m{}\x1b[0m",rgb.0,rgb.1,rgb.2,self)
            } else {
                format!("\x1b[38;2;{};{};{}m{}\x1b[0m",rgb.0,rgb.1,rgb.2,self)
            }
        }
    }
    // \x1b
    fn rgb_from_hex_str<S: AsRef<str>>(strlike:S) -> (u16,u16,u16) {
        // \x1b[38;2;<r>;<g>;<b>m // start
        // \x1b[0m // stop
        let arg: String = String::from(strlike.as_ref()).trim().replace("#", "");
        let mut arg_chars = arg.chars();
        let mut r_color: String = String::from(arg_chars.nth(0).unwrap());
        r_color.push(arg_chars.nth(0).unwrap());
        let mut g_color: String = String::from(arg_chars.nth(0).unwrap());
        g_color.push(arg_chars.nth(0).unwrap());
        let mut b_color: String = String::from(arg_chars.nth(0).unwrap());
        b_color.push(arg_chars.nth(0).unwrap());
        let rgb: (u16,u16,u16) = (
        u16::from_str_radix(&r_color, 16).unwrap()
        ,u16::from_str_radix(&g_color, 16).unwrap()
        ,u16::from_str_radix(&b_color, 16).unwrap());
        rgb
        }

        fn is_valid_variable(input: &str) -> bool {
            for letter in input.chars() {
                if letter == '"' || letter == '\''{
                    return false
                }
            } return true
        }
        fn find_index_quote_chars(raw_input: &str) -> String {
            let mut bm_indexes: [usize;3] = [0,0,0]; 
            let mut max_quotes: usize = 0;
            let mut clean_output: String = String::from("");
            
            for (i,c) in raw_input.chars().enumerate() {
                if (c == '"' && bm_indexes[2] == 0) || (c == '\'' && bm_indexes[2] == 0) {
                    bm_indexes[2] = 1;
                    bm_indexes[0] = i;
                    max_quotes += 1;
                }
                else if (c== '"' && bm_indexes[2] != 0) || (c== '\'' && bm_indexes[2] != 0) {
                    max_quotes += 1;
                    bm_indexes[1] = i;
                }
            } bm_indexes[2] = max_quotes;
            
            for (i,c) in raw_input.chars().enumerate() {
                if i >= bm_indexes[0] && i <= bm_indexes[1] {
                    clean_output.push(c);
                }
            } 
            clean_output
        }
        fn return_type_of<T>(_: &T) -> String {
            let raw_type: String = format!("{}", std::any::type_name::<T>());
            let simple_type = raw_type
            .replace("core::option::", "")
            .replace("core::result::", "")
            .replace("alloc::string::", "")
            .replace("alloc::boxed::", "")
            .replace("alloc::vec::", "")
            .replace("core::slice::iter::", "");
            simple_type
        }

        fn horizontal_border_draw_with_header(is_top: bool, distance: usize) -> String {
            let padded_distance = distance + 2; 
            if is_top {
                let mut border: String = String::from("┌───┬");
                for _ in 0..padded_distance {
                    border.push('─')
                }
                border.push('┐');
                border
            } else {
                let mut border: String = String::from("└───┴");
                for _ in 0..padded_distance {
                    border.push('─')
                }
                border.push('┘');
                border
            }
        }
        fn row_separator_draw_with_header(distance: usize) -> String {
                let mut border: String = String::from("├───┼");
                for _ in 0..distance + 2 {
                    border.push('─')
                }
                border.push('┤');
                border
            }
        fn find_and_count_index_chars(raw_input: &str) -> (String, usize) {
            let mut bm_indexes: [usize;3] = [0,0,0]; 
            let mut max_quotes: usize = 0;
            let mut clean_output: String = String::from("");
            let mut clean_output_length: usize = 0;
            for (i,c) in raw_input.chars().enumerate() {
                if (c == '"' && bm_indexes[2] == 0) || (c == '\'' && bm_indexes[2] == 0) {
                    bm_indexes[2] = 1;
                    bm_indexes[0] = i;
                    max_quotes += 1;
                }
                else if (c == '"' && bm_indexes[2] != 0) || (c == '\'' && bm_indexes[2] != 0) {
                    max_quotes += 1;
                    bm_indexes[1] = i;
                }
            } bm_indexes[2] = max_quotes;
            
            for (i,c) in raw_input.chars().enumerate() {
                if i >= bm_indexes[0] && i <= bm_indexes[1] {
                    clean_output.push(c);
                    clean_output_length += 1;
                }
            } 
            // need to add 2 to account for beginning and ending double quote chars
            (clean_output, clean_output_length + 2)
        }

        let mut args_total: i32 = 0; 
        let mut args_counter: i32 = 0; 
        $(args_total+=1; let _ = &$arg;)* 

        $(
            let read_only_arg = &$arg;
            let formatted_arg = format!("{:?}", $arg);
            // regular variable_name: type = value
            if is_valid_variable(stringify!($arg)) {
                let output_unit_length: usize = (stringify!($arg).chars().count() + return_type_of(read_only_arg).chars().count() + formatted_arg.chars().count());
                output_lengths_vector.push(output_unit_length + 5);
            // string types value: type
            } else {
                let (_, unit_length) = find_and_count_index_chars(stringify!($arg));
                let output_unit_length: usize = (unit_length + return_type_of(read_only_arg).chars().count());
                output_lengths_vector.push(output_unit_length);
            }
        )*
        
        let mut max_data_length: usize = 0;
        let length_option_output = output_lengths_vector.iter().max();
        if length_option_output.is_some() {
            max_data_length = *length_option_output.unwrap(); 
        }
            $(
                args_counter += 1; 
                let read_only_arg = &$arg;
                let formatted_arg = format!("{:?}", $arg);
                if args_counter == 1 {
                    let top_border = horizontal_border_draw_with_header(true, max_data_length);
                    // top_border = top_border.color_str_from_hex("#7252bc", false); // to color border, make top & seperator mutable and color
                    println!("{}",top_border);
                } else if args_counter != 1 && args_counter <= args_total {
                    let separator = row_separator_draw_with_header(max_data_length);
                    // separator = separator.color_str_from_hex("#7252bc", false); // to color border, make top & seperator mutable and color
                    println!("{}", separator);
                }

                if is_valid_variable(stringify!($arg)) {
                    let output_unit_length: usize = (stringify!($arg).chars().count() + return_type_of(read_only_arg).chars().count() + formatted_arg.chars().count());
                    output_lengths_vector.push(output_unit_length);
                    let mut left_vertical_border: String = String::new();
                    let _ = String::from(left_vertical_border);
                    // 1 - 9 args
                    if args_counter < 10 {
                        left_vertical_border = format!("│ {} │", args_counter);
                    } 
                    // 10 - 99 args
                    else if args_counter >= 10 && args_counter < 100 {
                        left_vertical_border = format!("│{} │", args_counter);
                    } 
                    // 100 - 999 args
                    else {
                        left_vertical_border = format!("│{}│", args_counter);
                    }
                    let output_unit_padding_required = max_data_length - output_unit_length;
                    let mut right_vertical_border: String = String::new();
                    // starting range at 5 basically allows us to subtract 5 from the padding without running into subtraction overflow errors since 5..3 (anything less than 5) gets ignored
                    for _ in 5..output_unit_padding_required {
                        right_vertical_border.push(' ')
                    }
                    right_vertical_border.push('│');
                    inner_data = format!("{} {}: {} = {}"
                    ,left_vertical_border
                    ,stringify!($arg).color_str_from_hex(TerminalColors::Variable.as_str(),true) 
                    ,return_type_of(read_only_arg).color_str_from_hex(TerminalColors::Type.as_str(),false)
                    ,formatted_arg.color_str_from_hex(TerminalColors::Value.as_str(),false)); 
                    println!("{} {}", inner_data, right_vertical_border);
                } else {
                    let (_, cleaned_output_length) = find_and_count_index_chars(stringify!($arg)); // new to santize string and count actual character length!
                    let output_unit_length: usize = (cleaned_output_length + return_type_of(read_only_arg).chars().count());
                    output_lengths_vector.push(output_unit_length);
                    let mut left_vertical_border: String = String::new();
                    let _ = String::from(left_vertical_border);
                    if args_counter < 10 {
                        left_vertical_border = format!("│ {} │", args_counter);
                    } else {
                        left_vertical_border = format!("│{} │", args_counter);
                    }
                    let output_unit_padding_required = (max_data_length - output_unit_length);
                    let mut right_vertical_border: String = String::new();
                    for _ in 0..output_unit_padding_required {
                        right_vertical_border.push(' ')
                    }
                    right_vertical_border.push('│');
                    inner_data = format!("{} {}: {}"
                    ,left_vertical_border
                    ,find_index_quote_chars(stringify!($arg)).color_str_from_hex(TerminalColors::Static.as_str(),false)
                    ,return_type_of(read_only_arg).color_str_from_hex(TerminalColors::Type.as_str(),false)); 
                    println!("{} {}", inner_data, right_vertical_border);
                }
                if args_counter == args_total {
                    let bottom_border = horizontal_border_draw_with_header(false, max_data_length);
                    println!("{}", bottom_border);
                    
                    // if is_valid_variable(stringify!($arg)) {
                        // let bottom_border = horizontal_border_draw_with_header(false, max_data_length);
                        // println!("{}", bottom_border);
                    // } else {
                        // let bottom_border = horizontal_border_draw_with_header(false, (max_data_length));
                        // println!("{}", bottom_border);
                    // }
                }
            )*
            println!("[finished display with {} args]", args_total);
        }};
}

pub(crate) use ntval;
