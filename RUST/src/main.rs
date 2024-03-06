trait Print {
    fn my_print(&self) -> ();
}

impl Print for i32 {
    fn my_print(&self) {
        println!("{}", self)
    }
}

fn display_array<T: std::fmt::Debug>(arr: &[T]) {
    println!("{:?}", arr)
}

trait Sort {
    fn my_sort(&self);
}

impl Sort for &[i32] {
    fn my_sort(&self) {
        println!("执行操作");
    }
}

fn main() {
    let a = [1,2,3];
    a.as_slice().my_sort()
}

