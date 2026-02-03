use image::GenericImageView;

pub struct Texture {
    id: u32,
    width: u32,
    height: u32
}

impl Texture {
    pub fn load(path: &str) -> Result<Texture, String> {
        let img = image::open(path).map_err(|e| format!("Failed to load image: {}", e))?;
        
        let mut id = 0;
        let (width, height) = img.dimensions();
        let data = img.to_rgba8().into_raw();

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);
            
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, width as i32, height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, data.as_ptr() as *const _);
            
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
        
        Ok(Texture { id, width, height })
    }
    
    pub fn bind(&self, texture_id: u32) {
        unsafe {
            gl::ActiveTexture(texture_id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::DeleteTextures(1, &self.id);
        }
    }

    pub fn get_texture_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}
