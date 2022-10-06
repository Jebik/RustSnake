use jpeg_decoder::Decoder;
pub struct TextureData
{    
    pub width: u16,
    pub height: u16,
    pub data: Vec<u8>,
}

pub fn get_texture(file_data: &[u8]) -> TextureData {
    let mut file = Decoder::new(file_data); 
    let pixels = file.decode().expect("failed to decode image");
    let metadata = file.info().unwrap();
    
    //LOADING IMAGE;
    let width:u16 = metadata.width as _;
    let height:u16 = metadata.height as _;
    TextureData
    {
        width,
        height,
        data:Vec::from(pixels)
    }
}
