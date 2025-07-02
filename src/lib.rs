use pyo3::prelude::*;
use rand::Rng;
use std::cmp::Ordering;

/// A simple guessing game library.
///
/// This library provides a `Game` class that allows you to play a guessing game.
/// You can create a new game, guess a number, and get feedback on your guess.
#[pyclass(name = "Game")]
struct Game {
    /// The secret number to guess.
    #[pyo3(get)]
    secret_number: u32,
    /// The maximum number for the guessing game.
    #[pyo3(get)]
    max_number: u32,
}

#[pymethods]
impl Game {
    /// Creates a new game with a random secret number.
    ///
    /// Args:
    ///     max_number (int, optional): The maximum number for the game. Defaults to 100.
    #[new]
    #[pyo3(signature = (max_number = 100))]
    fn new(max_number: Option<u32>) -> Self {
        let max = max_number.unwrap_or(100);
        let secret_number = rand::rng().random_range(1..=max);
        Game {
            secret_number,
            max_number: max,
        }
    }

    /// Guesses a number and returns a hint.
    ///
    /// Args:
    ///     guess (u32): The number to guess.
    ///
    /// Returns:
    ///     str: A string indicating whether the guess was "Too small!", "Too big!", or "You win!".
    fn guess(&self, guess: u32) -> String {
        match guess.cmp(&self.secret_number) {
            Ordering::Less => "Too small!".to_string(),
            Ordering::Greater => "Too big!".to_string(),
            Ordering::Equal => "You win!".to_string(),
        }
    }
}

/// A Python module for playing a guessing game.
#[pymodule]
fn guessing_game(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Game>()?;
    Ok(())
}
