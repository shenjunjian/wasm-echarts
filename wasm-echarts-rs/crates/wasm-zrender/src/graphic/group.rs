//! Group 容器

use crate::element::ElementBase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildRef {
    Group(usize),
    Path(usize),
}

#[derive(Debug, Clone)]
pub struct Group {
    pub base: ElementBase,
    pub children: Vec<ChildRef>,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            base: ElementBase::default(),
            children: Vec::new(),
        }
    }
}

impl Group {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_child(&mut self, child: ChildRef) {
        self.children.push(child);
        self.base.mark_redraw();
    }
}
