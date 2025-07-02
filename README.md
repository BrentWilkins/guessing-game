# Guessing Game

A simple guessing game library written in Rust and exposed to Python.

## Installation

```bash
pip install guessing-game
```

## Usage

```python
import guessing_game

# Create a game with a maximum number of 200
game = guessing_game.Game(max_number=200)

while True:
    try:
        guess = int(input(f"Guess a number between 1 and {game.max_number}: "))
        result = game.guess(guess)
        print(result)
        if result == "You win!":
            break
    except ValueError:
        print("Invalid input. Please enter a number.")
```

## Development

This project uses [maturin](https://www.maturin.rs/) to build the Rust extension.

To build the project, you'll need to have Rust and Python installed. Then, you can install the project in editable mode:

```bash
maturin develop
```

To run the tests:

```bash
pytest
```

## License

This project is dual-licensed under either:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.
