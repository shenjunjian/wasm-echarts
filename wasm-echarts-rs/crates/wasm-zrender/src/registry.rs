//! ZRender / Element 实例表

use std::cell::RefCell;
use std::collections::HashMap;

use rust_zrender::{ChildRef, ZRenderer};
use wasm_bindgen::prelude::*;

use crate::element::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Group,
    Path,
    Text,
}

#[derive(Debug, Clone)]
struct ElementRecord {
    kind: ElementKind,
    storage_index: usize,
    zr_id: Option<u32>,
    parent_id: Option<u32>,
    mounted: bool,
    type_name: String,
}

pub struct ElementRegistry {
    next_id: u32,
    elements: HashMap<u32, ElementRecord>,
}

impl Default for ElementRegistry {
    fn default() -> Self {
        Self {
            next_id: 1,
            elements: HashMap::new(),
        }
    }
}

impl ElementRegistry {
    pub fn register(
        &mut self,
        kind: ElementKind,
        storage_index: usize,
        type_name: impl Into<String>,
    ) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.elements.insert(
            id,
            ElementRecord {
                kind,
                storage_index,
                zr_id: None,
                parent_id: None,
                mounted: false,
                type_name: type_name.into(),
            },
        );
        id
    }

    pub fn contains(&self, id: u32) -> bool {
        self.elements.contains_key(&id)
    }

    pub fn kind(&self, id: u32) -> Option<ElementKind> {
        self.elements.get(&id).map(|r| r.kind)
    }

    pub fn element_type(&self, id: u32) -> Option<String> {
        self.elements.get(&id).map(|r| r.type_name.clone())
    }

    pub fn storage_index(&self, id: u32) -> Option<usize> {
        self.elements.get(&id).map(|r| r.storage_index)
    }

    pub fn child_ref(&self, id: u32) -> Result<ChildRef, JsValue> {
        let record = self
            .elements
            .get(&id)
            .ok_or_else(|| JsValue::from_str("invalid element"))?;
        match record.kind {
            ElementKind::Group => Ok(ChildRef::Group(record.storage_index)),
            ElementKind::Path => Ok(ChildRef::Path(record.storage_index)),
            ElementKind::Text => Err(JsValue::from_str(
                "Text as group child is not supported yet; use zr.add(text) at root",
            )),
        }
    }

    pub fn find_by_storage(&self, kind: ElementKind, storage_index: usize) -> Option<u32> {
        self.elements.iter().find_map(|(&id, r)| {
            if r.kind == kind && r.storage_index == storage_index {
                Some(id)
            } else {
                None
            }
        })
    }

    pub fn set_parent(&mut self, child_id: u32, parent_id: u32) -> Result<(), JsValue> {
        let parent_kind = self
            .kind(parent_id)
            .ok_or_else(|| JsValue::from_str("invalid parent element"))?;
        if parent_kind != ElementKind::Group {
            return Err(JsValue::from_str("parent must be a Group"));
        }
        if self.parent_id(child_id).is_some() {
            return Err(JsValue::from_str("element already has a parent"));
        }
        self.elements
            .get_mut(&child_id)
            .ok_or_else(|| JsValue::from_str("invalid element"))?
            .parent_id = Some(parent_id);
        Ok(())
    }

    pub fn parent_id(&self, id: u32) -> Option<u32> {
        self.elements.get(&id).and_then(|r| r.parent_id)
    }

    pub fn is_mounted(&self, id: u32) -> bool {
        self.elements.get(&id).is_some_and(|r| r.mounted)
    }

    pub fn can_add_to_zr(&self, id: u32) -> bool {
        self.elements.get(&id).is_some_and(|r| !r.mounted && r.parent_id.is_none())
    }

    pub fn mark_mounted(&mut self, id: u32, zr_id: u32) -> Result<(), JsValue> {
        let record = self
            .elements
            .get_mut(&id)
            .ok_or_else(|| JsValue::from_str("invalid element"))?;
        if record.mounted {
            return Err(JsValue::from_str("element already mounted"));
        }
        record.mounted = true;
        record.zr_id = Some(zr_id);
        Ok(())
    }

    pub fn mark_unmounted(&mut self, id: u32) {
        if let Some(record) = self.elements.get_mut(&id) {
            record.mounted = false;
            record.zr_id = None;
            record.parent_id = None;
        }
    }

    pub fn assert_belongs_to_zr(&self, element_id: u32, zr_id: u32) -> Result<(), JsValue> {
        let record = self
            .elements
            .get(&element_id)
            .ok_or_else(|| JsValue::from_str("invalid element"))?;
        match record.zr_id {
            Some(id) if id == zr_id => Ok(()),
            Some(_) => Err(JsValue::from_str("element belongs to another ZRender instance")),
            None if record.mounted => Err(JsValue::from_str("element is not mounted")),
            None => Ok(()),
        }
    }

    pub fn assert_ready_for_zr_add(&self, element_id: u32) -> Result<(), JsValue> {
        let record = self
            .elements
            .get(&element_id)
            .ok_or_else(|| JsValue::from_str("invalid element"))?;
        if record.mounted {
            return Err(JsValue::from_str("element already added to a ZRender instance"));
        }
        if record.parent_id.is_some() {
            return Err(JsValue::from_str(
                "nested element must be added via Group.add, not zr.add",
            ));
        }
        Ok(())
    }

    pub fn remove_by_zr(&mut self, zr_id: u32) {
        self.elements.retain(|_, r| r.zr_id != Some(zr_id));
    }

    pub fn clear(&mut self) {
        self.elements.clear();
        self.next_id = 1;
    }
}

