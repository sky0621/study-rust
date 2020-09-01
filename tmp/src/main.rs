use std::collections::HashMap;

fn main() {
    const t: Fn(u32) -> u32 = |n| n * n;
    let abc: Abc<u32, u32, t> = Abc::new();
}

struct Abc<K, V, T>
where
    T: Fn(K) -> V,
{
    m: HashMap<K, V>,
    c: T,
}

impl<K, V, T> Abc<K, V, T>
where
    T: Fn(K) -> V,
{
    fn new() -> Abc<K, V, T> {
        Abc {
            m: HashMap::new(),
            c: (),
        }
    }
}
