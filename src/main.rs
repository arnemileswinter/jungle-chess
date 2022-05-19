use std::fmt::{Display, Formatter};

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Piece {
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
            (a, b) => (a as u8) > (b as u8),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(u8)]
enum Player {
    Player1,
    Player2,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum Ground {
    Grass,
    Water,
    Trap(Player),
    Den(Player),
}

type Tile = (Ground, Option<(Player, Piece)>);
type Tiles = [Tile; TILES_W * TILES_H];
type TileIdx = usize;
type TileCoord = (isize, isize);

const TILES_W: TileIdx = 7;
const TILES_H: TileIdx = 9;
const TILES_COUNT: TileIdx = TILES_W * TILES_H;

#[derive(Debug)]
struct Board {
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
    let mut put_piece = |x: usize, y: usize, who, what| {
        tiles[map_project((x as isize, y as isize))] = (Ground::Grass, Some((who, what)));
    };

    put_piece(0, 0, Player::Player1, Piece::Lion);
    put_piece(6, 0, Player::Player1, Piece::Tiger);
    put_piece(1, 1, Player::Player1, Piece::Dog);
    put_piece(5, 1, Player::Player1, Piece::Cat);
    put_piece(0, 2, Player::Player1, Piece::Rat);
    put_piece(2, 2, Player::Player1, Piece::Leopard);
    put_piece(4, 2, Player::Player1, Piece::Wolf);
    put_piece(6, 2, Player::Player1, Piece::Elephant);

    put_piece(
        TILES_W - 1 - 0,
        TILES_H - 1 - 0,
        Player::Player2,
        Piece::Lion,
    );
    put_piece(
        TILES_W - 1 - 6,
        TILES_H - 1 - 0,
        Player::Player2,
        Piece::Tiger,
    );
    put_piece(
        TILES_W - 1 - 1,
        TILES_H - 1 - 1,
        Player::Player2,
        Piece::Dog,
    );
    put_piece(
        TILES_W - 1 - 5,
        TILES_H - 1 - 1,
        Player::Player2,
        Piece::Cat,
    );
    put_piece(
        TILES_W - 1 - 0,
        TILES_H - 1 - 2,
        Player::Player2,
        Piece::Rat,
    );
    put_piece(
        TILES_W - 1 - 2,
        TILES_H - 1 - 2,
        Player::Player2,
        Piece::Leopard,
    );
    put_piece(
        TILES_W - 1 - 4,
        TILES_H - 1 - 2,
        Player::Player2,
        Piece::Wolf,
    );
    put_piece(
        TILES_W - 1 - 6,
        TILES_H - 1 - 2,
        Player::Player2,
        Piece::Elephant,
    );

    // add player traps
    let mut put_trap = |x: usize, y: usize, who| {
        tiles[map_project((x as isize, y as isize))] = (Ground::Trap(who), None);
    };
    put_trap(2, 0, Player::Player1);
    put_trap(4, 0, Player::Player1);
    put_trap(3, 1, Player::Player1);
    put_trap(2, TILES_H - 1 - 0, Player::Player2);
    put_trap(4, TILES_H - 1 - 0, Player::Player2);
    put_trap(3, TILES_H - 1 - 1, Player::Player2);
    // add player dens
    let mut put_den = |x: usize, y: usize, who| {
        tiles[map_project((x as isize, y as isize))] = (Ground::Den(who), None);
    };
    put_den(3, 0, Player::Player1);
    put_den(3, TILES_H - 1, Player::Player2);

    return tiles;
}

impl Board {
    fn new() -> Self {
        let tiles = init_map();
        Board { tiles: tiles }
    }

    fn get_player_pieces(&self, who: Player) -> Vec<(Piece, TileCoord)> {
        self.tiles
            .iter()
            .zip(0..TILES_COUNT)
            .filter(|(t, _)| match t {
                (_, Some((p, _))) => *p == who,
                _ => false,
            })
            .map(|(t, idx)| match t {
                (_, Some((_, piece))) => (*piece, map_unproject(idx)),
                _ => panic!("Match should've been filtered out."),
            })
            .collect()
    }

