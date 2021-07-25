# Machiavelli

*Work in progress—this project is still in early development*

This is a simple implementation of a Machiavelli-like card game in the terminal. 

## The game

Machiavelli is an Italian card game derived from Rummy. The rules can be found [here](https://gamerules.com/rules/machiavelli-card-game/).

This implementation has an optional custom rules that jokers can not be kept: if you have a joker in your hand, you can nor pick a card not pass until you have played it. The reason is that, wihout this rule, it seems that keeping jokers in one's hand until late game is often more advantageous than playing them fast, reducing the fluidity of the game. Forcing a player with a joker in their hand play it immediately, thus making it available to other players, can make the game more dynamic and fun. (Obviously, this rule has no effect if the number of jokers is set to 0.)

## Single-terminal and client/server versions

There are two versions of the game: a single-terminal version and a client/server one. The first version is mostly designed for single player (because ~~that's the only way I can win at this game~~ using a single terminal is not well suited to multiplayer). The corresponding executable is called `machiavelli`. 

The client/server version consists (as you may have guessed) in two parts: a server and a client. The server should be lunched first; it sets up a TCP listener to which the client (one per player) can connect. The TCP listener is closed and the game starts when the required number of players have joined. 

There are a few small other differences between the two versions: 

* The order in which players play is fixed in the first one while the first player is chosen (pseudo-)randomly in the second one.
* The first one has an option to save the game while the second one does it automatically at the start of each turn.
* Some of the improvements to the second version have not been ported to the first one. They are only convenience changes, thought, ad do not affect the game rules. 

The client has one optional command-line argument: the name of the player.
The server has two optional arguments: 

* the first one tells whether a previous game should be loaded (‘1’ for ‘yes’, anything else for ‘no’),
* the second one is the name of the save file (if empty, the default name is used).

## Config files

By default, the game server loads the config from the `./Config/config.dat` file and connects to the port specified in `./Config/port_server.dat`. The client tries to connect to the address and port specified in `./Config/port_client.dat`. If one of these files is missing, or if an error occurs while parsing it, the server or client will ask for the corresponding information. 

The config file encodes the game settings in plaintext on a line by line basis, ignoring the first line:

* number of decks 
* number of jokers
* number of cards each player starts with
* whether the custom rule should be used (`1` for yes and `0` for no)
* number of players
* name of the save file (without the `.sav` extension)

## Requirements

The game currently requires an ansi-compatible terminal (or terminal emulator) for the single-terminal version and for the client. The server can in principle run on any terminal.

## Build

To build this game, you need a Rust compiler (probably at least version 1.41.0; I tested it with rustc version 1.51.0). If you have cargo installed, you may build it by running `cargo build --release` or `make release`. (The second option requires that all the prerequisite crates are already installed; the firt one will install them automatically if they are not.) The executables can be found in the folder `target/release`. 

## To do

* Add missing comments and unit tests.
* Replace `unwrap`s by proper error handling.
* Properly deal with player disconnection mid-game.
* Fully multi-threaded server.
* Allow players to sort their cards out of turn.
* Implement a simple chat function.
* Ensure the game works properly on different terminal emulators.
* Do more tests.
