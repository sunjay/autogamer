from . import Level

class Anchor:
    """
    Defines how a widget should be aligned with its position.

    Example: Anchor.N specifies that the top middle point of the widget rectangle
    will be equal to the widget position.
    """

    # Align north (top)
    N = 0
    # Align south (bottom)
    S = 1
    # Align east (right)
    E = 2
    # Align west (left)
    W = 3
    # Align north east (top-right)
    NE = 4
    # Align south east (bottom-right)
    SE = 5
    # Align south west (bottom-left)
    SW = 6
    # Align north west (top-left)
    NW = 7
    # Align center
    CENTER = 8

class EventLoopControl:
    """
    Returned from Screen.update to specify when the event loop should rerun that
    function.
    """

    # The event loop will wait until the next frame to run Screen.update
    # This is the default if nothing is returned from Screen.update
    WAIT_FOR_FRAME = 0
    # The event loop will immediately rerun Screen.update on the next iteration
    CONTINUE = 1
    # The event loop will exit and the window will be closed
    EXIT = 2

class Screen:
    def __init__(self, game):
        self.game = game

    def update(self, events):
        pass

    def draw(self, renderer):
        pass

class LevelScreen(Screen):
    def __init__(self, game):
        super().__init__(game)

        self.level = Level()
        self.hud = None

    def update(self, events):
        control = None
        if self.hud is not None:
            # HUD needs to process events before level so HUD controls can take
            # precedence
            control = self.hud.update(events)
        control2 = self.level.update(events)

        if control2 is not None:
            return control2
        return control

    def draw(self, renderer):
        self.level.draw(renderer)
        if self.hud is not None:
            self.hud.draw(renderer)

class Text:
    def __init__(self, renderer, content):
        self.renderer = renderer
        self.content = content
        self.width = None
        self.height = None

    def measure(self):
        #TODO: Cache the text in the renderer as a texture and return the
        # dimensions of the text
        #TODO: Cache the dimensions in self.width and self.height
        return (200, 40)

    @property
    def width(self):
        self.measure()
        return self.width

    @property
    def height(self):
        self.measure()
        return self.height

class Rect:
    def __init__(self, x, y, width, height):
        self.x = x
        self.y = y
        self.width = width
        self.height = height

    @staticmethod
    def from_center(center_x, center_y, width, height):
        pass
