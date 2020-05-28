#!/usr/bin/env python3

from pyautogamer import *

def main():
    game = Game()

    level = Map("levels/candy.tmx")
    game.add(level)

    game.fullscreen()
    game.run()

if __name__ == "__main__":
    main()
