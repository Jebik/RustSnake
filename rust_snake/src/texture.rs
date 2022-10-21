use jpeg_decoder::Decoder;
pub struct TextureData
{    
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}
extern crate bmp;
use bmp::{Image, Pixel};
pub fn get_texture(file_data: &[u8]) -> TextureData {
    let mut file = Decoder::new(file_data); 
    let pixels = file.decode().expect("failed to decode image");
    let metadata = file.info().unwrap();
    let mut img = Image::new(256, 256);

    for (x, y) in img.coordinates() {
        img.set_pixel(x, y, Pixel::new(255, 0, 0));
    }
    let _ = img.save("img.bmp");
    //LOADING IMAGE;
    let width:u16 = metadata.width as _;
    let height:u16 = metadata.height as _;
    TextureData
    {
        width,
        height,
        data:pixels
    }
}
