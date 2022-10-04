use miniquad::Context;
use crate::pos::Pos;
use crate::graphical_object::GraphicalObject;
pub(crate) struct Bonus
{
    pub pos: Pos,
    graphic: GraphicalObject
}
impl Bonus {    
    pub(crate) fn new(ctx: &mut Context) -> Bonus 
    {
        //LOADING IMAGE;
        let width = 1600;
        let height = 896;
        let texture = [0u8; 0];

        Bonus 
        {
            pos: Pos { x: 0, y: 0 },
            graphic: GraphicalObject::new(ctx, &texture, width, height) 
        }
    }
    
    pub fn draw(&mut self, ctx :&mut Context) 
    {
        self.graphic.draw(ctx, f32::from(self.pos.x), f32::from(self.pos.y));        
    }
}