// TABLE Printer

fn main() {
    // given Table-data like:
    //
    let table_data = vec![
        vec!["apples", "oranges", "cherries", "banana"],
        vec!["Alice", "Bob", "Carol", "David"],
        vec!["dogs", "cats", "moose", "goose"]
        ];
    //
    // the progam should print:
    // 
    //  apples Alice  dogs
    // oranges   Bob  cats
    //cherries Carol moose
    //  banana David goose

    fn print_table(t_data: Vec<Vec<&str>>) {

        let longest_words: Vec<usize> = t_data
                            .iter()
                            .map(|row| row.iter().fold(0, |l, w| if w.len() > l { w.len() } else { l }))
                            .collect();
                       
        let t_data_formatted: Vec<Vec<String>> = t_data
                            .iter()
                            .enumerate()
                            .map(|(i, row)| {
                                row
                                .iter()
                                .map(|word| format!("{word:>width$}", word=word, width=longest_words[i]))
                                .collect()
                            }).collect();
        
        for col in 0..t_data[0].len() {
            let current_row = (0..t_data.len())
                        .map(|row| {
                            t_data_formatted[row][col].as_ref()
                        })
                        .collect::<Vec<&str>>()
                        .join(" ");
            println!("{}", current_row);
        }
    }

    
    print_table(table_data);

    // and indeed. it prints:
    //     apples Alice  dogs
    //    oranges   Bob  cats
    //   cherries Carol moose
    //     banana David goose
}
