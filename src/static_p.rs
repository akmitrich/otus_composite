#![allow(unused, dead_code)]
use crate::{
    widget::{Number, Widget},
    Component,
};
use std::fmt::Write;

pub struct Container<Content: Component> {
    content: Vec<Content>,
}

impl<Content: Component> Default for Container<Content> {
    fn default() -> Self {
        Self { content: vec![] }
    }
}

impl<Content: Component> Component for Container<Content> {
    fn report(&self) -> String {
        let mut result = String::new();
        for c in self.content.iter() {
            writeln!(result, "{}", c.report());
        }
        result.trim_end().into()
    }
}

impl<Content: Component> Container<Content> {
    pub fn add(&mut self, content: Content) -> usize {
        self.content.push(content);
        self.content.len() - 1
    }

    pub fn remove(&mut self, index: usize) -> Option<Content> {
        if index < self.content.len() {
            Some(self.content.remove(index))
        } else {
            None
        }
    }
}

#[non_exhaustive]
pub enum Poly {
    Widget(Widget),
    Number(Number),
    Container(Container<Poly>),
}

impl Component for Poly {
    fn report(&self) -> String {
        match self {
            Poly::Widget(w) => w.report(),
            Poly::Number(n) => n.report(),
            Poly::Container(c) => c.report(),
            _ => "Unreachable".into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_container() {
        let mut widget_container: Container<Widget> = Default::default();
        assert_eq!(0, widget_container.add(Widget::new("Zero")));
        assert_eq!(1, widget_container.add(Widget::new("One")));
        assert_eq!("Widget 'Zero'.\nWidget 'One'.", widget_container.report());
        assert!(widget_container.remove(42).is_none());
        widget_container.remove(0).unwrap();

        let mut window: Container<Container<Widget>> = Default::default();
        assert_eq!(0, window.add(widget_container));
        assert_eq!(1, window.add(Default::default()));
        assert_eq!("Widget 'One'.", window.report());
    }

    #[test]
    fn test_polymorphism() {
        let mut global_container: Container<Poly> = Default::default();
        let mut window1: Container<Poly> = Default::default();
        window1.add(Poly::Widget(Widget::new("0.0")));
        window1.add(Poly::Number(Number::new(1)));
        global_container.add(Poly::Container(window1));
        global_container.add(Poly::Widget(Widget::new("Main")));
        global_container.add(Poly::Number(Number::new(42)));
        let mut window2: Container<Poly> = Default::default();
        window2.add(Poly::Number(Number::new(2)));
        let mut window_inner: Container<Poly> = Default::default();
        window_inner.add(Poly::Widget(Widget::new("View")));
        window_inner.add(Poly::Widget(Widget::new("Bar")));
        window_inner.add(Poly::Number(Number::new(3)));
        window2.add(Poly::Container(window_inner));
        global_container.add(Poly::Container(window2));
        assert_eq!("Widget '0.0'.\nNumber 1.\nWidget 'Main'.\nNumber 42.\nNumber 2.\nWidget 'View'.\nWidget 'Bar'.\nNumber 3.", global_container.report());
    }
}
