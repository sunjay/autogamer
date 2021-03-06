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
        self.level.physics.set_gravity((0.0, 700))

        player_spritesheet = CharacterSpritesheet(
            "images/character/female.png",
            "images/character/character.json",
        )
        player_sprites = self.level.load_sprites(player_spritesheet)

        # Add a player to the game
        player = self.level.add_player()
        player.add(PlatformerControls(
            horizontal_velocity=250.0,
            jump_velocity=-450.0,
            midair_horizontal_multiplier=0.7,
        ))
        player.add(PhysicsBody(mass=40.0))
        player.add(PhysicsCollider(
            shape=ShapeRect(width=70.0, height=80.0),
            offset=(-35.0, -75.0),
            collision_groups=PLAYER_COLLISION_GROUPS,
        ))
        player.add(Health(6))
        player.add(Wallet())
        player.add(ViewportTarget())
        player.add(player_sprites.default_sprite())
        player.add(player_sprites)

        self.level.set_viewport_dimensions(
            width=level1_map.tile_width * 16,
            height=level1_map.tile_height * 12,
        )

        #TODO: Configure HUD and menu
        #TODO: Configure pause menu keyboard shortcut