    fn get_next_moves(&self, who: Player) -> Vec<(Piece, TileCoord, Vec<TileCoord>)> {
        let up_of = |(x, y): TileCoord| -> TileCoord { (x, y + 1) };
        let down_of = |(x, y): TileCoord| -> TileCoord { (x, y - 1) };
        let left_of = |(x, y): TileCoord| -> TileCoord { (x - 1, y) };
        let right_of = |(x, y): TileCoord| -> TileCoord { (x + 1, y) };

        let is_coord_in_bounds = |(x,y): TileCoord| x >= 0 && x < (TILES_W as isize) && y >= 0 && y < (TILES_H as isize);
        
        let is_water_at = |c| is_coord_in_bounds(c) && match self.tiles[map_project(c)] {
            (Ground::Water, _) => true,
            _ => false,
        };
        let is_rat_at = |c| is_coord_in_bounds(c) && match self.tiles[map_project(c)] {
            (_, Some((_, Piece::Rat))) => true,
            _ => false,
        };


        let can_step_from_to = |p: Piece, from, to| {
            is_coord_in_bounds(from) && is_coord_in_bounds(to)
                && match (
                    p,
                    self.tiles[map_project(from)],
                    self.tiles[map_project(to)],
                ) {
                    // rats beat other rats in water.
                    (Piece::Rat, (Ground::Water, _), (Ground::Water, _)) => true,
                    // rats dont beat other rats if coming from grass to water.
                    (Piece::Rat, (Ground::Grass, _), (Ground::Water, Some(_))) => false,
                    // other pieces cannot enter water.
                    (_, _, (Ground::Water, _)) => false,
                    // every piece can walk freely on grass.
                    (_, (Ground::Grass, _), (Ground::Grass, None)) => true,
                    // if grass is occupied, a piece can only move towards with capture.
                    (_, (Ground::Grass, _), (Ground::Grass, Some((other_player, other_piece)))) => {
                        who != other_player && p.beats(other_piece)
                    }
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

            let mut steps: Vec<TileCoord> = generic_neighbors(p,c);
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
                        if  !is_rat_at(up_of(c))
                            && !is_rat_at(up_of(up_of(c)))
                            && !is_rat_at(up_of(up_of(up_of(c))))
                            && can_step_from_to(p, c, up_of(up_of(up_of(up_of(c)))))
                        {
                            steps.push(up_of(up_of(up_of(up_of(c)))));
                        }
                    }
                    if is_water_at(left_of(c)) {
                        if  !is_rat_at(left_of(c))
                            && !is_rat_at(left_of(left_of(c)))
                            && can_step_from_to(p, c, left_of(left_of(left_of(c))))
                        {
                            steps.push(left_of(left_of(left_of(c))));
                        }
                    }
                    if is_water_at(right_of(c)) {
                        if  !is_rat_at(right_of(c))
                            && !is_rat_at(right_of(right_of(c)))
                            && can_step_from_to(p, c, right_of(right_of(right_of(c))))
                        {
                            steps.push(right_of(right_of(right_of(c))));
                        }
                    }
                },
                _ => (),
            }
            steps
        };
        let pieces = self.get_player_pieces(who);

        pieces.iter().map(|(p,c)| (p,c,next_steps(*p,*c))).map(|(p,c,d)| (*p,*c,d)).collect()
    }
}

impl Display for Board {
    fn fmt(&self, frmtt: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        let player_colored = |p, s: &str| match p {
            Player::Player1 => format!("\x1b[41m{}\x1b[0m", s.to_string()),
            Player::Player2 => format!("\x1b[46m{}\x1b[0m", s.to_string()),
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
        let formatted = self
            .tiles
            .iter()
            .zip(0..TILES_COUNT)
            .map(|(t, idx)| {
                format!(
                    "{}{}",
                    tile_to_str(t),
                    if (idx + 1) % TILES_W == 0 { "\n" } else { "" }
                )
            })
            .fold("".to_string(), |acc, s| format!("{}{}", acc, s));
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
    fn next_steps() {
        let b = Board::new();
        dbg!(b.get_next_moves(Player::Player2));
    }

    #[test]
    fn beats() {
        assert!(Piece::Elephant.beats(Piece::Lion));
        assert!(Piece::Rat.beats(Piece::Elephant));
        assert!(Piece::Cat.beats(Piece::Rat));
        assert!(!(Piece::Elephant.beats(Piece::Rat)));
    }
}

fn main() {
    let b = Board::new();
    println!("{}", b);
}
