#!/usr/bin/env python3

from autogamer import *

def main():
    game = Game()

    level1 = Map("levels/candy.tmx")
    game.add(level1)
    level2 = Map("levels/Level 2.tmx")
    game.add(level2)

    level1.set_gravity((0.0, -9.81))

    player = level1.player()
    player.add(PlatformerControls(
        left_velocity=10,
        right_velocity=10,
        jump_velocity=20,
    ))
    player.add(Health(100))
    player.add(FollowCamera())

    hud = HeadsUpDisplay()
    hud.add(HealthBar())

    menu = Menu()
    hud.add(menu)

    pause_menu = Menu()
    game.on_key_pressed("p", lambda: pause_menu.show())

    # To move to the next level
    health = player.get(Health)
    level2_player = level2.player()
    level2_player.add(health)

    game.fullscreen()
    game.run()

if __name__ == "__main__":
    main()
