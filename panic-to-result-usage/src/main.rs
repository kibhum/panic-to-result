use panic_to_result_macro::panic_to_result;

#[derive(Debug)]
pub struct Person {
    name: String,
    age: u32,
}
#[panic_to_result]
fn create_person(name: String, age: u32) -> Person {
    if age > 30 {
        panic!("This person is getting old")
    }
    Person { name, age }
}

#[panic_to_result]
fn create_person_with_empty_panic(name: String, age: u32) -> Person {
    if age > 30 {
        panic!()
    }
    Person { name, age }
}

#[panic_to_result]
fn create_person_with_result(name: String, age: u32) -> Person {
    if age > 30 {
        panic!()
    }
    Person { name, age }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn happy_path() {
        let actual = create_person("Sam".to_string(), 22).unwrap();

        assert_eq!(actual.name, "Sam".to_string());
        assert_eq!(actual.age, 22);
    }

    #[test]
    fn should_err_on_invalid_age() {
        let actual = create_person("S".to_string(), 32);
        println!("{:?}", actual);
        assert_eq!(
            actual.expect_err("This should be an err"),
            "This person is getting old".to_string()
        );
    }
}
