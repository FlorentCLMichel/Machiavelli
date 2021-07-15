# Machiavelli

*Work in progress—this project is still in early development*

This is a simple implementation of a Machiavelli-like card game in the terminal. 

## The game

Machiavelli is an Italian card game derived from Rummy. The rules can be found [here](https://gamerules.com/rules/machiavelli-card-game/).

There are two versions of the game: a single-terminal version and a client/server one. The first version is mostly designed for single player (because ~~that's the only way I can win at this game~~ using a single terminal is not well suited to multiplayer). The corresponding executable is called `machiavelli`. 

The client/server version consists (as you may have guessed) in two parts: a server and a client. The server should be lunched first; it sets up a TCP listener to which the client (one per player) can connect. 

By default, the game server loads the config from the `./Config/config.dat` file and connects to the port specified in `./Config/port_server.dat`. The client tries to connect to the address and port specified in `./Config/port_client.dat`. If one of these files is missing, or if an error occurs while parsing it, the server or client will ask for the information.

## Requirements

The game currently requires an ansi-compatible terminal (or terminal emulator) with support for true colours for the single-terminal version and for the client. The server can in principle run on any terminal.

## Build

To build this game, you need a Rust compiler (probably at least version 1.41.0; I tested it with rustc version 1.51.0). If you have cargo installed, you may build it by running `cargo build --release` or `make release`. (The second option requires that all the prerequisite crates are already installed; the firt one will install them automatically if they are not.) The executables can be found in the folder `target/release`. 

## To do

* Implement the save and load functionalities in the client/server version.
* Deal with a player disconnecting mid-game (at the moment, this makes the server and other clients crash).
* Test the client/server version on distant machines (so far, all tests were done with the server and clients running on the same one).
* Make more tests.
