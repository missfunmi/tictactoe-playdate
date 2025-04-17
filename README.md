# TicTacToe: A Playdate game built in Rust

<a href='https://www.recurse.com/scout/click?t=c7bc9ba4cb3e6725e05e413f16f8c5a3' title='Made with love at the Recurse Center'><img src='https://cloud.githubusercontent.com/assets/2883345/11325206/336ea5f4-9150-11e5-9e90-d86ad31993d8.png' height='20px'/></a>

This is a version of the classic TicTacToe game that runs on the [Playdate](https://play.date) handheld gaming console. All of the code is written in Rust courtesy of the unofficial [crankstart](https://crates.io/crates/crankstart) library. It has only been tested on the Playdate simulator.

## Game Levels

This version of TicTacToe has 7 increasingly chaotic levels as follows. Levels marked ✅ have been implemented, ❌ have not yet been implemented

| level | level name                             | rules                                                                                                                                               |
|-------|----------------------------------------|-----------------------------------------------------------------------------------------------------------------------------------------------------|
| 1 ✅   | **basic tictactoe**                    | computer selects a move randomly, although it almost never picks an optimal or sensible move                                                        |
| 2 ✅   | **reverse tictactoe**                  | reverse of basic game play, where player loses if they get any 3 in a row                                                                           |
| 3 ✅   | **forgiving disappearing tictactoe**   | same rules as basic game play, except each player's entry disappears from the screen on the next turn; the selected slot is remembered by the game  |
| 4 ✅   | **unforgiving disappearing tictactoe** | same as level 3, except if you try to play in a cell already filled you lose                                                                        |
| 5 ✅   | **reverse disappearing tictactoe**     | levels 2 and 3 combined                                                                                                                             |
| 6 ✅   | **rearranging tictactoe**              | same rules as basic game play, except every turn, the x's and o's are shuffled                                                                      |
| 7 ✅   | **murder tictactoe**                   | same rules as basic game play, except the human player gets one chance per game to override the computer's entry                                    |
| 8 ❌   | **extra hard tictactoe**               | the computer employs the unbeatable [minimax algorithm](https://www.baeldung.com/java-minimax-algorithm) and will almost always win or tie the game |

**Note**: at the end of each game, the player can choose to repeat the same level or advance to the next level
regardless of game status (win, lose or tie).

## Running the game

### Pre-requisites
You'll need to be set up for Playdate game development with Rust -- follow the instructions on [crankstart](https://github.com/pd-rs/crankstart) if you haven't yet done so. Then update the location of the [crankstart](https://crates.io/crates/crankstart) and [crankstart-sys](https://crates.io/crates/crankstart-sys) crates specified in this project's dependencies in [Cargo.toml](Cargo.toml) if they differ from yours. The public crates should **probably** work (I haven't tested that yet); I made some modifications to those libraries locally that I haven't had a chance to submit PR's for yet.

### Playing the game
To play the game on the Playdate simulator, run the following command from the project root directory:

```bash
crank run --release
```