use std::io::Write;

fn convert_to_rs(image_filename:&str, rust_constname: &str)
{
    let img = image::open("E:/WORK/Rust/RustSnake/export_image/src/Image".to_owned() + &image_filename).unwrap();
    let img = match img 
    {
        image::DynamicImage::ImageRgba8(img) => img,
        x => x.to_rgba8()
    };
    let width = img.width();
    let height = img.height();

    let mut file = std::fs::File::create(rust_constname.to_lowercase() + ".rs").expect("create failed");

    let width_txt = "pub(crate) const ".to_owned() + &rust_constname + &"_WIDTH:u16 = ".to_owned() +  &width.to_string() + ";\n";
    let height_txt = "pub(crate) const ".to_owned() + &rust_constname + &"_HEIGHT:u16 = ".to_owned() +  &height.to_string() + ";\n\n";
    
    let init_txt = "pub(crate) const ".to_owned() + &rust_constname + &"_RGB:[u8;" + &width.to_string() + &" * " + &height.to_string() + &" * 4] = [";

    file.write_all(width_txt.as_bytes()).expect("write failed");
    file.write_all(height_txt.as_bytes()).expect("write failed");
    file.write_all(init_txt.as_bytes()).expect("write failed");

    for h in (0..height).rev()
    {
        for w in 0..width
        {
            let p = img.get_pixel(w, h);            
            let current = "".to_owned() + &p[0].to_string() + "," +  &p[1].to_string() + "," +&p[2].to_string() + "," +&p[3].to_string() + ",";
            file.write_all(current.as_bytes()).expect("write failed");
        }
    }
    file.write_all("];".as_bytes()).expect("write failed");
}


fn convert_to_multiple_rs(image_filename:&str, rust_constname: &str)
{
    let img = image::open("E:/WORK/Rust/RustSnake/export_image/src/Image".to_owned() + &image_filename).unwrap();
    let img = match img 
    {
        image::DynamicImage::ImageRgba8(img) => img,
        x => x.to_rgba8()
    };
    let width = img.width();
    let height = img.height();

    let mut file1 = std::fs::File::create(rust_constname.to_lowercase() + &1.to_string() + ".rs").expect("create failed");
    let mut file2 = std::fs::File::create(rust_constname.to_lowercase() + &2.to_string() + ".rs").expect("create failed");
    let mut file3 = std::fs::File::create(rust_constname.to_lowercase() + &3.to_string() + ".rs").expect("create failed");
    let mut file4 = std::fs::File::create(rust_constname.to_lowercase() + &4.to_string() + ".rs").expect("create failed");
    
    let init_txt = "pub(crate) const ".to_owned() + &rust_constname + &"_RGB:[u8;" + &(width/2).to_string() + &" * " + &(height/2).to_string() + &" * 4] = [";

    file1.write_all(init_txt.as_bytes()).expect("write failed");
    file2.write_all(init_txt.as_bytes()).expect("write failed");
    file3.write_all(init_txt.as_bytes()).expect("write failed");
    file4.write_all(init_txt.as_bytes()).expect("write failed");

    for h in (0..height/2).rev()
    {
        for w in 0..width/2
        {
            let p = img.get_pixel(w, h);            
            let current = "".to_owned() + &p[0].to_string() + "," +  &p[1].to_string() + "," +&p[2].to_string() + "," +&p[3].to_string() + ",";
            file1.write_all(current.as_bytes()).expect("write failed");
        }
    }    
    for h in (0..height/2).rev()
    {
        for w in width/2..width
        {
            let p = img.get_pixel(w, h);            
            let current = "".to_owned() + &p[0].to_string() + "," +  &p[1].to_string() + "," +&p[2].to_string() + "," +&p[3].to_string() + ",";
            file2.write_all(current.as_bytes()).expect("write failed");
        }
    }    
    for h in (height/2..height).rev()
    {
        for w in 0..width/2
        {
            let p = img.get_pixel(w, h);            
            let current = "".to_owned() + &p[0].to_string() + "," +  &p[1].to_string() + "," +&p[2].to_string() + "," +&p[3].to_string() + ",";
            file3.write_all(current.as_bytes()).expect("write failed");
        }
    }    
    for h in (height/2..height).rev()
    {
        for w in width/2..width
        {
            let p = img.get_pixel(w, h);            
            let current = "".to_owned() + &p[0].to_string() + "," +  &p[1].to_string() + "," +&p[2].to_string() + "," +&p[3].to_string() + ",";
            file4.write_all(current.as_bytes()).expect("write failed");
        }
    }
    file1.write_all("];".as_bytes()).expect("write failed");
    file2.write_all("];".as_bytes()).expect("write failed");
    file3.write_all("];".as_bytes()).expect("write failed");
    file4.write_all("];".as_bytes()).expect("write failed");
}

fn main() {
    convert_to_multiple_rs("/SnakeBg.png", "SNAKE_BG");
    convert_to_rs("64/SnakeHead.png", "SNAKE_HEAD");
    convert_to_rs("64/SnakeBonus.png", "SNAKE_BONUS");
    convert_to_rs("64/SnakeBody.png", "SNAKE_BODY");
    convert_to_rs("32/SnakeBody.png", "SNAKE_BODY_32");
    convert_to_rs("16/SnakeBody.png", "SNAKE_BODY_16");
}