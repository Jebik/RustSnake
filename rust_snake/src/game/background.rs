use std::{fs::File, io::{BufReader, Read}};

use winopengl::Context;
use crate::{graphical_object::{GraphicalObject}, pos::Pos, texture::{TextureData, get_texture}};
pub(crate) struct Background
{
    graphic: GraphicalObject
}
impl Background {    
    pub(crate) fn new(ctx: &mut Context) -> Background 
    { 
        //BG NOIR SI PAS D'IMAGE SIMPLE VECTEUR u8 de Size 1600x896x4
        let open_file = File::open("./Map/Strasbourg.jpg");
        
        let buffer = vec![70u8; 1600*896*3];
        let mut texture = TextureData
        {
            width: 1600,
            height: 896,
            data: buffer
        }; 
        if open_file.is_ok()
        {
            let mut file_buffer = Vec::new();
            let f = open_file.unwrap();
            let mut reader = BufReader::new(f);
            reader.read_to_end(&mut file_buffer).expect("readError");
            texture = get_texture(&file_buffer as &[u8]);
        }

        Background 
        {
            graphic: GraphicalObject::new(ctx, texture, false) 
        }
    }
    
    pub fn draw(&mut self, ctx: &mut Context)
    {
        self.graphic.draw(ctx, Pos{ x: 0, y: 0}, 0.);        
    }
}