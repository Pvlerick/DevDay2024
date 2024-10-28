fn main() {
    let x = "Hello".to_owned();
    print(x);
}

fn print<T: Print>(x: T) {
    println!("{}: {}", x.type_name(), x.value());
}

trait Print {
    fn type_name(&self) -> String;
    fn value(&self) -> String;
}

impl Print for u32 {
    fn type_name(&self) -> String {
        "u32".to_owned()
    }

    fn value(&self) -> String {
        format!("{self}")
    }
}

impl Print for String {
    fn type_name(&self) -> String {
        "String".to_owned()
    }

    fn value(&self) -> String {
        self.to_owned()
    }
}
