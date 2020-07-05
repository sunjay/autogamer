from pyautogamer import *
from pyautogamer.ui import *

__all__ = ['PlayScreen']

class PlayScreen(LevelScreen):
    def __init__(self, game):
        super().__init__(game)

        # Load the map
        level1_map = TileMap("levels/level1.tmx")
        self.level.load(level1_map)

        # Configure gravity in the physics engine
        self.level.physics().set_gravity((0.0, -9.81))

        # Add a player to the game
        player = self.level.add_player()
        player.add(PlatformerControls(
            left_velocity=10.0,
            right_velocity=10.0,
            jump_velocity=20.0,
        ))
        player.add(Health(100))
        player.add(ViewportTarget())

        self.level.viewport(
            width=level1_map.tile_width() * 16,
            height=level1_map.tile_height() * 12,
        )

        hud = HeadsUpDisplay()
        self.level.add(hud)
        hud.add(HealthBar())

        menu = Menu()
        hud.add(menu)

        pause_menu = Menu()
        self.level.on_key_pressed("p", lambda: pause_menu.show())

        # To move to the next level
        health = player.get(Health)
        level2_player = level2.player()
        level2_player.add(health)
