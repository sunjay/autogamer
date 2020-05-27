# autogamer

Welcome to autogamer! This is an opinionated, convention over configuration game
creation framework designed for use with the [Tiled editor] and the [Python]
programming language.

## Getting Started

autogamer requires very little setup to get started:

1. Make sure you have [Python] and autogamer installed
2. Create a file called `game.py` with the following code:
  ```py
  #!/usr/bin/env python3

  from autogamer import *

  def main():
      game = Game()

      level = Map("path/to/your/tiled_map.tmx")
      game.add(level)

      game.fullscreen()
      game.run()

  if __name__ == '__main__':
      main()
  ```
3. Make sure you replace `path/to/your/tiled_map.tmx` with the path to the map
   you've created in the Tiled editor
4. Run the `game.py` Python script (hit the Esc key to exit the game)

That's it! You now have a running version of your first game. :tada:

## Layers

autogamer uses the names of the layers in your map to interpret what they should
be used for. Names are case-sensitive and should not have any leading or
trailing whitespace. Layers are always rendered in the order specified in Tiled.

### The `map` Layer

All of the static tiles in your level should go on a [Tile Layer] named `map`.
The `map` layer tiles will be used in physics calculations. The player, enemies
and other entities in your game will not be able to pass through a tile on the
`map` layer by default.

### The `markers` Layer

An [Object Layer] called `markers` is used to annotate the level with additional
information.

* A point object named `level_start` indicates where the player should begin
  when the game/level is first initialized

### Other Layers

A layer without one of the names listed above is drawn as-is, with no further
interpretation by autogamer. No player, enemy, etc. can interact with tiles on
those layers. Most games have at least several such layers for things like the
background, foliage, decorations, etc.

[Tiled editor]: https://www.mapeditor.org
[Python]: https://www.python.org

[Tile Layer]: https://doc.mapeditor.org/en/stable/manual/layers/#tile-layers
[Object Layer]: https://doc.mapeditor.org/en/stable/manual/layers/#object-layers