pub struct ZRenderRegistry {
    next_id: u32,
    instances: HashMap<u32, ZRenderer>,
}

impl Default for ZRenderRegistry {
    fn default() -> Self {
        Self {
            next_id: 1,
            instances: HashMap::new(),
        }
    }
}

impl ZRenderRegistry {
    pub fn insert(&mut self, zr: ZRenderer) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.instances.insert(id, zr);
        id
    }

    pub fn contains(&self, id: u32) -> bool {
        self.instances.contains_key(&id)
    }

    pub fn get(&self, id: u32) -> Option<&ZRenderer> {
        self.instances.get(&id)
    }

    pub fn get_mut(&mut self, id: u32) -> Option<&mut ZRenderer> {
        self.instances.get_mut(&id)
    }

    pub fn remove(&mut self, id: u32) -> Option<ZRenderer> {
        self.instances.remove(&id)
    }

    pub fn clear(&mut self) {
        self.instances.clear();
    }
}

thread_local! {
    pub static ZR_REGISTRY: RefCell<ZRenderRegistry> = RefCell::new(ZRenderRegistry::default());
    pub static ELEMENT_REGISTRY: RefCell<ElementRegistry> = RefCell::new(ElementRegistry::default());
}

pub(crate) fn with_zr<F, T>(id: u32, f: F) -> Result<T, JsValue>
where
    F: FnOnce(&mut ZRenderer) -> Result<T, JsValue>,
{
    ZR_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        let zr = reg
            .get_mut(id)
            .ok_or_else(|| JsValue::from_str("ZRender instance not found"))?;
        f(zr)
    })
}

pub(crate) fn mount_element_to_zr(zr_id: u32, element: &Element) -> Result<(), JsValue> {
    let element_id = element.raw_id();
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        reg.assert_ready_for_zr_add(element_id)?;
        let child = reg.child_ref(element_id)?;
        reg.mark_mounted(element_id, zr_id)?;
        with_zr(zr_id, |zr| {
            zr.storage.add_root(child);
            Ok(())
        })
    })
}

pub(crate) fn unmount_element_from_zr(zr_id: u32, element: &Element) -> Result<(), JsValue> {
    let element_id = element.raw_id();
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        reg.assert_belongs_to_zr(element_id, zr_id)?;
        let child = reg.child_ref(element_id)?;
        reg.mark_unmounted(element_id);
        with_zr(zr_id, |zr| {
            zr.storage.del_root(child);
            Ok(())
        })
    })
}

pub(crate) fn register_group(storage_index: usize) -> Element {
    let id = ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut()
            .register(ElementKind::Group, storage_index, "group")
    });
    Element::from_id(id)
}

pub(crate) fn register_path(storage_index: usize, type_name: &str) -> Element {
    let id = ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut()
            .register(ElementKind::Path, storage_index, type_name)
    });
    Element::from_id(id)
}

pub(crate) fn register_text(storage_index: usize) -> Element {
    let id = ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut()
            .register(ElementKind::Text, storage_index, "text")
    });
    Element::from_id(id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_zrender::ZRenderer;

    #[test]
    fn element_registry_tracks_mount_state() {
        let mut reg = ElementRegistry::default();
        let id = reg.register(ElementKind::Path, 0, "rect");
        assert!(reg.can_add_to_zr(id));
        reg.mark_mounted(id, 1).unwrap();
        assert!(reg.is_mounted(id));
        assert!(!reg.can_add_to_zr(id));
    }

    #[test]
    fn zrender_registry_insert_remove() {
        let mut reg = ZRenderRegistry::default();
        let id = reg.insert(ZRenderer::new(100, 100).unwrap());
        assert!(reg.contains(id));
        reg.remove(id);
        assert!(!reg.contains(id));
    }
}
