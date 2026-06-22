//! Storage：场景图 + displayList

use crate::element::REDRAW_BIT;
use crate::graphic::displayable::normalize_z;
use crate::graphic::group::{ChildRef, Group};
use crate::graphic::path::Path;

#[derive(Debug, Clone)]
pub struct DisplayItem {
    pub path_index: usize,
    pub sort_key: (f64, f64, f64),
}

pub struct Storage {
    groups: Vec<Group>,
    paths: Vec<Path>,
    roots: Vec<ChildRef>,
    display_list: Vec<DisplayItem>,
    display_dirty: bool,
}

impl Default for Storage {
    fn default() -> Self {
        Self::new()
    }
}

impl Storage {
    pub fn new() -> Self {
        Self {
            groups: Vec::new(),
            paths: Vec::new(),
            roots: Vec::new(),
            display_list: Vec::new(),
            display_dirty: true,
        }
    }

    pub fn create_group(&mut self) -> usize {
        let idx = self.groups.len();
        self.groups.push(Group::new());
        self.display_dirty = true;
        idx
    }

    pub fn create_path(&mut self, path: Path) -> usize {
        let idx = self.paths.len();
        self.paths.push(path);
        self.display_dirty = true;
        idx
    }

    pub fn add_root(&mut self, child: ChildRef) {
        self.roots.push(child);
        self.display_dirty = true;
    }

    pub fn group_add_child(&mut self, group_index: usize, child: ChildRef) {
        self.groups[group_index].add_child(child);
        self.display_dirty = true;
    }

    pub fn group_mut(&mut self, index: usize) -> &mut Group {
        &mut self.groups[index]
    }

    pub fn path_mut(&mut self, index: usize) -> &mut Path {
        &mut self.paths[index]
    }

    pub fn paths(&self) -> &[Path] {
        &self.paths
    }

    pub fn mark_all_dirty(&mut self) {
        self.display_dirty = true;
        for g in &mut self.groups {
            g.base.mark_redraw();
        }
        for p in &mut self.paths {
            p.base.mark_redraw();
        }
    }

    pub fn get_display_list(&mut self, update: bool) -> &[DisplayItem] {
        if update || self.display_dirty {
            self.update_display_list();
        }
        &self.display_list
    }

    pub fn update_display_list(&mut self) {
        self.display_list.clear();
        let roots = self.roots.clone();
        for root in roots {
            self.update_and_add(&root, None);
        }
        self.display_list.sort_by(|a, b| {
            a.sort_key
                .0
                .partial_cmp(&b.sort_key.0)
                .unwrap_or(std::cmp::Ordering::Equal)
                .then_with(|| {
                    a.sort_key
                        .1
                        .partial_cmp(&b.sort_key.1)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .then_with(|| {
                    a.sort_key
                        .2
                        .partial_cmp(&b.sort_key.2)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
        });
        self.display_dirty = false;
    }

    fn update_and_add(&mut self, child: &ChildRef, parent_transform: Option<&[f32; 6]>) {
        match *child {
            ChildRef::Group(gi) => {
                let group_dirty = self.groups[gi].base.dirty;
                let children = self.groups[gi].children.clone();
                {
                    let group = &mut self.groups[gi];
                    group.base.update_transform(parent_transform);
                }
                let transform = self.groups[gi].base.transform().clone();
                for c in children {
                    if group_dirty & REDRAW_BIT != 0 {
                        match c {
                            ChildRef::Group(ci) => self.groups[ci].base.dirty |= REDRAW_BIT,
                            ChildRef::Path(pi) => self.paths[pi].base.dirty |= REDRAW_BIT,
                        }
                    }
                    self.update_and_add(&c, Some(&transform));
                }
                self.groups[gi].base.dirty = 0;
            }
            ChildRef::Path(pi) => {
                let path = &mut self.paths[pi];
                path.base.update_transform(parent_transform);
                normalize_z(&mut path.displayable);
                let sort_key = (
                    path.displayable.zlevel,
                    path.displayable.z,
                    path.displayable.z2,
                );
                path.ensure_path();
                self.display_list.push(DisplayItem {
                    path_index: pi,
                    sort_key,
                });
            }
        }
    }
}
