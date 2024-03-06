struct A {
    name: String,
}

impl A {
    fn new() -> Self {
        println!("zhangyuxuan");
        return A {
            name: String::from("zhangyuxuan"),
        };
    }

    fn change_name(mut self, a: &str) -> Self {
        self.name = String::from(a);
        println!("{}", self.name);
        self
    }
}
