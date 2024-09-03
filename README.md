# Guessing Game in Rust

## Overview

This is a simple command-line guessing game written in Rust. The program generates a random number between 1 and 10, and the user has to guess the number. The game provides feedback on whether the guess is correct, greater than, or less than the generated number. The user can exit the game at any time by entering `0`.

## Features

- Generates a random number between 1 and 10.
- Accepts user input for guesses.
- Provides feedback on whether the guess is correct, greater than, or less than the generated number.
- Allows the user to exit the game by entering `0`.

## How to Run

1. **Clone the repository:**
    ```sh
    git clone https://github.com/yourusername/guessing_game.git
    cd guessing_game
    ```

2. **Build the project:**
    ```sh
    cargo build
    ```

3. **Run the project:**
    ```sh
    cargo run
    ```

## Code Structure

### `main.rs`

The `main.rs` file contains the main loop of the game. It handles user input, generates the random number, and provides feedback to the user.

### `lib.rs`

The `lib.rs` file contains the core functions of the game, including generating the random number, asking for the user’s guess, and comparing the guess to the generated number.



## Contributing
Feel free to fork this repository and submit pull requests. For major changes, please open an issue first to discuss what you would like to change.

## License
This project is licensed under the MIT License. See the LICENSE file for details.

## Acknowledgements
Rust Programming Language
rand crate