# Machiavelli

*Work in progress—this project is still in early development*

This is a simple implementation of a Machiavelli-like card game in the terminal. 

## The game

Machiavelli is an Italian card game derived from Rummy. The rules can be found [here](https://gamerules.com/rules/machiavelli-card-game/).

At the moment, only a one-player version of the game is implemented (because ~~that's the only way I can win at this game~~ the current design is not well-suited for multiplayer). 

*Update:* Technically, multiplayer is now implemented. However, each player can see the other player' cards, which is far from optimal... A proper multiplayer version is yet to be implemented.

## Requirements

The game currently requires an ansi-compatible terminal (or terminal emulator) with support for true colours.

## Build

To build this game, you need a Rust compiler (probably at least version 1.41.0; I tested it with rustc version 1.51.0). If you have cargo installed, you may build it by running `cargo build --release` or `make release`. (The second option requires that all the prerequisite crates are already installed; the firt one will install them automatically if they are not.) The executable can be found in the folder `target/release`. 
