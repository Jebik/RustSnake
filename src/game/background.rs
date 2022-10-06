use miniquad::Context;
use crate::{graphical_object::{GraphicalObject}, images::SNAKE_BG};
pub(crate) struct Background
{
    graphic: GraphicalObject
}
impl Background {    
    pub(crate) fn new(ctx: &mut Context) -> Background 
    {
        Background 
        {
            graphic: GraphicalObject::new(ctx, SNAKE_BG, false) 
        }
    }
    
    pub fn draw(&mut self, ctx: &mut Context) 
    {
        self.graphic.draw(ctx, 0., 0.);        
    }
}