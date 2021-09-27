use iced::{button, container, Container, Align, Length, HorizontalAlignment, VerticalAlignment, Background, Button, Row, Column, Element, Sandbox, Settings, Text};
use rand::{thread_rng, seq::SliceRandom};
use lazy_static::lazy_static;

use std::sync::Mutex;
use chess_engine::*;
pub use chess_engine::Board;

pub fn run(get_cpu_move: fn(&Board) -> Move, starting_board: Board) -> iced::Result {
    {
        let mut x = GET_CPU_MOVE.lock().unwrap();
        *x = get_cpu_move;
        let mut x = STARTING_BOARD.lock().unwrap();
        *x = starting_board;
    };
    
    ChessBoard::run(Settings {
        window: iced::window::Settings {
            size: (
                (SQUARE_SIZE * 8) as u32,
                (SQUARE_SIZE * 8) as u32
            ),
            resizable: false,
            ..iced::window::Settings::default()
        },
        ..Settings::default()
    })
}

lazy_static! {
    static ref GET_CPU_MOVE: Mutex<fn(&Board) -> Move> = Mutex::new(best_move);
    static ref STARTING_BOARD: Mutex<Board> = Mutex::new(Board::default());
}

const SQUARE_SIZE: u16 = 48;
pub const AI_DEPTH: i32 = if cfg!(debug_assertions) {2} else {4};

pub fn get_symbol(piece: &Piece) -> impl ToString {
	match piece {
		Piece::King(_, _) => "K",
		Piece::Queen(_, _) => "Q",
		Piece::Rook(_, _) => "R",
		Piece::Bishop(_, _) => "B",
		Piece::Knight(_, _) => "N",
		Piece::Pawn(_, _) => "P",
	}
}

pub fn best_move(board: &Board) -> Move {
    board.get_best_next_move(AI_DEPTH).0
}

pub fn worst_move(board: &Board) -> Move {
    board.get_worst_next_move(AI_DEPTH).0
}

pub fn random_move(board: &Board) -> Move {
    let moves = board.get_legal_moves();

    let mut rng = thread_rng();
    *moves.choose(&mut rng).unwrap()
}


#[derive(Debug, Clone, Copy)]
pub enum Message {
    SelectSquare(Position),
}

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        iced::Color::from_rgb($r as f32 / 255.0, $g as f32 / 255.0, $b as f32 / 255.0)
    }
}

const SELECTED_DARK_SQUARE: iced::Color = rgb!(170,162,58);
const SELECTED_LIGHT_SQUARE: iced::Color = rgb!(205,210,106);

const LIGHT_SQUARE: iced::Color = rgb!(240,217,181);
const DARK_SQUARE: iced::Color = rgb!(181,136,99);


struct ChessSquare { row: i32, col: i32, piece_color: Color, is_selected: bool }

impl From<(Position, Color, bool)> for ChessSquare {
    fn from(pos_color: (Position, Color, bool)) -> Self {
        let (pos, color, is_selected) = pos_color;
        Self::new(pos.get_row(), pos.get_col(), color, is_selected)
    }
}

impl ChessSquare {
    fn new(row: i32, col: i32, piece_color: Color, is_selected: bool) -> Self {
        Self { row, col, piece_color, is_selected }
    }

    fn get_bg_color(&self, is_selected: bool) -> iced::Color {
        if (self.row * 9 + self.col) % 2 == 1 {
            if is_selected {
                SELECTED_LIGHT_SQUARE
            } else {
                LIGHT_SQUARE
            }
        } else {
            if is_selected {
                SELECTED_DARK_SQUARE
            } else {
                DARK_SQUARE
            }
        }
    }

    fn get_text_color(&self) -> iced::Color {
        if self.piece_color == WHITE {
            iced::Color::WHITE
        } else {
            iced::Color::BLACK
        }
    }
}


impl button::StyleSheet for ChessSquare {
    fn active(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.get_bg_color(self.is_selected))),
            border_color: self.get_bg_color(self.is_selected),
            text_color: self.get_text_color(),
            border_radius: 0.0,
            border_width: 0.0,
            ..button::Style::default()
        }
    }

    fn hovered(&self) -> button::Style {
        self.active()
    }

    fn pressed(&self) -> button::Style {
        button::Style {
            background: Some(Background::Color(self.get_bg_color(true))),
            border_color: self.get_bg_color(true),
            text_color: self.get_text_color(),
            border_radius: 0.0,
            border_width: 0.0,
            ..button::Style::default()
        }
    }
}

struct ChessBoardStyle;

impl container::StyleSheet for ChessBoardStyle {
    fn style(&self) -> container::Style {
        container::Style {
            border_color: iced::Color::BLACK,
            border_width: 10.0,
            border_radius: 0.0,
            ..container::Style::default()
        }
    }
}

#[derive(Clone, Copy)]
pub struct ChessBoard {
    get_cpu_move: fn(&Board) -> Move,
    starting_board: Board,
    result: GameResult,
    from_square: Option<Position>,
    board: Board,
    squares: [button::State; 64],
}

