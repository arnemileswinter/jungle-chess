use rand::seq::SliceRandom;
use rand::thread_rng;

use crate::board::{get_other_player, Board, Player, TileCoord};

fn manhattan_distance((x1, y1): TileCoord, (x2, y2): TileCoord) -> isize {
    (x1 - x2).abs() + (y1 - y2).abs()
}

#[derive(PartialEq, PartialOrd, Eq, Ord, Clone, Copy, Debug)]
pub enum Evaluation {
    MinusInfinity,
    Evaluation(i32),
    PlusInfinity,
}

pub fn evaluate_board(board: &Board, who: Player) -> Evaluation {
    if board.has_player_won(who) {
        return Evaluation::PlusInfinity;
    } else if board.has_player_won(get_other_player(who)) {
        return Evaluation::MinusInfinity;
    }

    let our_pieces = board.get_player_pieces(who);
    let our_piece_values = our_pieces.iter().fold(0, |acc, p| acc + (p.0 as i32) + 1);

    let other_den_coord = board.get_den_coord_of(get_other_player(who));
    let combined_den_distances: i32 = our_pieces
        .iter()
        .map(|(_, pos)| manhattan_distance(*pos, other_den_coord) as i32)
        .sum(); //.expect("If no distance was here, we couldn't have won or lost.");
                // let shortest_den_distance= our_pieces.iter().map(|(_,pos)| manhattan_distance(*pos, other_den_coord) as i32).min().expect("If no distance was here, we couldn't have won or lost.");

    Evaluation::Evaluation(our_piece_values * 80 - combined_den_distances)
}

fn minimax(
    board: &Board,
    who: Player,
    horizon: i32,
    alpha: Evaluation,
    beta: Evaluation,
    maxing: bool,
) -> Evaluation {
    if horizon <= 0 || board.is_game_over() {
        return evaluate_board(board, who);
    }

    let mut mut_alpha = alpha;
    let mut mut_beta = beta;

    if maxing {
        let mut max_eval = Evaluation::MinusInfinity;
        for (from, to) in board
            .get_next_moves(who)
            .iter()
            .flat_map(move |(_, from, tos)| tos.iter().map(|to| (*from, *to)))
        {
            let next_board = board.make_move(who, from, to).unwrap().0;
            let eval = minimax(&next_board, who, horizon - 1, mut_alpha, mut_beta, false);
            max_eval = std::cmp::max(max_eval, eval);
            mut_alpha = std::cmp::max(mut_alpha, eval);
            if mut_beta <= mut_alpha {
                break;
            }
        }
        max_eval
    } else {
        let mut min_eval = Evaluation::PlusInfinity;
        let other = get_other_player(who);
        for (from, to) in board
            .get_next_moves(other)
            .iter()
            .flat_map(move |(_, from, tos)| tos.iter().map(|to| (*from, *to)))
        {
            let next_board = board.make_move(other, from, to).unwrap().0;
            let eval = minimax(&next_board, who, horizon - 1, mut_alpha, mut_beta, true);
            min_eval = std::cmp::min(min_eval, eval);
            mut_beta = std::cmp::min(mut_beta, eval);
            if mut_beta <= mut_alpha {
                break;
            }
        }
        min_eval
    }
}

pub fn get_ai_move(board: &Board, who: Player, horizon: i32) -> Option<(TileCoord, TileCoord)> {
    let mut next_moves = board.get_next_moves(who);
    let mut rng = thread_rng();
    next_moves.shuffle(&mut rng); // shuffle to be less predictable about what max_by will yield.

    next_moves
        .iter()
        .flat_map(|(_, from, tos)| tos.iter().map(move |to| (*from, *to)))
        .map(|(from, to)| {
            (
                from,
                to,
                minimax(
                    &board.make_move(who, from, to).unwrap().0,
                    who,
                    horizon,
                    Evaluation::MinusInfinity,
                    Evaluation::PlusInfinity,
                    false,
                ),
            )
        })
        .max_by(|(_, _, e1), (_, _, e2)| e1.cmp(e2))
        .map(|(from, to, _)| (from, to))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_evaluation_ord() {
        assert!(Evaluation::MinusInfinity < Evaluation::Evaluation(1));
        assert!(Evaluation::Evaluation(-1) < Evaluation::Evaluation(1));
        assert!(Evaluation::Evaluation(2) < Evaluation::PlusInfinity);
    }
}
