fn calcualate_line_spaces(total_symbols: usize, num_words: usize, line_width: usize) -> (usize, usize) {
    let base_num_spaces;
    let mut extra_spaces = 0;
    if num_words == 1 { 
        base_num_spaces = line_width - total_symbols;
    } else { 
        base_num_spaces = (line_width - total_symbols) / (num_words - 1);
        extra_spaces = line_width - total_symbols - (base_num_spaces * (num_words - 1)); 
    };

    return (base_num_spaces, extra_spaces);
}

fn fill_the_line(result: &mut String, words_in_line: &Vec<&str>, symbols_in_line: usize, line_width: usize) {

    if words_in_line.is_empty() {
        return;
    }

    const SPACE: char = ' ';
    const EOL: char = '\n';
    
    if !result.is_empty() {
        result.push(EOL);
    }

    let (base_num_spaces, mut extra_spaces) = calcualate_line_spaces(symbols_in_line, words_in_line.len(), line_width);
    for (i, w) in words_in_line.iter().enumerate() {
        *result += w;

        if (i == 0) || (i != (words_in_line.len() - 1)) {
            let mut num_spaces = base_num_spaces;
            if extra_spaces > 0 {
                extra_spaces -= 1;
                num_spaces += 1
            }
            for _ in 0..num_spaces {
                result.push(SPACE);
            }
        }
    }
}

fn split_into_lines(input: &str, line_width: usize) -> String { 
    let mut result = String::new();
    // optional: since the task has an emphasis on performance
    // reserve some bytes for result (it's proportional to input)
    result.reserve(input.len());
    let mut words = input.split_whitespace();
    let mut words_in_line: Vec<&str> = Vec::new();
    let mut line_filling: usize = 0;
    let mut symbols_in_line: usize = 0;

    loop {
        match words.next() {
            Some(w) => {
                // TODO: It's better to use "unicode-segmentation" crate and operate 
                // with graphemes but for now let's simplify task and operate with code points
                let word_length = w.chars().count();
                if word_length > line_width {
                    panic!("word \"{}\" is too big for the line (width {})", w, line_width);
                }

                let space: usize = if words_in_line.len() == 0 { 0 } else { 1 };
                let new_line_filling = line_filling + space + word_length;
                if new_line_filling <= line_width {
                    words_in_line.push(w);
                    symbols_in_line += word_length;
                    line_filling = new_line_filling;
                    
                } else {
                    fill_the_line(&mut result, &words_in_line, symbols_in_line, line_width);
                    words_in_line.clear();
                    words_in_line.push(w);
                    symbols_in_line = word_length;
                    line_filling = word_length;
                }
            },
            None => {
                // after the last word processed
                fill_the_line(&mut result, &words_in_line, symbols_in_line, line_width);
                break;
            }
        }
    }

    return result;
}

pub fn transform(input: &str, line_width: u32) -> String {
    return split_into_lines(input, line_width as usize);
}

#[cfg(test)]
mod tests {
    use super::transform;

    #[test]
    fn transform_succeeded() {
        let test_cases = [
            ("", 5, ""),
            ("test", 5, "test "),
            ("Catonem autem certius exemplar sapientis uiri nobis deos inmortalis dedisse quam Vlixem et Herculem prioribus saeculis", 12,
             "Catonem     \nautem       \ncertius     \nexemplar    \nsapientis   \nuiri   nobis\ndeos        \ninmortalis  \ndedisse quam\nVlixem    et\nHerculem    \nprioribus   \nsaeculis    "),
            ("    Catonem    autem    certius    ", 21, "Catonem autem certius"),
            ("Без труда не выловишь и рыбку из пруда", 17, "Без    труда   не\nвыловишь  и рыбку\nиз          пруда"),
            ("塞翁失马 焉知非福", 7, "塞翁失马   \n焉知非福   ")
        ];

        for &(input, line_width, expected) in &test_cases {
            println!("input: '{}'", input);
            assert_eq!(transform(input, line_width), expected);
        }
    }

    #[test]
    #[should_panic]
    fn transform_panics_on_too_big_word() {
        transform("should panic on 8symbols or more", 7);
    }
}
