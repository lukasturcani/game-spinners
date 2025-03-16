import pytest
import py_game_spinners


def test_sum_as_string():
    assert py_game_spinners.sum_as_string(1, 1) == "2"
