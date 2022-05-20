use crate::board::{get_other_player, Board, Player, Piece, TileCoord};

fn manhattan_distance((x1,y1) : TileCoord, (x2,y2) : TileCoord) -> isize {
    (x1-x2).abs() + (y1-y2).abs()
}

/// Larger means better.
pub fn estimate(who: Player, board: &Board) -> i32 {
    if board.has_player_won(who) {
        return 0;
    }
    
    let other_den_coord = board.get_den_coord_of(get_other_player(who));
    let pieces = board.get_player_pieces(who);
    (pieces
     .iter()
     .map(|(p, piece_coord)| manhattan_distance(other_den_coord, *piece_coord) * -(*p as isize + 1))
     .min()
     .unwrap()) as i32
}
