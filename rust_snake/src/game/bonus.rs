use winopengl::Context;
use crate::game::Images::{SNAKE_BONUS};
use crate::pos::Pos;
use crate::graphical_object::GraphicalObject;
use crate::texture::get_texture;

pub(crate) struct Bonus
{
    graphic: GraphicalObject
}
impl Bonus {    
    pub(crate) fn new(ctx: &mut Context) -> Bonus 
    {
        Bonus 
        {
            graphic: GraphicalObject::new(ctx, get_texture(SNAKE_BONUS), false) 
        }
    }
    
    pub fn draw(&mut self, ctx :&mut Context, pos: Pos) 
    {
        self.graphic.draw(ctx, pos, 0.);        
    }
}