use std::path::PathBuf;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(usize);

#[derive(Debug, Default)]
struct CachedImage {
    /// Image data is loaded on-demand when the image is used the first time
    ///
    /// This avoids some overhead and unnecessarily allocation in cases where
    /// not all tileset images are actually used in the game (very common).
    /// The trade-off is that the initial render and any render that uses an
    /// image for the first time will be slower because of the time it takes to
    /// load images.
    //TODO: Potentially provide a method to help warm the cache and allow the
    // user to preload paths that they know will definitely be loaded at some
    // point.
    image_data: Option<(/*TODO*/)>
}

#[derive(Debug, Default)]
pub struct ImageCache {
    /// Map from the canonical path of an image file to its image ID
    image_paths: HashMap<PathBuf, ImageId>,
    /// The value in `ImageId` indexes into this field
    images: Vec<CachedImage>,
}

impl ImageCache {
    /// Adds an image to the cache and returns its ID. The image is not loaded
    /// until it is first used by the renderer.
    ///
    /// If the image is already in the cache, its existing ID is returned.
    pub fn add<P: Into<PathBuf>>(&mut self, path: P) -> ImageId {
        let path = path.into();
        match self.image_paths.get(&path) {
            Some(&id) => id,
            None => {
                let id = ImageId(self.images.len());
                self.images.push(CachedImage::default());
                id
            },
        }
    }
}
