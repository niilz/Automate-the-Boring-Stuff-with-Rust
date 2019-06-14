use std::collections::HashMap;
use std::io;

fn main() {
    // TicTacToe
    let mut theBoard = HashMap::new();
    theBoard.insert("top-L".to_string(), " ");
    theBoard.insert("top-M".to_string(), " ");
    theBoard.insert("top-R".to_string(), " ");
    theBoard.insert("mid-L".to_string(), " ");
    theBoard.insert("mid-M".to_string(), " ");
    theBoard.insert("mid-R".to_string(), " ");
    theBoard.insert("low-L".to_string(), " ");
    theBoard.insert("low-M".to_string(), " ");
    theBoard.insert("low-R".to_string(), " ");

    fn printBoard(board: &HashMap<String, &str>) {
        println!(" {}|{}|{}", board["top-L"], board["top-M"], board["top-R"]);
        println!("--+-+--");
        println!(" {}|{}|{}", board["mid-L"], board["mid-M"], board["mid-R"]);
        println!("--+-+--");
        println!(" {}|{}|{}", board["low-L"], board["low-M"], board["low-R"]);
    }

    let mut turn = "X";
    
    for _ in 0..9 {
        printBoard(&theBoard);
        println!("Turn for {}. Move on which space?", turn);
        let mut mov = String::new();
        io::stdin().read_line(&mut mov);
        theBoard.insert(mov.trim().to_string(), turn);
        if turn == "X" {
            turn = "O";
        } else {
            turn = "X";
        }
    }
    printBoard(&theBoard);
}
