# Jungle-Chess

> This is my first project written in Rust. Happy for contributors and feedback! The code is **dirty**.

Play Jungle Chess on an Emoji-Enabled Linux Terminal!

![terminal screenshot](screenshot.png)

## How to play

Each player tries to either: 

    * enter their opponent's Den (🏠). 
    * capture all opponent's pieces.

Player's play in turns. Each piece can move freely on the ground.

### Capturing Pieces

Every piece can capture another piece of equal rank.

In order to capture a piece, the following hierarchy holds:

    * Elephant (🐘)
    * Lion (🦁)
    * Tiger (🐯)
    * Leopard (🐆)
    * Wolf (🐺)
    * Dog (🐕)
    * Cat (🐱)
    * Rat (🐭)

Whereas the Elephant cannot capture the Rat (🐭) , but the Rat (🐭) captures the Elephant (🐘).

The Rat (🐭) can not move to capture an Elephant (🐘) while emerging from water (🟦) (see #making-a-move). 

Note, however, that if a piece is on an opponent's Trap (🥅), it can be captured by *any* piece.


### Making a move (#making-a-move)

Every piece moves freely on the grass (🟩).

The Rat (🐭) is the only piece that can enter water (🟦).

The Lion (🦁) and Tiger (🐯) can jump across the water (🟦) both horizontally and vertically, but only if the line is not blocked by a Rat (🐭).

No Piece can enter their player's den(🏠).

Naturally, a piece can only move onto an occupied square, if it can capture the occupant.
