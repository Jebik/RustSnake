use miniquad::Context;
use crate::{graphical_object::{GraphicalObject}, images::SNAKE_BG, pos::Pos};
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
        self.graphic.draw(ctx, Pos{ x: 0, y: 0}, 0.);        
    }
}