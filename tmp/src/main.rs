use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let mut c = Cat::new(|n| n * n);
    let r = c.value(33);
    println!("{}", r);
}

struct Cat<T>
where
    T: Fn(u32) -> u32,
{
    // 計算クロージャ
    calculation: T,
    // 計算クロージャによる計算結果を保持する
    value_map: HashMap<u32, u32>,
}
impl<T> Cat<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cat<T> {
        Cat {
            calculation,
            value_map: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        if self.value_map.contains_key(&arg) {
            match self.value_map.get(&arg) {
                Some(v) => *v,
                None => 0,
            }
        } else {
            let v = (self.calculation)(arg);
            self.value_map.insert(arg, v);
            v
        }
    }
}
