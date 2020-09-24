pub trait Draw {
    fn draw(&self);
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}
impl Draw for Button {
    fn draw(&self) {
        println!("draw a button [{}]", self.label);
    }
}

pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub label: String,
    pub options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("draw a select box [{}]", self.label);
    }
}

pub struct Screen {
    pub components: Vec<Box<Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
