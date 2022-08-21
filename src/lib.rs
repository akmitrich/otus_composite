#![allow(unused, dead_code)]
pub trait Component {
    fn report(&self) -> String;
}

pub trait Composite: Component {
    fn add_component(&mut self, child: Box<dyn Component>) -> usize; //Must return index of added child
    fn remove_component(&mut self, index: usize) -> Option<Box<dyn Component>>;
}

pub mod static_p;
pub mod widget;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
