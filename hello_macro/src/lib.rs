// https://doc.rust-lang.org/stable/book/ch19-06-macros.html
pub trait HelloMacro {
    fn members(self) -> Vec<(String, Box<dyn std::any::Any>)>;
}
