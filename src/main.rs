mod ai;
mod board;

use clap::Parser;

use std::io::{self, Read, Write};

use board::Piece;

use crate::{
    ai::get_ai_move,
    board::{get_other_player, Board, Player, TileCoord},
};

enum Prompt<T> {
    Valid(T),
    Invalid,
    Abort,
}

fn print_turn(board: &Board, winner: Option<Player>, caps: Option<(Player, Piece)>) {
    println!(
        "{} {}",
        winner
            .map(|p| format!("{}\n{} won the game!", board, p))
            .unwrap_or_default(),
        caps.map(|(p, c)| format!("captured {}'s {}.", p, c))
            .unwrap_or_default()
    );
}

fn get_position_from_stdin(stdin: &mut io::StdinLock) -> Prompt<TileCoord> {
    let mut in_buf = [0, 0, 0]; // one for newline.
    let _ = stdin
        .read(&mut in_buf)
        .expect("failed reading into buffer.");

    match in_buf[0] {
        b'q' | b'h' => return Prompt::Abort, // b'c' does the same as 'c' as u8, but it cannot crash on non ascii characters.
        b'a'..=b'g' => (),            // Handle all valid input and do nothing
        _ => {
            // Catch any input left, which is by definition invalid
            println!("Did not understand x coordinate. Must be a lower-case letter.");
            return Prompt::Invalid;
        }
    }
    if !in_buf[1].is_ascii_digit() {
        // There are a ton of nice helper functions see: https://doc.rust-lang.org/std/primitive.u8.html
        println!("Did not understand y coordinate. Must be a digit.");
        return Prompt::Invalid;
    }
    Prompt::Valid(((in_buf[0] - b'a').into(), (in_buf[1] - b'1').into())) // This can be consolidated to a single line, but that is personal preference
}

fn coord_to_position((x, y): TileCoord) -> String {
    format!("{}{}", (x as u8 + b'a') as char, (y as u8 + b'1') as char) // Here I would say a single line is definitely better
}

fn get_next_valid_move_from_stdin(
    board: &Board,
    player_to_move: Player,
    stdin: &mut io::StdinLock,
) -> (TileCoord, TileCoord) {
    let mut from_pos = (0, 0);
    let mut piece_to_move = None;
    while piece_to_move.is_none() {
        print!("{player_to_move} to move. Select your piece: "); // This is very recent syntax, just wanted to show it off ;-)
        io::stdout().flush().expect("failed writing to stdout.");

        match get_position_from_stdin(stdin) {
            Prompt::Valid(p) => {
                from_pos = p;
            }
            Prompt::Invalid => {
                println!("Not a valid coordinate! must be a1 to g9.");
                continue;
            }
            Prompt::Abort => {
                continue;
            }
        };

        match board.get_piece_at(from_pos) {
            None => println!("No player piece at coordinate."),
            Some((owner, p)) => {
                if owner != player_to_move {
                    println!("That is not your piece.")
                } else {
                    piece_to_move = Some(p)
                }
            }
        }
    }

    let possible_moves: Vec<TileCoord> = board
        .get_next_moves(player_to_move)
        .iter()
        .filter(|(p, _, _)| piece_to_move.unwrap() == *p)
        .flat_map(|(_, _, ms)| -> Vec<TileCoord> { ms.iter().map(|m| *m).collect() })
        .collect();
    if possible_moves.is_empty() {
        println!("Cannot move that piece! Select another.");
        return get_next_valid_move_from_stdin(board, player_to_move, stdin);
    }
    let mut to_pos = None;
    while to_pos.is_none() {
        println!(
            "Selected {}. Options are: {} or q to select another piece.",
            piece_to_move.unwrap(),
            possible_moves
                .iter()
                .map(|m| format!("{}", coord_to_position(*m)))
                .reduce(|accum, item| format!("{}, {}", accum, item))
                .unwrap()
        );
        print!("Where to move? ");
        io::stdout().flush().expect("failed writing to stdout.");

        match get_position_from_stdin(stdin) {
            Prompt::Valid(p) => {
                if possible_moves.iter().any(|m: &TileCoord| *m == p) {
                    to_pos = Some(p);
                }
            }
            Prompt::Invalid => {
                println!("Not a valid coordinate! must be a1 to g9.");
                continue;
            }
            Prompt::Abort => return get_next_valid_move_from_stdin(board, player_to_move, stdin),
        };
    }

    (from_pos, to_pos.unwrap())
}

fn human_game() {
    let mut b = Board::new();

    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut player_to_move = Player::Player1;

    loop {
        println!("{}", b);
        let (from_pos, to_pos) = get_next_valid_move_from_stdin(&b, player_to_move, &mut stdin);
        let (new_b, winner, caps) = b.make_move(player_to_move, from_pos, to_pos).unwrap(); // (|| panic!("Somehow an invalid move got through, namely {} to {}.", coord_to_position(from_pos), coord_to_position(to_pos)));
        print_turn(&b, winner, caps);


        if winner.is_some() {
            println!("{}", new_b);
            break;
        }
 
        b = new_b;
        player_to_move = get_other_player(player_to_move);
    }
}

fn ai_game(human_player: Player, ai_horizon : i32) {
    let mut b = Board::new();
    let stdin = io::stdin();
    let mut stdin = stdin.lock();
    let mut player_to_move = Player::Player1;

    loop {
        if player_to_move == human_player {
            println!("{}", b);
        }

        let (from_pos, to_pos) = if human_player == player_to_move {
            get_next_valid_move_from_stdin(&b, human_player, &mut stdin)
        } else {
            let (f, t) = get_ai_move(&b, player_to_move, ai_horizon).unwrap();
            println!(
                "Computer played {} to {}.",
                coord_to_position(f),
                coord_to_position(t)
            );
            (f, t)
        };
        let (new_b, winner, caps) = b.make_move(player_to_move, from_pos, to_pos).unwrap(); // (|| panic!("Somehow an invalid move got through, namely {} to {}.", coord_to_position(from_pos), coord_to_position(to_pos)));
        print_turn(&b, winner, caps);

        if winner.is_some() {
            println!("{}", new_b);
            break;
        }

        b = new_b;
        player_to_move = get_other_player(player_to_move);
    }
}

#[derive(Parser, Debug)]
#[clap(name = "Jungle Chess")]
#[clap(author = "Arne Winter https://github.com/arnemileswinter")]
#[clap(version,about,long_about=None)]
struct Cli {
    /// To play against AI.
    // By creating a group and requiring that group you make sure none can pass `red` without passing `ai`.
    // Arguably in this case that is OP, but it is a nice feature when you need it ;-)
    #[clap(short, long, group = "ai_group")]
    ai: bool,
    /// To start as red in AI match.
    #[clap(short, long, requires = "ai_group")]
    red: bool,

    /// how many turns ahead the AI tries to look.
    /// Defaults to 3.
    /// Smaller values make the AI easier to beat.
    /// Larger values make the game slower.
    /// The default seems to work good.
    #[clap(short,long,requires="ai_group",default_value_t=3)]
    horizon: u32,
}

fn main() {
    let cli = Cli::parse();
    if cli.ai {
        ai_game(if cli.red {
            Player::Player2
        } else {
            Player::Player1
        }, cli.horizon as i32);
    } else {
        human_game();
    }
}
