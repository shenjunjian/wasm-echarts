//! Group 容器

use crate::element::ElementBase;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChildRef {
    Group(usize),
    Path(usize),
    Image(usize),
    Text(usize),
}

#[derive(Debug, Clone)]
pub struct Group {
    pub base: ElementBase,
    pub children: Vec<ChildRef>,
    /// clipPath 引用（paths 数组中的索引）；对子树生效
    pub clip_path: Option<usize>,
}

impl Default for Group {
    fn default() -> Self {
        Self {
            base: ElementBase::default(),
            children: Vec::new(),
            clip_path: None,
        }
    }
}

impl Group {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_clip_path(mut self, clip_index: usize) -> Self {
        self.clip_path = Some(clip_index);
        self
    }

    pub fn add_child(&mut self, child: ChildRef) {
        self.children.push(child);
        self.base.mark_redraw();
    }

    pub fn insert_child(&mut self, index: usize, child: ChildRef) {
        let index = index.min(self.children.len());
        self.children.insert(index, child);
        self.base.mark_redraw();
    }

    pub fn replace_child(&mut self, old: ChildRef, new: ChildRef) -> bool {
        if let Some(idx) = self.children.iter().position(|c| *c == old) {
            self.children[idx] = new;
            self.base.mark_redraw();
            true
        } else {
            false
        }
    }

    /// 从组中移除指定子节点（对齐 zrender Group#remove）
    pub fn remove_child(&mut self, child: ChildRef) -> bool {
        if let Some(idx) = self.children.iter().position(|c| *c == child) {
            self.children.remove(idx);
            self.base.mark_redraw();
            true
        } else {
            false
        }
    }
}
