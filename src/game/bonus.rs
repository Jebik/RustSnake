use miniquad::Context;
use crate::images::{SNAKE_BONUS};
use crate::pos::Pos;
use crate::graphical_object::{GraphicalObject, self};
pub(crate) struct Bonus
{
    pub pos: Pos,
    graphic: GraphicalObject
}
impl Bonus {    
    pub(crate) fn new(ctx: &mut Context) -> Bonus 
    {
        Bonus 
        {
            pos: Pos { x: 0, y: 0 },
            graphic: GraphicalObject::new(ctx, SNAKE_BONUS) 
        }
    }
    
    pub fn draw(&mut self, ctx :&mut Context) 
    {
        self.graphic.draw(ctx, f32::from(self.pos.x), f32::from(self.pos.y));        
    }
}