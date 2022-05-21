use std::{fmt::{Display, Formatter}};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Piece {
    Rat = 0,
    Cat = 1,
    Dog = 2,
    Wolf = 3,
    Leopard = 4,
    Tiger = 5,
    Lion = 6,
    Elephant = 7,
}

impl Piece {
    fn beats(self, other: Self) -> bool {
        match (self, other) {
            (Piece::Rat, Piece::Elephant) => true,
            (Piece::Elephant, Piece::Rat) => false,
            (a, b) => (a as u8) >= (b as u8),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        // f is the common name here, only renaming it because frmtt was flagged as misspelled in my IDE
        f.write_str(match self {
            Piece::Rat => "🐭",
            Piece::Cat => "🐱",
            Piece::Dog => "🐕",
            Piece::Wolf => "🐺",
            Piece::Leopard => "🐆",
            Piece::Tiger => "🐯",
            Piece::Lion => "🦁",
            Piece::Elephant => "🐘",
        })
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
pub enum Player {
    Player1,
    Player2,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        f.write_str(match self {
            Player::Player1 => "player blue",
            _ => "player red",
        })
    }
}

pub fn get_other_player(p: Player) -> Player {
    match p {
        Player::Player1 => Player::Player2,
        Player::Player2 => Player::Player1,
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Ground {
    Grass,
    Water,
    Trap(Player),
    Den(Player),
}

type Tile = (Ground, Option<(Player, Piece)>);
type Tiles = [Tile; TILES_W * TILES_H];
type TileIdx = usize;
pub type TileCoord = (isize, isize);

const TILES_W: TileIdx = 7;
const TILES_H: TileIdx = 9;
const TILES_COUNT: TileIdx = TILES_W * TILES_H;

#[derive(Debug, Clone)]
pub struct Board {
    tiles: Tiles,
}

fn map_project((x, y): TileCoord) -> TileIdx {
    (y * (TILES_W as isize) + x) as usize
}

fn map_unproject(i: TileIdx) -> TileCoord {
    ((i % TILES_W) as isize, (i / TILES_W) as isize)
}

fn init_map() -> Tiles {
    // fill with ground
    let mut tiles: Tiles = [(Ground::Grass, None); TILES_W * TILES_H];

    // fill water
    let mut put_water = |x: isize, y: isize| {
        tiles[map_project((x, y))] = (Ground::Water, None);
    };

    for x in 1..(TILES_W - 1) {
        if x == 3 {
            continue;
        }
        for y in 3..(TILES_H - 1 - 2) {
            put_water(x.clone() as isize, y.clone() as isize);
        }
    }
    // add player's pieces.
    // Because the index pattern was identical for the two players it can be shrinked down a lot.
    let mut put_piece = |x: usize, y: usize, what| {
        tiles[map_project((x as isize, y as isize))] =
            (Ground::Grass, Some((Player::Player1, what)));
        tiles[map_project(((TILES_W - 1 - x) as isize, (TILES_H - 1 - y) as isize))] =
            (Ground::Grass, Some((Player::Player2, what)));
    };

    put_piece(0, 0, Piece::Lion);
    put_piece(6, 0, Piece::Tiger);
    put_piece(1, 1, Piece::Dog);
    put_piece(5, 1, Piece::Cat);
    put_piece(0, 2, Piece::Rat);
    put_piece(2, 2, Piece::Leopard);
    put_piece(4, 2, Piece::Wolf);
    put_piece(6, 2, Piece::Elephant);

    // add player traps
    // Same trick as above
    let mut put_trap = |x: usize, y: usize| {
        tiles[map_project((x as isize, y as isize))] = (Ground::Trap(Player::Player1), None);
        tiles[map_project((x as isize, (TILES_H - 1 - y) as isize))] =
            (Ground::Trap(Player::Player2), None);
    };
    put_trap(2, 0);
    put_trap(4, 0);
    put_trap(3, 1);

    // add player dens
    let mut put_den = |x: usize, y: usize, who| {
        tiles[map_project((x as isize, y as isize))] = (Ground::Den(who), None);
    };
    put_den(3, 0, Player::Player1);
    put_den(3, TILES_H - 1, Player::Player2);

    tiles
}

pub fn is_coord_in_bounds((x, y): TileCoord) -> bool {
    x >= 0 && x < (TILES_W as isize) && y >= 0 && y < (TILES_H as isize)
}

impl Board {
    pub fn new() -> Self {
        Board { tiles: init_map() }
    }

    pub fn get_den_coord_of(&self, who: Player) -> TileCoord {
        match who {
            Player::Player1 => (3, 0),
            Player::Player2 => (3, (TILES_H - 1) as isize),
        }
    }

    pub fn has_player_won(&self, who: Player) -> bool {
        match self.tiles[map_project(self.get_den_coord_of(get_other_player(who)))] {
            (_, Some((p, _))) => p == who, // someone's at their den.
            _ => self.get_player_pieces(get_other_player(who)).is_empty(), // other player has no more pieces.
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.has_player_won(Player::Player1) || self.has_player_won(Player::Player2)
    }

    pub fn get_piece_at(&self, at: TileCoord) -> Option<(Player, Piece)> {
        if !is_coord_in_bounds(at) {
            None
        } else {
            match self.tiles[map_project(at)] {
                (_, o) => o,
            }
        }
    }

    /// Receives a player's pieces on the board.
    pub fn get_player_pieces(&self, who: Player) -> Vec<(Piece, TileCoord)> {
        self.tiles
            .iter()
            .zip(0..TILES_COUNT)
            .filter(|(t, _)| match t {
                (_, Some((p, _))) => *p == who,
                _ => false,
            })
            .map(|(t, idx)| match t {
                (_, Some((_, piece))) => (*piece, map_unproject(idx)),
                _ => unreachable!(), // This macro is a nice way of saying that code should not be possible to be reached
            })
            .collect()
    }

    /// makes a move as retrieved by get_next_moves.
    /// The result may be erroneous, if the move is illegal (e.g. not retrieved by get_next_moves).
    /// On success, the next state of the board is returned, along with an optional winning player, and optional captured pieces.
    pub fn make_move(
        &self,
        who: Player,
        from: TileCoord,
        to: TileCoord,
    ) -> Result<(Board, Option<Player>, Option<(Player, Piece)>), String> {
        if self
            .get_next_moves(who)
            .iter()
            .filter(|(_, s, _)| from == *s)
            .any(|(_, _, t)| t.iter().any(|tt| *tt == to))
        {
            // both from and to are valid coords because they were in next_moves.
            let moving_piece = self.get_piece_at(from);

            let mut won = Box::new(false);
            let mut capped = Box::new(None::<(Player, Piece)>);
            let mut next_board = self.clone();

            next_board.tiles[map_project(from)] = match self.tiles[map_project(from)] {
                (g, _) => (g, None),
            };
            next_board.tiles[map_project(to)] = match self.tiles[map_project(to)] {
                (g, optional_other_piece) => {
                    match g {
                        Ground::Den(other) => {
                            *won = who != other; // win - player entered opponent's den.
                        }
                        _ => (),
                    };
                    *capped = optional_other_piece;
                    (g, moving_piece)
                }
            };

            *won = *won
                || next_board
                    .get_player_pieces(get_other_player(who))
                    .is_empty(); // win - opponent has no more pieces.

            Ok((next_board, if *won { Some(who) } else { None }, *capped))
        } else {
            Err(format!("illegal move! from {:?} to {:?} ", from, to))
        }
    }

    /// Get the next available moves for player `who` on the current board.
    /// The result type is a vector that map's each piece of `who` to the respective position,
    /// along with the possible follow-up positions.
    pub fn get_next_moves(&self, who: Player) -> Vec<(Piece, TileCoord, Vec<TileCoord>)> {
        let up_of = |(x, y): TileCoord| -> TileCoord { (x, y + 1) };
        let down_of = |(x, y): TileCoord| -> TileCoord { (x, y - 1) };
        let left_of = |(x, y): TileCoord| -> TileCoord { (x - 1, y) };
        let right_of = |(x, y): TileCoord| -> TileCoord { (x + 1, y) };

        let is_water_at = |c| {
            is_coord_in_bounds(c)
                && match self.tiles[map_project(c)] {
                    (Ground::Water, _) => true,
                    _ => false,
                }
        };
        let is_rat_at = |c| {
            is_coord_in_bounds(c)
                && match self.tiles[map_project(c)] {
                    (_, Some((_, Piece::Rat))) => true,
                    _ => false,
                }
        };

        let can_step_from_to = |p: Piece, from, to| {
            is_coord_in_bounds(from)
                && is_coord_in_bounds(to)
                && match (
                    p,
                    self.tiles[map_project(from)],
                    self.tiles[map_project(to)],
                ) {
                    // rats beat other rats in water.
                    (Piece::Rat, (Ground::Water, _), (Ground::Water, _)) => true,
                    // rats can enter water if unoccupied.
                    (Piece::Rat, (Ground::Grass, _), (Ground::Water, None)) => true,
                    // rats don't beat other rats if coming from grass to water.
                    (Piece::Rat, (Ground::Grass, _), (Ground::Water, Some(_))) => false,
                    // rats cannot emerge from water if occupied.
                    (Piece::Rat, (Ground::Water, _), (Ground::Grass, Some(_))) => false,
                    // other pieces cannot enter water.
                    (_, _, (Ground::Water, _)) => false,
                    // every piece can walk freely on grass.
                    (_, (Ground::Grass, _), (Ground::Grass, None)) => true,
                    // if grass is occupied, a piece can only move towards with capture of opponent piece.
                    (
                        _,
                        (Ground::Grass | Ground::Trap(_), _),
                        (Ground::Grass, Some((other_player, other_piece))),
                    ) => who != other_player && p.beats(other_piece),
                    // if trap is occupied, a piece can only move towards with capture of opponent piece.
                    (_, _, (Ground::Trap(trap_owner), Some((piece_owner, _)))) => {
                        if trap_owner == piece_owner {
                            trap_owner != who // cannot capture our own pieces in traps.
                        } else {
                            false
                        }
                    }
                    // can always enter the opponent's den, but not our own.
                    (_, _, (Ground::Den(other_player), _)) => other_player != who,
                    (_, _, _) => true,
                }
        };
        let next_steps = |p: Piece, c: TileCoord| -> Vec<TileCoord> {
            let generic_neighbors = |p, c| {
                [up_of(c), left_of(c), right_of(c), down_of(c)]
                    .iter()
                    .filter(|cc: &&TileCoord| can_step_from_to(p, c, **cc))
                    .map(|c| *c)
                    .collect()
            };

            let mut steps: Vec<TileCoord> = generic_neighbors(p, c);
            match p {
                Piece::Tiger | Piece::Lion => {
                    if is_water_at(down_of(c)) {
                        if !is_rat_at(down_of(c))
                            && !is_rat_at(down_of(down_of(c)))
                            && !is_rat_at(down_of(down_of(down_of(c))))
                            && can_step_from_to(p, c, down_of(down_of(down_of(down_of(c)))))
                        {
                            steps.push(down_of(down_of(down_of(down_of(c)))));
                        }
                    }
                    if is_water_at(up_of(c)) {
                        if !is_rat_at(up_of(c))
                            && !is_rat_at(up_of(up_of(c)))
                            && !is_rat_at(up_of(up_of(up_of(c))))
                            && can_step_from_to(p, c, up_of(up_of(up_of(up_of(c)))))
                        {
                            steps.push(up_of(up_of(up_of(up_of(c)))));
                        }
                    }
                    if is_water_at(left_of(c)) {
                        if !is_rat_at(left_of(c))
                            && !is_rat_at(left_of(left_of(c)))
                            && can_step_from_to(p, c, left_of(left_of(left_of(c))))
                        {
                            steps.push(left_of(left_of(left_of(c))));
                        }
                    }
                    if is_water_at(right_of(c)) {
                        if !is_rat_at(right_of(c))
                            && !is_rat_at(right_of(right_of(c)))
                            && can_step_from_to(p, c, right_of(right_of(right_of(c))))
                        {
                            steps.push(right_of(right_of(right_of(c))));
                        }
                    }
                }
                _ => (),
            }
            steps
        };
        let pieces = self.get_player_pieces(who);

        pieces
            .iter()
            .map(|(p, c)| (*p, *c, next_steps(*p, *c)))
            .collect()
    }

    pub fn iter(&self) -> BoardIterator<'_> {
        BoardIterator {board : self, idx : 0}
    }
}

impl<'a> IntoIterator for &'a Board {
    type Item = (TileCoord, Tile);
    type IntoIter = BoardIterator<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

pub struct BoardIterator<'a> {
    board : &'a Board,
    idx : usize,
}

impl<'a> Iterator for BoardIterator<'a> {
    type Item = (TileCoord, Tile);

    fn next(&mut self) -> Option<Self::Item> {
        let result = if self.idx >= TILES_COUNT{
            None
        } else {
            Some((map_unproject(self.idx), self.board.tiles[self.idx]))
        };
        self.idx += 1;
        result
    }
}
impl Display for Board {
    fn fmt(&self, frmtt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let player_colored = |p, s: &str| match p {
            Player::Player1 => format!("\x1b[46m{}\x1b[0m", s),
            Player::Player2 => format!("\x1b[41m{}\x1b[0m", s),
        };

        let tile_to_str = |t: &Tile| match *t {
            (_, Some((p, Piece::Rat))) => player_colored(p, "🐭"),
            (_, Some((p, Piece::Cat))) => player_colored(p, "🐱"),
            (_, Some((p, Piece::Dog))) => player_colored(p, "🐕"),
            (_, Some((p, Piece::Wolf))) => player_colored(p, "🐺"),
            (_, Some((p, Piece::Leopard))) => player_colored(p, "🐆"),
            (_, Some((p, Piece::Tiger))) => player_colored(p, "🐯"),
            (_, Some((p, Piece::Lion))) => player_colored(p, "🦁"),
            (_, Some((p, Piece::Elephant))) => player_colored(p, "🐘"),

            (Ground::Grass, _) => "🟩".to_string(),
            (Ground::Den(p), _) => player_colored(p, "🏠"),
            (Ground::Water, _) => "🟦".to_string(),
            (Ground::Trap(p), _) => player_colored(p, "🥅"),
        };
        let formatted = " a b c d e f g\n".to_string()
            + &(self
                .tiles
                .iter()
                .zip(0..TILES_COUNT)
                .map(|(t, idx)| {
                    format!(
                        "{}{}{}",
                        if idx % TILES_W == 0 {
                            format!("{}", (idx / TILES_W) + 1)
                        } else {
                            "".to_string()
                        },
                        tile_to_str(t),
                        if (idx + 1) % TILES_W == 0 { "\n" } else { "" }
                    )
                })
                .fold("".to_string(), |acc, s| format!("{}{}", acc, s)));
        frmtt.write_str(&formatted)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_project() {
        assert_eq!(map_project((0, 0)), 0);
        assert_eq!(map_project((1, 0)), 1);
        assert_eq!(map_project((0, 1)), TILES_W);
        assert_eq!(map_project((1, 2)), 2 * TILES_W + 1);
    }

    #[test]
    fn test_map_unproject() {
        assert_eq!(map_unproject(0), (0, 0));
        assert_eq!(map_unproject(TILES_W), (0, 1));
        assert_eq!(map_unproject(2 * TILES_W + 1), (1, 2));
        assert_eq!(map_unproject(4), (4, 0));
    }

    #[test]
    fn test_get_player_pieces() {
        let b = Board::new();
        assert_eq!(
            vec![
                (Piece::Lion, (0, 0)),
                (Piece::Tiger, (6, 0)),
                (Piece::Dog, (1, 1)),
                (Piece::Cat, (5, 1)),
                (Piece::Rat, (0, 2)),
                (Piece::Leopard, (2, 2)),
                (Piece::Wolf, (4, 2)),
                (Piece::Elephant, (6, 2))
            ],
            b.get_player_pieces(Player::Player1)
        );
        assert_eq!(
            vec![
                (Piece::Elephant, (0, 6)),
                (Piece::Wolf, (2, 6)),
                (Piece::Leopard, (4, 6)),
                (Piece::Rat, (6, 6)),
                (Piece::Cat, (1, 7)),
                (Piece::Dog, (5, 7)),
                (Piece::Tiger, (0, 8)),
                (Piece::Lion, (6, 8))
            ],
            b.get_player_pieces(Player::Player2)
        );
    }

    #[test]
    fn beats() {
        assert!(Piece::Elephant.beats(Piece::Lion));
        assert!(Piece::Elephant.beats(Piece::Elephant));
        assert!(Piece::Rat.beats(Piece::Elephant));
        assert!(Piece::Cat.beats(Piece::Rat));
        assert!(!(Piece::Elephant.beats(Piece::Rat)));
    }

    #[test]
    fn iterates() {
        let b = Board::new();
        for (coord, t) in b.into_iter() {
            assert_eq!(coord, (0,0));
            assert_eq!(t, (Ground::Grass, Some((Player::Player1, Piece::Lion))));
            break;
        }

        for _ in b.into_iter() { 
            // should terminate.
        }

        assert_eq!(b.iter().last(), Some(((6,8), (Ground::Grass, Some((Player::Player2, Piece::Lion))))));
    }
}
