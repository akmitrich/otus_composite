#![allow(unused, dead_code)]
use crate::{Component, Composite};
use std::fmt::Write;

pub struct Widget {
    name: String,
}

pub struct Number(i32);

pub struct Window {
    widgets: Vec<Box<dyn Component>>,
}

impl Component for Widget {
    fn report(&self) -> String {
        format!("Widget '{}'.", self.name)
    }
}

impl Component for Number {
    fn report(&self) -> String {
        format!("Number {}.", self.0)
    }
}

impl Component for Window {
    fn report(&self) -> String {
        let mut result = String::new();
        for w in self.widgets.iter() {
            writeln!(result, "{}", w.report()).unwrap_or_default();
        }
        result.trim_end().into()
    }
}

impl Composite for Window {
    fn add_component(&mut self, child: Box<dyn Component>) -> usize {
        self.widgets.push(child);
        self.widgets.len() - 1
    }

    fn remove_component(&mut self, index: usize) -> Option<Box<dyn Component>> {
        if index < self.widgets.len() {
            Some(self.widgets.remove(index))
        } else {
            None
        }
    }
}

impl Widget {
    pub fn new(name: &str) -> Self {
        Self {
            name: String::from(name),
        }
    }
}

impl Number {
    pub fn new(value: i32) -> Self {
        Self(value)
    }
}

impl Window {
    pub fn new() -> Self {
        Self { widgets: vec![] }
    }
}

impl Default for Window {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_widgets() {
        let widget = Widget::new("Test");
        assert_eq!("Widget 'Test'.", widget.report());

        let mut w1 = Window::new();
        assert_eq!(0, w1.add_component(Box::new(Widget::new("Widget 1"))));
        assert_eq!(1, w1.add_component(Box::new(Widget::new("Widget 2"))));
        assert_eq!(2, w1.add_component(Box::new(Number::new(3))));
        assert_eq!(
            "Widget 'Widget 1'.\nWidget 'Widget 2'.\nNumber 3.",
            w1.report()
        );
        let mut w2 = Window::new();
        assert_eq!(0, w2.add_component(Box::new(Widget::new("Widget 4"))));
        assert_eq!("Widget 'Widget 4'.", w2.report());
        assert!(w1.remove_component(42).is_none());
        w1.remove_component(1).unwrap();

        let mut window = Window::new();
        assert_eq!(0, window.add_component(Box::new(w1)));
        assert_eq!(1, window.add_component(Box::new(Number::new(42))));
        assert_eq!(2, window.add_component(Box::new(w2)));
        assert_eq!(
            "Widget 'Widget 1'.\nNumber 3.\nNumber 42.\nWidget 'Widget 4'.",
            window.report()
        );

        window.remove_component(0).unwrap();
        assert_eq!("Number 42.\nWidget 'Widget 4'.", window.report());
    }
}
