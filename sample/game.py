#!/usr/bin/env python3

from pyautogamer import *
from pyautogamer.ui import *

from screens.main_menu import MainMenu

def main():
    game = Game()

    main_menu = MainMenu(game)
    game.set_screen(main_menu)

    game.fullscreen()
    game.run()

if __name__ == "__main__":
    main()
