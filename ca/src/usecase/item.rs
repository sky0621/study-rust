use crate::domain;

pub fn exec() {
    println!("usecase/item");
    domain::item::exec();
    domain::repository::item::exec();
}
