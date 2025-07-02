import pytest
from guessing_game import Game

@pytest.fixture
def game():
    """Provides a Game instance for testing."""
    return Game()

def test_guess_returns_string(game):
    """Tests that the guess method returns a string."""
    assert isinstance(game.guess(1), str)

def test_too_small_and_too_big(game):
    """Tests the 'Too small!' and 'Too big!' conditions."""
    secret_number = game.secret_number
    assert game.guess(secret_number - 1) == "Too small!"
    assert game.guess(secret_number + 1) == "Too big!"

def test_win(game):
    """Tests the winning condition."""
    secret_number = game.secret_number
    assert game.guess(secret_number) == "You win!"

def test_max_number():
    """Tests that the max_number is set correctly."""
    game = Game(max_number=200)
    assert game.max_number == 200
    assert game.secret_number <= 200