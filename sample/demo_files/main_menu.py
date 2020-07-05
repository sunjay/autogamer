from pyautogamer.ui import *

from .play_screen import PlayScreen

__all__ = ['MainMenu']

class MainMenu(Screen):
    def update(self, events):
        if start_button_pressed():
            self.game.set_screen(PlayScreen(self.game))
            return

    def render(self, renderer):
        width = renderer.width()
        height = renderer.height()

        start_text = Text(renderer, "Start")
        # Extra space after the start text
        start_margin = 20
        renderer.draw_text(
            start_text,
            x=width/2,
            y=height/2 - start_text.height - start_margin,
            anchor=Anchor.CENTER,
        )
        self.start_text_button = Rect.from_center()

        renderer.draw_text(
            "Quit"
        )
