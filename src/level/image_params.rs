#[derive(Debug, Clone, Copy)]
pub(in super) struct TiledImageParams {
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub flip_diagonal: bool,
}

#[derive(Debug, Default, Clone, Copy)]
pub(in super) struct ImageFlipRot {
    pub flip_horizontal: bool,
    pub flip_vertical: bool,
    pub angle: f64,
}

impl TiledImageParams {
    /// Maps the tiled flip parameters to the flip and rotation parameters used
    /// in this engine.
    pub fn normalize(self) -> ImageFlipRot {
        // When rendering, the diagonal flip (x/y axis swap) is done first,
        // followed by the horizontal and vertical flips.
        //
        // See: https://doc.mapeditor.org/en/stable/reference/tmx-map-format/#tile-flipping
        let Self {flip_horizontal, flip_vertical, flip_diagonal} = self;

        // See docs on `ImageParams`
        //
        // Based on: https://ket.ketandkat.com/Software%20Development/Implementing%20Tiled%20Map%20Editor%28TMX%29%20Rotation%20and%20Flipping/
        // Source: https://github.com/theulings/Tiled-Map-Flip-Rotate-Example/blob/master/src/example.cpp
        //
        // Note that the code in the links above doesn't adequately account for
        // the effects of flipping a non-symmetrical sprite. The code below
        // fixes that and should work in all cases for any sprite.
        match (flip_horizontal, flip_vertical, flip_diagonal) {
            (false, false, false) => ImageFlipRot::default(),

            (true, false, false) => ImageFlipRot {
                flip_horizontal: true,
                ..ImageFlipRot::default()
            },

            (false, true, false) => ImageFlipRot {
                flip_vertical: true,
                ..ImageFlipRot::default()
            },

            (false, false, true) => ImageFlipRot {
                flip_horizontal: true,
                angle: 270.0,
                ..ImageFlipRot::default()
            },

            (true, true, false) => ImageFlipRot {
                flip_horizontal: true,
                flip_vertical: true,
                ..ImageFlipRot::default()
            },

            (true, false, true) => ImageFlipRot {
                angle: 90.0,
                ..ImageFlipRot::default()
            },

            (false, true, true) => ImageFlipRot {
                angle: 270.0,
                ..ImageFlipRot::default()
            },

            (true, true, true) => ImageFlipRot {
                flip_vertical: true,
                angle: 270.0,
                ..ImageFlipRot::default()
            },
        }
    }
}