impl Default for ChessBoard {
    fn default() -> Self {
        let x = GET_CPU_MOVE.lock().unwrap();
        let get_cpu_move = *x;
        let x = STARTING_BOARD.lock().unwrap();
        let starting_board = *x;
        Self {
            get_cpu_move,
            starting_board,
            result: GameResult::Continuing(starting_board),
            from_square: None,
            board: if rand::random::<bool>() {
                starting_board
            } else {
                match starting_board.play_move((get_cpu_move)(&starting_board)) {
                    GameResult::Continuing(x) => x,
                    _ => starting_board
                }
            },
            squares: [button::State::default(); 64]
        }
    }
}

impl Sandbox for ChessBoard {
    type Message = Message;

    fn new() -> Self {
        Self::default()
    }

    fn title(&self) -> String {
        match self.result {
            GameResult::Victory(color) => format!("{} wins", color),
            GameResult::Stalemate => format!("Stalemate"),
            GameResult::IllegalMove(m) => format!("Illegal move by {}, '{}'", self.board.get_current_player_color(), m),
            _ => String::from("Chess")
        }
    }

    fn update(&mut self, message: Message) {
        match self.result {
            GameResult::Victory(_) | GameResult::Stalemate => {
                self.board = self.starting_board;
                self.result = GameResult::Continuing(self.board);
            },
            _ => {
                match (self.from_square, message) {
                    (None, Message::SelectSquare(pos)) => {
                        self.from_square = Some(pos);
                    }
                    (Some(from), Message::SelectSquare(to)) if from != to => {
                        let m = if ((from == E1 && to == G1) || (from == E8 && to == G8)) && Some(from) == self.board.get_king_pos(self.board.get_current_player_color()) {
                            Move::KingSideCastle
                        } else if ((from == E1 && to == C1) || (from == E8 && to == C8)) && Some(from) == self.board.get_king_pos(self.board.get_current_player_color()) {
                            Move::QueenSideCastle
                        } else {
                            Move::Piece(from, to)
                        };
                        
                        self.from_square = None;
                        self.board = match self.board.play_move(m) {
                            GameResult::Continuing(next_board) => {
                                match next_board.play_move((self.get_cpu_move)(&next_board)) {
                                    GameResult::Continuing(board) => {
                                        board
                                    }
                                    GameResult::Victory(color) => {
                                        self.result = GameResult::Victory(color);
                                        self.starting_board
                                    },
                                    GameResult::Stalemate => {
                                        self.result = GameResult::Stalemate;
                                        self.starting_board
                                    },
                                    GameResult::IllegalMove(m) => {
                                        eprintln!("AI tried to play illegal move '{}'", m);
                                        unreachable!()
                                    },
                                }
                            },
                            GameResult::Victory(color) => {
                                self.result = GameResult::Victory(color);
                                self.starting_board
                            },
                            GameResult::Stalemate => {
                                self.result = GameResult::Stalemate;
                                self.starting_board
                            },
                            GameResult::IllegalMove(_) => {
                                self.from_square = Some(to);
                                self.board
                            },
                        };
                    }
                    (Some(_), Message::SelectSquare(to)) => {
                        self.from_square = Some(to);
                    }
                }
            }
        }
    }

    fn view(&mut self) -> Element<Message> {
        let mut result = Column::new().spacing(0).align_items(Align::Center);
        let mut row = Row::new().spacing(0).align_items(Align::Center);
        let mut i = 0;

        let is_white = self.board.get_current_player_color() == WHITE;
        for button in &mut self.squares {
            // let r = if is_white { 7 - i / 8 } else { i / 8 };
            // let c = if is_white { i % 8 } else { 7 - (i % 8) };
            let r = if is_white { 7 - i / 8 } else { i / 8 };
            let c = if is_white { i % 8 } else { 7 - (i % 8) };
            
            let pos = Position::new(r, c);

            let (text, color) = if let Some(piece) = self.board.get_piece(pos) {
                (get_symbol(&piece).to_string(), piece.get_color())
            } else {
                (String::from(" "), WHITE)
            };
            
            row = row.push(Button::new(button,
                    Text::new(text)
                        .horizontal_alignment(HorizontalAlignment::Center)
                        .vertical_alignment(VerticalAlignment::Center)
                        .width(Length::Fill)
                        .height(Length::Fill)
                        .size(SQUARE_SIZE)
                )
                .width(Length::Units(SQUARE_SIZE))
                .height(Length::Units(SQUARE_SIZE))
                .on_press(Message::SelectSquare(pos))
                .style(ChessSquare::from((pos, color, self.from_square == Some(pos))))
            );
            i += 1;

            if i % 8 == 0 {
                result = result.push(row);
                row = Row::new().spacing(0).align_items(Align::Center);
            }
        }
        
        Container::new(result)
            .style(ChessBoardStyle)
            .width(Length::Shrink)
            .height(Length::Shrink)
            .padding(1)
            .into()
    }
}
