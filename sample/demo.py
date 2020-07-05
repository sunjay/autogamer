#!/usr/bin/env python3

from pyautogamer import *
from pyautogamer.ui import *

from demo_files.main_menu import MainMenu

def main():
    game = Game(window_width=800, window_height=600)

    main_menu = MainMenu(game)
    game.set_screen(main_menu)

    game.run()

if __name__ == "__main__":
    main()
