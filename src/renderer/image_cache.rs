use std::fmt;
use std::path::PathBuf;
use std::collections::HashMap;
use sdl2::{video::WindowContext, render::{Texture, TextureCreator}};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(usize);

#[derive(Default)]
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
    image_data: Option<Texture>,
}

#[derive(Default)]
pub struct ImageCache {
    texture_creator: Option<TextureCreator<WindowContext>>,
    /// Map from the canonical path of an image file to its image ID
    image_paths: HashMap<PathBuf, ImageId>,
    /// The value in `ImageId` indexes into this field
    images: Vec<CachedImage>,
}

impl fmt::Debug for ImageCache {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            texture_creator: _,
            image_paths,
            images: _,
        } = self;

        f.debug_struct("Level")
            .field("texture_creator", &"TextureCreator { ... }")
            .field("image_paths", &image_paths)
            .field("images", &"[...]")
            .finish()
    }
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

    /// Updates the texture creator used to load images
    ///
    /// This invalidates all cached images
    pub fn set_texture_creator(&mut self, texture_creator: TextureCreator<WindowContext>) {
        self.invalidate_all();

        self.texture_creator = Some(texture_creator);
    }

    fn invalidate_all(&mut self) {
        // There is nothing to invalidate if there is no texture creator
        if self.texture_creator.is_none() {
            return;
        }

        for image in &mut self.images {
            let CachedImage {image_data} = image;

            //TODO: Maybe this should call Texture::destroy()? In that case this
            // should be an unsafe method since you need to be able to guarantee
            // that the previous texture creator is still alive. The
            // set_texture_creator does not need to be unsafe because at that
            // point the previous texture creator is still alive (stored in self)
            *image_data = None;
        }
    }
}
