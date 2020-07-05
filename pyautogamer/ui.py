from . import Level

class Align:
    """
    Defines how a widget should be aligned with its position.

    Example: Align.N specifies that the top middle point of the widget rectangle
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

class Screen:
    def __init__(self, game):
        self.game = game

    def update(self):
        pass

    def render(self):
        pass

class LevelScreen(Screen):
    def __init__(self, game):
        super().__init__(game)

        self.level = Level()

class Widget:
    def __init__(self, *, x=0, y=0, width=1, height=1, align=Align.CENTER):
        self.x = x
        self.y = y
        self.width = width
        self.height = height
        self.align = align

    def draw(self):
        pass

    def destroy(self):
        pass

class Button(Widget):
    def __init__(self, text, *, onclick, **kwargs):
        super().__init__(**kwargs)

        self.text = text
        self.onclick = onclick

class Text(Widget):
    def __init__(self, text, **kwargs):
        super().__init__(**kwargs)

        self.text = text
