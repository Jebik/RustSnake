
pub struct TextureData
{    
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}
extern crate bmp;

pub fn get_texture(file_data: &[u8]) -> TextureData {
    let file = bmp::open("./images/SnakeHeadBorder.bmp"); 
    let pixels = file.unwrap();
    
    //LOADING IMAGE;
    let width = pixels.get_width();
    let height = pixels.get_height();

    let mut data = Vec::new();
    for i in 0..width
    {
        for j in 0..height
        {
            let p = pixels.get_pixel(i, j);
            data.push(p.r);
            data.push(p.g);
            data.push(p.b);
        }
    }
    TextureData
    {
        width: width as _,
        height: height as _,
        data: data
    }
}
