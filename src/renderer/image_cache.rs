use std::fmt;
use std::path::PathBuf;
use std::collections::HashMap;

use sdl2::{
    video::WindowContext,
    render::{Texture, TextureCreator},
    image::LoadTexture,
};

use crate::{SdlError, ImageParams};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ImageId(usize);

// Image data is loaded on-demand when the image is used the first time
//
// This avoids some overhead and unnecessarily allocation in cases where
// not all tileset images are actually used in the game (very common).
// The trade-off is that the initial render and any render that uses an
// image for the first time will be slower because of the time it takes to
// load images.
//TODO: Potentially provide a method to help warm the cache and allow the
// user to preload paths that they know will definitely be loaded at some
// point. Could also be a custom property in Tiled - `preload_image: true`
struct CachedImage {
    /// The path to the image file
    path: PathBuf,
    /// The raw image loaded from the path on-demand
    base_image: Option<Texture>,
    /// Cached versions of the image texture, with different parameters applied
    params_cache: HashMap<ImageParams, Texture>,
}

impl CachedImage {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            base_image: None,
            params_cache: HashMap::new(),
        }
    }

    pub fn load(
        &mut self,
        texture_creator: &mut TextureCreator<WindowContext>,
        params: ImageParams,
    ) -> Result<&mut Texture, SdlError> {
        if !self.params_cache.contains_key(&params) {
            let tex = self.base_image(texture_creator)?;
            //TODO: Use params to generate another image and insert it into the cache
            return Ok(tex)
        }

        // This unwrap is safe because the code above inserts the texture
        Ok(self.params_cache.get_mut(&params).unwrap())
    }

    pub fn base_image(
        &mut self,
        texture_creator: &mut TextureCreator<WindowContext>,
    ) -> Result<&mut Texture, SdlError> {
        if self.base_image.is_none() {
            let tex = texture_creator.load_texture(&self.path)?;
            self.base_image = Some(tex);
        }

        // This unwrap is safe because the code above loads the texture
        Ok(self.base_image.as_mut().unwrap())
    }

    pub fn invalidate(&mut self) {
        let CachedImage {
            path: _,
            base_image,
            params_cache,
        } = self;

        //TODO: Maybe this should call Texture::destroy()? In that case this
        // should be an unsafe method since you need to be able to guarantee
        // that the previous texture creator is still alive. The
        // set_texture_creator does not need to be unsafe because at that point
        // the previous texture creator is still alive (stored in self)
        *base_image = None;
        params_cache.clear();
    }
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
                self.images.push(CachedImage::new(path.clone()));
                self.image_paths.insert(path, id);
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

    /// Loads an image and generates a version that matches the given parameters
    ///
    /// The image is cached so no loading is necessary when the same image with
    /// the same parameters is loaded.
    pub fn load(&mut self, image: ImageId, params: ImageParams) -> Result<&mut Texture, SdlError> {
        let texture_creator = self.texture_creator.as_mut()
            .expect("attempt to load images before texture creator was setup");

        let ImageId(index) = image;
        self.images[index].load(texture_creator, params)
    }

    fn invalidate_all(&mut self) {
        // There is nothing to invalidate if there is no texture creator
        if self.texture_creator.is_none() {
            return;
        }

        for image in &mut self.images {
            image.invalidate();
        }
    }
}
