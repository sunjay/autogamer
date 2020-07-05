#!/usr/bin/env python3

from pyautogamer import *
from pyautogamer.ui import *

#from demo_files.main_menu import MainMenu
from demo_files.play_screen import PlayScreen

def main():
    game = Game(window_width=800, window_height=600)

    #init_screen = MainMenu(game)
    init_screen = PlayScreen(game)
    game.set_screen(init_screen)

    game.run()

if __name__ == "__main__":
    main()
