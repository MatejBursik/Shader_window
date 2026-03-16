use image::GenericImageView;
use std::ffi::c_void;
use std::ptr::null;

#[derive(Clone)]
pub struct Texture {
    id: u32,
    width: i32,
    height: i32
}

impl Texture {
    pub fn load_file(path: &str) -> Result<Texture, String> {
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
        
        Ok(Texture { id, width: width as i32, height: height as i32 })
    }

    pub fn empty(width: i32, height: i32) -> Result<Texture, String> {
        let mut id = 0;

        unsafe {
            gl::GenTextures(1, &mut id);
            gl::BindTexture(gl::TEXTURE_2D, id);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

            // allocate but do NOT upload data
            gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA8 as i32, width, height, 0, gl::BGRA, gl::UNSIGNED_BYTE, null());

            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Ok(Texture { id, width, height })
    }

    pub fn update_from_ptr(&self, size: (usize, usize), pos: (usize, usize), stride: usize, frame_ptr: *const c_void) {
        let (width, height) = size;
        let (x, y) = pos;

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.id);

            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, stride as i32 / 4);
            gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, x as i32);
            gl::PixelStorei(gl::UNPACK_SKIP_ROWS, y as i32);

            gl::TexSubImage2D(gl::TEXTURE_2D, 0, 0, 0, width as i32, height as i32, gl::BGRA, gl::UNSIGNED_BYTE, frame_ptr);

            gl::PixelStorei(gl::UNPACK_ROW_LENGTH, 0);
            gl::PixelStorei(gl::UNPACK_SKIP_PIXELS, 0);
            gl::PixelStorei(gl::UNPACK_SKIP_ROWS, 0);
        }
    }
    
    pub fn bind(&self, texture_id: u32) {
        unsafe {
            gl::ActiveTexture(texture_id);
            gl::BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn get_texture_size(&self) -> (i32, i32) {
        (self.width, self.height)
    }
}
