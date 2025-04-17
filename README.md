# TicTacToe: A Playdate game built in Rust

<a href='https://www.recurse.com/scout/click?t=c7bc9ba4cb3e6725e05e413f16f8c5a3' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a>

This is a version of the classic TicTacToe game that runs on the [Playdate](https://play.date) handheld gaming console. All of the code is written in Rust courtesy of the unofficial [crankstart](https://crates.io/crates/crankstart) library. It has only been tested on the Playdate simulator.

![level 1 cover screen](docs/1.png) ![level 1 sample play](docs/2.png)

## Game Levels

This version of TicTacToe has 7 increasingly chaotic levels as follows. Levels marked ✅ have been implemented, ❌ have not yet been implemented:

| level | level name                             | rules                                                                                                                                               |
|-------|----------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| 1 ✅   | **basic tictactoe**                    | computer selects a move randomly, although it almost never picks an optimal or sensible move                                                        |
| 2 ✅   | **reverse tictactoe**                  | reverse of basic game play, where player loses if they get any 3 in a row                                                                           |
| 3 ✅   | **forgiving disappearing tictactoe**   | same rules as basic game play, except each player's entry disappears from the screen on the next turn; the selected slot is remembered by the game  |
| 4 ✅   | **unforgiving disappearing tictactoe** | same as level 3, except if you try to play in a cell already filled you lose                                                                        |
| 5 ✅   | **reverse disappearing tictactoe**     | levels 2 and 3 combined                                                                                                                             |
| 6 ✅   | **rearranging tictactoe**              | same rules as basic game play, except every turn, the x's and o's are shuffled                                                                      |
| 7 ✅   | **murder tictactoe**                   | same rules as basic game play, except the human player can choose to override the computer's entry instead of playing their turn                    |
| 8 ❌   | **extra hard tictactoe**               | the computer employs the unbeatable [minimax algorithm](https://www.baeldung.com/java-minimax-algorithm) and will almost always win or tie the game |

**Note**: at the end of each game, the player can choose to repeat the same level or advance to the next level
regardless of game status (win, lose or tie). Currently, scores are not tallied.

## Running the game

### Pre-requisites

You'll need to be set up for Playdate game development with Rust:
1. Install the Playdate SDK at https://play.date/dev. The SDK comes with the Playdate simulator.
2. Install the `crank` command line tool by following the instructions on https://github.com/pd-rs/crank.
3. Clone or download this project to your computer, then update the location of the `crankstart` and `crankstart-sys` dependencies in [Cargo.toml](Cargo.toml) to either point to the released dependencies on [crates.io](https://crates.io) or your local copy if you're using that. 

### Playing the game
To play the game on the Playdate simulator, run the following command from the project root directory:

```bash
crank run --release
```