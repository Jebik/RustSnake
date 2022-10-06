use miniquad::Context;
use crate::images::{SNAKE_BONUS};
use crate::pos::Pos;
use crate::graphical_object::GraphicalObject;

pub(crate) struct Bonus
{
    graphic: GraphicalObject
}
impl Bonus {    
    pub(crate) fn new(ctx: &mut Context) -> Bonus 
    {
        Bonus 
        {
            graphic: GraphicalObject::new(ctx, SNAKE_BONUS, false) 
        }
    }
    
    pub fn draw(&mut self, ctx :&mut Context, pos: GamePos) 
    {
        self.graphic.draw(ctx, pos.curr.x, pos.curr.y, 0.);        
    }
}