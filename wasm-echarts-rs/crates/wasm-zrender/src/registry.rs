//! ZRender / Element 实例表

use std::cell::RefCell;
use std::collections::HashMap;

use rust_zrender::{
    ChildRef, Image, Path, PathStylePatch, Text, ZRenderer,
};
use wasm_bindgen::prelude::*;

use crate::bridge::build::{build_pending_image, build_pending_path, build_pending_text};
use crate::element::pending::PendingData;
use crate::element::Element;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ElementKind {
    Group,
    Path,
    Text,
    Image,
}

#[derive(Debug, Clone)]
struct ElementRecord {
    kind: ElementKind,
    storage_index: Option<usize>,
    zr_id: Option<u32>,
    parent_id: Option<u32>,
    mounted: bool,
    type_name: String,
    pending: PendingData,
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
    fn register(&mut self, kind: ElementKind, pending: PendingData, type_name: impl Into<String>) -> u32 {
        let id = self.next_id;
        self.next_id += 1;
        self.elements.insert(
            id,
            ElementRecord {
                kind,
                storage_index: None,
                zr_id: None,
                parent_id: None,
                mounted: false,
                type_name: type_name.into(),
                pending,
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
        self.elements.get(&id).and_then(|r| r.storage_index)
    }

    pub fn zr_id(&self, id: u32) -> Option<u32> {
        self.elements.get(&id).and_then(|r| r.zr_id)
    }

    pub fn child_ref(&self, id: u32) -> Result<ChildRef, JsValue> {
        let record = self
            .elements
            .get(&id)
            .ok_or_else(|| JsValue::from_str("invalid element"))?;
        let idx = record
            .storage_index
            .ok_or_else(|| JsValue::from_str("element is not materialized"))?;
        match record.kind {
            ElementKind::Group => Ok(ChildRef::Group(idx)),
            ElementKind::Path => Ok(ChildRef::Path(idx)),
            ElementKind::Image => Ok(ChildRef::Image(idx)),
            ElementKind::Text => Err(JsValue::from_str(
                "Text as group child is not supported yet; use zr.add(text) at root",
            )),
        }
    }

    pub fn find_by_storage(&self, kind: ElementKind, storage_index: usize) -> Option<u32> {
        self.elements.iter().find_map(|(&id, r)| {
            if r.kind == kind && r.storage_index == Some(storage_index) {
                Some(id)
            } else {
                None
            }
        })
    }

    pub fn children_of(&self, parent_id: u32) -> Vec<u32> {
        self.elements
            .iter()
            .filter_map(|(&id, r)| {
                if r.parent_id == Some(parent_id) {
                    Some(id)
                } else {
                    None
                }
            })
            .collect()
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

    pub fn clear_parent(&mut self, child_id: u32) {
        if let Some(record) = self.elements.get_mut(&child_id) {
            record.parent_id = None;
        }
    }

    pub fn parent_id(&self, id: u32) -> Option<u32> {
        self.elements.get(&id).and_then(|r| r.parent_id)
    }

    pub fn is_mounted(&self, id: u32) -> bool {
        self.elements.get(&id).is_some_and(|r| r.mounted)
    }

    pub fn can_add_to_zr(&self, id: u32) -> bool {
        self.elements
            .get(&id)
            .is_some_and(|r| !r.mounted && r.parent_id.is_none())
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
            None => Err(JsValue::from_str("element is not mounted")),
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

    pub fn materialize_element(
        &mut self,
        zr: &mut ZRenderer,
        zr_id: u32,
        element_id: u32,
    ) -> Result<(), JsValue> {
        if self
            .elements
            .get(&element_id)
            .is_some_and(|r| r.storage_index.is_some())
        {
            return Ok(());
        }

        let kind = self
            .kind(element_id)
            .ok_or_else(|| JsValue::from_str("invalid element"))?;
        match kind {
            ElementKind::Group => self.materialize_group(zr, zr_id, element_id),
            ElementKind::Path => self.materialize_path(zr, zr_id, element_id),
            ElementKind::Text => self.materialize_text(zr, zr_id, element_id),
            ElementKind::Image => self.materialize_image(zr, zr_id, element_id),
        }
    }

    pub fn materialize_tree(
        &mut self,
        zr: &mut ZRenderer,
        zr_id: u32,
        root_id: u32,
    ) -> Result<(), JsValue> {
        self.materialize_element(zr, zr_id, root_id)?;
        if self.kind(root_id) == Some(ElementKind::Group) {
            let child_ids = self.children_of(root_id);
            for child_id in child_ids {
                self.materialize_tree(zr, zr_id, child_id)?;
                let group_idx = self.storage_index(root_id).unwrap();
                let child = self.child_ref(child_id)?;
                zr.storage.group_add_child(group_idx, child);
            }
        }
        Ok(())
    }

    fn materialize_group(
        &mut self,
        zr: &mut ZRenderer,
        zr_id: u32,
        element_id: u32,
    ) -> Result<(), JsValue> {
        let idx = zr.storage.create_group();
        if let Some(record) = self.elements.get_mut(&element_id) {
            record.storage_index = Some(idx);
            record.zr_id = Some(zr_id);
        }
        Ok(())
    }

    fn materialize_path(
        &mut self,
        zr: &mut ZRenderer,
        zr_id: u32,
        element_id: u32,
    ) -> Result<(), JsValue> {
        let pending = self
            .elements
            .get(&element_id)
            .and_then(|r| {
                if let PendingData::Path(p) = &r.pending {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| JsValue::from_str("invalid path element"))?;

        let mut path = Path::new(pending.shape, pending.style)
            .with_displayable(pending.displayable)
            .with_ec_data(pending.ec_data);
        path.silent = pending.silent;
        path.base.name = pending.name;

        for (state, patch) in &pending.state_patches {
            path.states.set_state_patch(state, patch.clone());
        }
        for state in &pending.active_states {
            path.use_state(state);
        }

        let idx = zr.storage.create_path(path);
        if let Some(record) = self.elements.get_mut(&element_id) {
            record.storage_index = Some(idx);
            record.zr_id = Some(zr_id);
        }
        Ok(())
    }

    fn materialize_text(
        &mut self,
        zr: &mut ZRenderer,
        zr_id: u32,
        element_id: u32,
    ) -> Result<(), JsValue> {
        let pending = self
            .elements
            .get(&element_id)
            .and_then(|r| {
                if let PendingData::Text(t) = &r.pending {
                    Some(t.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| JsValue::from_str("invalid text element"))?;

        let mut text = Text::new(pending.content, pending.x, pending.y)
            .with_style(pending.style)
            .with_displayable(pending.displayable);
        text.silent = pending.silent;
        text.ec_data = pending.ec_data;
        text.base.name = pending.name;

        let idx = zr.storage.create_text(text);
        if let Some(record) = self.elements.get_mut(&element_id) {
            record.storage_index = Some(idx);
            record.zr_id = Some(zr_id);
        }
        Ok(())
    }

    fn materialize_image(
        &mut self,
        zr: &mut ZRenderer,
        zr_id: u32,
        element_id: u32,
    ) -> Result<(), JsValue> {
        let pending = self
            .elements
            .get(&element_id)
            .and_then(|r| {
                if let PendingData::Image(p) = &r.pending {
                    Some(p.clone())
                } else {
                    None
                }
            })
            .ok_or_else(|| JsValue::from_str("invalid image element"))?;

        let mut image = Image::new(pending.style)
            .with_displayable(pending.displayable)
            .with_ec_data(pending.ec_data);
        image.silent = pending.silent;
        image.base.name = pending.name;

        let idx = zr.storage.create_image(image);
        if let Some(record) = self.elements.get_mut(&element_id) {
            record.storage_index = Some(idx);
            record.zr_id = Some(zr_id);
        }
        Ok(())
    }

    pub fn apply_path_state(&mut self, element_id: u32, state: &str) -> Result<(), JsValue> {
        if self.storage_index(element_id).is_none() {
            if let Some(record) = self.elements.get_mut(&element_id) {
                if let PendingData::Path(pending) = &mut record.pending {
                    pending.active_states = vec![state.to_string()];
                    return Ok(());
                }
            }
            return Err(JsValue::from_str("invalid path element"));
        }

        let zr_id = self
            .zr_id(element_id)
            .ok_or_else(|| JsValue::from_str("element is not attached to a ZRender instance"))?;
        let path_index = self.storage_index(element_id).unwrap();
        with_zr(zr_id, |zr| {
            zr.set_path_state(path_index, state);
            Ok(())
        })
    }

    pub fn apply_path_state_style(
        &mut self,
        element_id: u32,
        state: &str,
        patch: PathStylePatch,
    ) -> Result<(), JsValue> {
        if self.storage_index(element_id).is_none() {
            if let Some(record) = self.elements.get_mut(&element_id) {
                if let PendingData::Path(pending) = &mut record.pending {
                    pending.state_patches.insert(state.to_string(), patch);
                    return Ok(());
                }
            }
            return Err(JsValue::from_str("invalid path element"));
        }

        let zr_id = self
            .zr_id(element_id)
            .ok_or_else(|| JsValue::from_str("element is not attached to a ZRender instance"))?;
        let path_index = self.storage_index(element_id).unwrap();
        with_zr(zr_id, |zr| {
            zr.set_path_state_style(path_index, state, patch);
            Ok(())
        })
    }

    pub fn attach_to_zr(&mut self, element_id: u32, zr_id: u32) -> Result<(), JsValue> {
        if let Some(record) = self.elements.get_mut(&element_id) {
            record.zr_id = Some(zr_id);
            Ok(())
        } else {
            Err(JsValue::from_str("invalid element"))
        }
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

    pub fn update_all_font_databases(&mut self, resolved: &rust_zrender::ResolvedFontConfig) {
        for zr in self.instances.values_mut() {
            zr.update_font_database(resolved);
        }
    }
}

thread_local! {
    pub static ZR_REGISTRY: RefCell<ZRenderRegistry> = RefCell::new(ZRenderRegistry::default());
    pub static ELEMENT_REGISTRY: RefCell<ElementRegistry> = RefCell::new(ElementRegistry::default());
}

pub(crate) fn refresh_all_font_databases(resolved: &rust_zrender::ResolvedFontConfig) {
    ZR_REGISTRY.with(|reg| {
        reg.borrow_mut().update_all_font_databases(resolved);
    });
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
        reg.attach_to_zr(element_id, zr_id)?;
        with_zr(zr_id, |zr| {
            reg.materialize_tree(zr, zr_id, element_id)?;
            let kind = reg.kind(element_id).unwrap();
            if kind != ElementKind::Text {
                let child = reg.child_ref(element_id)?;
                zr.storage.add_root(child);
            }
            reg.mark_mounted(element_id, zr_id)?;
            Ok(())
        })
    })
}

pub(crate) fn unmount_element_from_zr(zr_id: u32, element: &Element) -> Result<(), JsValue> {
    let element_id = element.raw_id();
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        reg.assert_belongs_to_zr(element_id, zr_id)?;
        let kind = reg.kind(element_id).unwrap();
        if kind != ElementKind::Text {
            let child = reg.child_ref(element_id)?;
            reg.mark_unmounted(element_id);
            with_zr(zr_id, |zr| {
                zr.storage.del_root(child);
                Ok(())
            })
        } else {
            reg.mark_unmounted(element_id);
            Ok(())
        }
    })
}

pub(crate) fn register_group() -> Element {
    let id = ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut()
            .register(ElementKind::Group, PendingData::group(), "group")
    });
    Element::from_id(id)
}

pub(crate) fn register_path(type_name: &str, opts: &JsValue) -> Result<Element, JsValue> {
    let pending = build_pending_path(type_name, opts)?;
    let id = ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut()
            .register(ElementKind::Path, pending, type_name)
    });
    Ok(Element::from_id(id))
}

pub(crate) fn register_text(opts: &JsValue) -> Element {
    let pending = build_pending_text(opts);
    let id = ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut()
            .register(ElementKind::Text, pending, "text")
    });
    Element::from_id(id)
}

