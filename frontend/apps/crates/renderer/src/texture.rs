/*
    Texture loading from a url is async
    Therefore the id_cache is in a RefCell
    Otherwise the parent would need to keep a mutable ref
    During the lifetime of the fetch
    And that would panic while the renderer is used for other things
    It's a good idea to keep the texture's Id around after loading
    Which avoids the inner cache lookup/borrow
*/
use crate::prelude::*;
use std::cell::RefCell;
use web_sys::HtmlImageElement;
use std::{collections::HashMap, error::Error};
use awsm_web::{
    loaders,
    webgl::{
        Id,
        TextureTarget, 
        SimpleTextureOptions, 
        PixelFormat, 
        WebGlTextureSource
    }
};

pub struct Textures {
    pub id_cache: RefCell<HashMap<String, Id>>,
}

impl Textures {
    pub fn new() -> Self {
        Self {
            id_cache: RefCell::new(HashMap::new())
        }
    }

    pub fn get_id(&self, url:&str) -> Option<Id> {
        self.id_cache.borrow().get(url).copied()
    }

    pub fn set_id(&self, url:String, id:Id) {
        self.id_cache.borrow_mut().insert(url, id);
    }
}

impl Renderer {
    pub async fn load_texture(&self, url:String) -> Result<Id, awsm_web::errors::Error> {
        match self.textures.get_id(&url) {
            Some(id) => Ok(id),
            None => {
                let img = loaders::image::load(url.clone()).await?;
                let mut webgl = self.world.borrow::<GlMut>().unwrap();
                let id = webgl.create_texture()?;
                webgl.assign_simple_texture(
                    id,
                    TextureTarget::Texture2d,
                    &SimpleTextureOptions {
                        pixel_format: PixelFormat::Rgba,
                        ..SimpleTextureOptions::default()
                    },
                    &WebGlTextureSource::ImageElement(&img),
                )?;

                self.textures.set_id(url, id);
                
                Ok(id)
            }
        }
    }
}
