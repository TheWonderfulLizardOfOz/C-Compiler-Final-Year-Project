use compiler_implementation::lexer;
use compiler_implementation::lexer::Lexer;
use compiler_implementation::lexer::Token;
use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Error: Please provide a file path.");
        std::process::exit(1);
    }

    let file_path = &args[1];
    println!("In file {file_path}");

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut lexer = Lexer::new(&contents);
    lexer.print_input();

    let mut cur_tok: Token = lexer.get_next_token();
    while cur_tok.category != lexer::TokenCategory::Eof {
        println!("{:?}", cur_tok);
        cur_tok = lexer.get_next_token();
    }
    println!("{:?}", cur_tok);
}
