mod board;

use std::io::{self, BufRead, Read};

use crate::board::{get_other_player, Board, Player, TileCoord};

fn get_position_from_stdin(stdin: &mut io::StdinLock) -> TileCoord {
    let mut in_buf = [0, 0, 0]; // one for newline.
    stdin
        .read(&mut in_buf)
        .expect("failed reading into buffer.");
    if in_buf[0] < 'a' as u8 {
        println!("Did not understand x coordinate. Must be a lower-case letter.");
        return get_position_from_stdin(stdin);
    }
    if in_buf[1] < '1' as u8 {
        println!("Did not understand y coordinate. Must be a digit.");
        return get_position_from_stdin(stdin);
    }
    let c = (
        (in_buf[0] - 'a' as u8).into(),
        (in_buf[1] - '1' as u8).into(),
    );
    c
}

fn coord_to_position((x, y): TileCoord) -> String {
    let mut out = String::new();
    out.push(((x as u8) + 'a' as u8) as char);
    out.push(((y as u8) + '1' as u8) as char);
    out
}

fn main() {
    let mut b = Board::new();

    let mut stdin = io::stdin().lock();
    let mut player_to_move = Player::Player1;

    loop {
        let mut from_pos = (0, 0);
        let mut piece_to_move = None;
        while piece_to_move.is_none() {
            println!("{}", b);
            println!("{} to move. Select your piece.", player_to_move);
            from_pos = get_position_from_stdin(&mut stdin);
            match b.get_piece_at(from_pos) {
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

        let possible_moves: Vec<TileCoord> = b
            .get_next_moves(player_to_move)
            .iter()
            .filter(|(p, _, _)| piece_to_move.unwrap() == *p)
            .flat_map(|(_, _, ms)| -> Vec<TileCoord> { ms.iter().map(|m| *m).collect() })
            .collect();
        let mut to_pos = None;
        while to_pos.is_none() {
            println!(
                "Selected {}, where to go? Options are: ",
                piece_to_move.unwrap()
            );
            for m in possible_moves.iter() {
                print!("{}, ", coord_to_position(*m));
            }
            println!();

            let p = get_position_from_stdin(&mut stdin);
            if possible_moves.iter().any(|m: &TileCoord| *m == p) {
                to_pos = Some(p);
            }
        }

        let (new_b, winner, caps) = b
            .make_move(player_to_move, from_pos, to_pos.unwrap())
            .expect("Somehow you cheated.");

        println!("{:?} {:?}", winner, caps);
        if winner.is_some() {
            println!("{} won!", winner.unwrap());
            break;
        }
        b = new_b;

        player_to_move = get_other_player(player_to_move);
    }
}
