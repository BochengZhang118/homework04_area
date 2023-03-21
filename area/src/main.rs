trait GraphicalArea {
    fn print_area(&self);
}

struct Rectangular {
 base:u32,
 height:u32,
}

impl GraphicalArea for Rectangular{
    fn print_area(&self) {
        println!("area is {}",self.base*self.height);
    }
}

fn area_print<T:GraphicalArea>(item:T) {
    item.print_area();
}
fn main() {
    let test = Rectangular {base:2,height:3};
    area_print(test);
}
