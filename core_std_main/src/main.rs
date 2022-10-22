use core_std::vec::Vec;
#[derive(Debug, Clone, Copy)]
pub struct Pos
{    
    pub x:i16,
    pub y:i16,
}

fn main() 
{
    let mut bonus_list: Vec<u8> = Vec::new();
    println!("{:?}", bonus_list);
    bonus_list.push(16);
    println!("{:?}", bonus_list);
    bonus_list.push(32);
    println!("{:?}", bonus_list);
    bonus_list.push(34);
    println!("{:?}", bonus_list);
    bonus_list.push(56);
    println!("{:?}", bonus_list);

    for i in 0..bonus_list.len
    {        
        println!("elem:{:?} = {:?}",i, bonus_list.get(i));
        println!("elem:{:?} = {:?}",i, bonus_list[i]);
    }

    //POS
    let mut pos_list: Vec<Pos> = Vec::new();
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 0, y: 0 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
    pos_list.push(Pos { x: 22, y: 22 });
    println!("{:?}", pos_list);
}