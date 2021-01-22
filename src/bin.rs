extern crate chess;
use chess::{Board, Color, Move, Position, Evaluate};
use std::io::{stdin, stdout, Write};

fn input(prompt: impl std::fmt::Display) -> String {
    let mut s = String::new();
    print!("{}", prompt);
    let _ = stdout().flush();
    stdin()
        .read_line(&mut s)
        .expect("Did not enter a correct string");
    return s;
}

fn main() -> Result<(), String> {
    let mut b = Board::default();
    println!("{}", b);

    let mut history = vec![];

    loop {
        let s = input(">>> ");
        let s = s.trim().to_string();

        let split_input = s.split_ascii_whitespace().collect::<Vec<&str>>();

        let m = if s.is_empty() {
            println!("CPU performing move...");
            let m = b.get_best_next_move(4, 2000000);
            println!("CPU chose {}", m);
            m
        } else if s == "history" {
            for (i, m) in history.iter().enumerate() {
                println!("{}. {}", i+1, m);
            }
            continue;
        } else if s == "resign" {
            let c = b.get_turn_color();
            println!("{} resigned. {} is victorious.", c, !c);
            break;
        } else if s == "O-O" {
            Move::KingSideCastle
        } else if s == "O-O-O" {
            Move::QueenSideCastle
        } else if split_input.len() == 1 && split_input[0].len() == 4 {
            match (Position::pgn(&s[0..2]), Position::pgn(&s[2..4])) {
                (Ok(from), Ok(to)) => Move::Piece(from, to),
                (Err(e), _) | (_, Err(e)) => {
                    eprintln!("error: {}", e);
                    continue;
                }
            }
        } else if split_input.len() == 2 {
            match (Position::pgn(split_input[0]), Position::pgn(split_input[1])) {
                (Ok(from), Ok(to)) => Move::Piece(from, to),
                (Err(e), _) | (_, Err(e)) => {
                    eprintln!("error: {}", e);
                    continue;
                }
            }
        } else {
            eprintln!("error: invalid input `{}`", s);
            continue;
        };

        if b.is_legal_move(m, b.get_turn_color()) {
            println!("{}", b.apply_move(m).set_turn(Color::White));
            b = b.apply_move(m).change_turn();
            history.push(m);
            // println!("{}", b.set_turn(Color::White));
        } else {
            eprintln!("error: illegal move");
            continue;
        }
    }

    for m in history {
        println!("{}", m);
    }
    Ok(())
}
