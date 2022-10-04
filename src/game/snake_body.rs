use crate::pos::{Pos, FloatPos};
pub struct SnakeBody{ 
    //ForDrawing
    pub real: FloatPos,
    //ForLogic
    pub curr: Pos,
    pub dest: Pos,
}

impl SnakeBody {    
    pub(crate) fn new(x:i16, y:i16) -> SnakeBody {
        SnakeBody {
            real:FloatPos { x: f32::from(x), y: f32::from(y) },
            curr:Pos { x: x, y: y },
            dest:Pos { x: -1, y: -1 }
        }
    } 
}