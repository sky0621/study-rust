fn main() {
    let n1 = 3;
    let n2 = 10;
    let container = Container(n1, n2);
    println!("{}", container.contains(&n1, &n2));
    println!("{}", container.first());
    println!("{}", container.last());
    println!("{}", difference(&container));
}

struct Container(i32, i32);

trait Contains<A, B> {
    fn contains(&self, _: &A, _: &B) -> bool;
    fn first(&self) -> i32;
    fn last(&self) -> i32;
}

impl Contains<i32, i32> for Container {
    fn contains(&self, n1: &i32, n2: &i32) -> bool {
        (&self.0 == n1) && (&self.1 == n2)
    }

    fn first(&self) -> i32 {
        self.0
    }

    fn last(&self) -> i32 {
        self.1
    }
}

fn difference<A, B, C>(container: &C) -> i32
where
    C: Contains<A, B>,
{
    container.last() - container.first()
}
