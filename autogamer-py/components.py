class PlatformerControls:
    __autogamer_component__ = "PlatformerControls"

    def __init__(self, *, left_velocity, right_velocity, jump_velocity):
        self.left_velocity = left_velocity
        self.right_velocity = right_velocity
        self.jump_velocity = jump_velocity

class Health:
    __autogamer_component__ = "Health"

    def __init__(self, initial_health):
        self.health = initial_health

class FollowCamera:
    __autogamer_component__ = "FollowCamera"
