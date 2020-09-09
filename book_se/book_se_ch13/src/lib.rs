use std::collections::HashMap;
use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(1));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) -> Vec<String> {
    let mut r: Vec<String> = vec![];
    let mut expensive_result = Cacher::new(|num| simulated_expensive_calculation(num));
    if intensity < 25 {
        r.push(format!(
            "Today, do {} pushups!",
            expensive_result.value(intensity)
        ));
        r.push(format!(
            "Next, do {} situps!",
            expensive_result.value(intensity)
        ));
    } else {
        if random_number == 3 {
            r.push(String::from(
                "Take a break today! Remember to stay hydrated!",
            ));
        } else {
            r.push(format!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            ));
        }
    }
    return r;
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    // 計算クロージャ
    calculation: T,
    // 計算クロージャによる計算結果を保持する
    value_map: HashMap<u32, u32>,
}
impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulated_expensive_calculation() {
        let param: u32 = 101;
        let result = simulated_expensive_calculation(param);
        assert_eq!(101, result);
    }

    #[test]
    fn test_generate_workout_intensity_lt_25() {
        let intensity = 24;
        let random_number = 7;
        let result = generate_workout(intensity, random_number);
        let mut expected: Vec<String> = vec![];
        expected.push(String::from("Today, do 24 pushups!"));
        expected.push(String::from("Next, do 24 situps!"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_generate_workout_intensity_ge_25() {
        let intensity = 25;
        let random_number = 7;
        let result = generate_workout(intensity, random_number);
        let mut expected: Vec<String> = vec![];
        expected.push(String::from("Today, run for 25 minutes!"));
        assert_eq!(expected, result);
    }

    #[test]
    fn test_generate_workout_intensity_ge_25_random_eq_3() {
        let intensity = 25;
        let random_number = 3;
        let result = generate_workout(intensity, random_number);
        let mut expected: Vec<String> = vec![];
        expected.push(String::from(
            "Take a break today! Remember to stay hydrated!",
        ));
        assert_eq!(expected, result);
    }

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|num| num);
        let _v1 = c.value(1);
        let v2 = c.value(2);
        assert_eq!(2, v2);
    }
}
