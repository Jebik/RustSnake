use std::{fs::File, io::{BufReader, Read}};

use winopengl::Context;
use crate::{graphical_object::{GraphicalObject}, pos::Pos};
pub(crate) struct Background
{
    graphic: GraphicalObject
}
impl Background {    
    pub(crate) fn new(ctx: &mut Context) -> Background 
    { 
        //BG NOIR SI PAS D'IMAGE SIMPLE VECTEUR u8 de Size 1600x896x4
        let f = File::open("./Map/Strasbourg.webp") .expect("no file found");
        let mut reader = BufReader::new(f);
        let mut buffer = Vec::new();
        reader.read_to_end(&mut buffer).expect("readError");
        let texture = &buffer as &[u8];

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