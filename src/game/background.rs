use miniquad::Context;
use crate::graphical_object::GraphicalObject;
pub(crate) struct Background
{
    graphic: GraphicalObject
}
impl Background {    
    pub(crate) fn new(ctx: &mut Context) -> Background 
    {
        //LOADING IMAGE;
        let width = 1600;
        let height = 896;
        let texture = [0u8; 0];

        Background 
        {
            graphic: GraphicalObject::new(ctx, &texture, width, height) 
        }
    }
    
    pub fn draw(&mut self, ctx: &mut Context) 
    {
        self.graphic.draw(ctx, 0., 0.);        
    }
}