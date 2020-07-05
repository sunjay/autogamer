class Player:
    """
    A marker component given to an entity to indicate that it represents one of
    the players of the game. This component is automatically added when you call
    `Game.add_player`.
    """

    __autogamer_component__ = "Player"

class Position:
    """
    The position of an entity
    """

    __autogamer_component__ = "Position"

    def __init__(self, *, x=0.0, y=0.0):
        self.x = x
        self.y = y

class PlatformerControls:
    """
    An entity with this component will respond to arrow key presses by setting
    its velocity to the configured values. `left_velocity` and `right_velocity`
    will be applied to the x-axis velocity. `jump_velocity` will be applied to
    the y-axis velocity.
    """

    __autogamer_component__ = "PlatformerControls"

    def __init__(self, *, left_velocity, right_velocity, jump_velocity):
        self.left_velocity = left_velocity
        self.right_velocity = right_velocity
        self.jump_velocity = jump_velocity

class Health:
    """
    The health of an entity
    """

    __autogamer_component__ = "Health"

    def __init__(self, initial_health):
        self.health = initial_health

class ViewportTarget:
    """
    If an entity is given this component, the viewport will attempt to center
    itself around the position of the entity.

    Warning: Multiple entities should not have this component at the same time.
    """

    __autogamer_component__ = "ViewportTarget"