pub(crate) fn register_image(opts: &JsValue) -> Result<Element, JsValue> {
    let pending = build_pending_image(opts)?;
    let id = ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut()
            .register(ElementKind::Image, pending, "image")
    });
    Ok(Element::from_id(id))
}

pub(crate) fn group_add_child(group_id: u32, child_id: u32) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        reg.set_parent(child_id, group_id)?;
        if let Some(zr_id) = reg.zr_id(group_id) {
            with_zr(zr_id, |zr| {
                reg.attach_to_zr(child_id, zr_id)?;
                reg.materialize_element(zr, zr_id, child_id)?;
                let group_idx = reg.storage_index(group_id).unwrap();
                let child = reg.child_ref(child_id)?;
                zr.storage.group_add_child(group_idx, child);
                Ok(())
            })
        } else {
            Ok(())
        }
    })
}

pub(crate) fn group_remove_child(group_id: u32, child_id: u32) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        let mut reg = reg.borrow_mut();
        if reg.parent_id(child_id) != Some(group_id) {
            return Err(JsValue::from_str("element is not a child of this group"));
        }

        if let Some(zr_id) = reg.zr_id(group_id) {
            if reg.storage_index(group_id).is_some() && reg.storage_index(child_id).is_some() {
                let group_idx = reg.storage_index(group_id).unwrap();
                let child = reg.child_ref(child_id)?;
                with_zr(zr_id, |zr| {
                    zr.storage.group_remove_child(group_idx, child);
                    Ok(())
                })?;
            }
        }

        reg.clear_parent(child_id);
        Ok(())
    })
}

pub(crate) fn path_use_state(element_id: u32, state: &str) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut().apply_path_state(element_id, state)
    })
}

pub(crate) fn path_set_state_style(
    element_id: u32,
    state: &str,
    patch: PathStylePatch,
) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut()
            .apply_path_state_style(element_id, state, patch)
    })
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use rust_zrender::ZRenderer;

    #[test]
    fn element_registry_tracks_mount_state() {
        use rust_zrender::{DisplayableProps, PathStyle, Shape, RectShape};

        let mut reg = ElementRegistry::default();
        let pending = PendingData::Path(crate::element::pending::PendingPath {
            shape: Shape::Rect(RectShape {
                x: 0.0,
                y: 0.0,
                width: 10.0,
                height: 10.0,
            }),
            style: PathStyle::default(),
            displayable: DisplayableProps::default(),
            silent: false,
            name: String::new(),
            ec_data: Default::default(),
            state_patches: HashMap::new(),
            active_states: Vec::new(),
        });
        let id = reg.register(ElementKind::Path, pending, "rect");
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
