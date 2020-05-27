# autogamer

Welcome to autogamer! This is an opinionated, convention over configuration game
creation framework designed for use with the [Tiled editor] and the [Python]
programming language.

* autogamer is *opinionated* in that it is not designed to support all use cases
  or all possible types of games. It comes with a few nice defaults and then
  expects you to use the Python API to fill in the rest.

* autogamer favors *convention over configuration*. That means that if you
  follow the documented conventions, you will have to do very little programming
  to create your game. You may still have to do *some* programming, but
  autogamer is designed to do a lot of the work for you when it can.

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

  if __name__ == "__main__":
      main()
  ```
3. Make sure you replace `path/to/your/tiled_map.tmx` with the path to the map
   you've created in the Tiled editor
4. Run the `game.py` Python script (hit the Esc key to exit the game)

That's it! You now have a running version of your first game. :tada:

The rest of this documentation goes into detail about how you can make your game
interactive, add a player/goal, etc. Have fun making games! :video_game:

## Limitations

The Tiled editor is a large piece of software with many different features. The
autogamer framework does **not** support all of those features. autogamer will
do its best to interpret your map and use as much information as it can. When it
can't interpret parts of your map, it will do its best to at least draw them on
the screen. In cases where it can't even draw the information, it will simply
*ignore* it.

That means that it is possible for you to use autogamer in a way you think
*should* work, but simply isn't supported yet. You may even see the tiles in
your map on the screen but then observe that they don't behave the way you want
them to. This is unfortunate, and we are working to eliminate as many of these
cases as we can. If you find something that you think should be supported but
isn't for some reason, please report it so we can take a look!

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

### Unsupported Layers

Currently, autogamer does not support [Image Layers] or [Group Layers]. Support
for those layer types may be added in the future if requested.

### Other Layers

A [Tile Layer] without one of the names listed above is drawn as-is, with no
further interpretation by autogamer. No player, enemy, etc. can interact with
tiles on those layers. Most games have at least several such layers for things
like the background, foliage, decorations, etc.

[Tiled editor]: https://www.mapeditor.org
[Python]: https://www.python.org

[Tile Layer]: https://doc.mapeditor.org/en/stable/manual/layers/#tile-layers
[Object Layer]: https://doc.mapeditor.org/en/stable/manual/layers/#object-layers
[Image Layers]: https://doc.mapeditor.org/en/stable/manual/layers/#image-layers
[Group Layers]: https://doc.mapeditor.org/en/stable/manual/layers/#group-layers
