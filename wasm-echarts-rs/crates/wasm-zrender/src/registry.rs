//! ZRender / Element 实例表

use std::cell::RefCell;
use std::collections::HashMap;

use rust_zrender::{
    ChildRef, Image, Path, PathStylePatch, Text, ZRenderer,
};
use wasm_bindgen::prelude::*;

use crate::bridge::build::{build_pending_image, build_pending_path, build_pending_text};
use crate::bridge::opts::DraggableKind;
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
    children: Vec<u32>,
    mounted: bool,
    type_name: String,
    pending: PendingData,
    listeners: HashMap<String, Vec<JsValue>>,
    draggable: DraggableKind,
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
        let draggable = pending.draggable();
        let id = self.next_id;
        self.next_id += 1;
        self.elements.insert(
            id,
            ElementRecord {
                kind,
                storage_index: None,
                zr_id: None,
                parent_id: None,
                children: Vec::new(),
                mounted: false,
                type_name: type_name.into(),
                pending,
                listeners: HashMap::new(),
                draggable,
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

    pub fn pending(&self, id: u32) -> Option<&PendingData> {
        self.elements.get(&id).map(|r| &r.pending)
    }

    pub fn pending_mut(&mut self, id: u32) -> Option<&mut PendingData> {
        self.elements.get_mut(&id).map(|r| &mut r.pending)
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
            ElementKind::Text => Ok(ChildRef::Text(idx)),
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
            .get(&parent_id)
            .map(|r| r.children.clone())
            .unwrap_or_default()
    }

    pub fn child_index(&self, parent_id: u32, child_id: u32) -> Option<usize> {
        self.elements
            .get(&parent_id)
            .and_then(|r| r.children.iter().position(|&id| id == child_id))
    }

    fn link_parent(&mut self, child_id: u32, parent_id: u32) -> Result<(), JsValue> {
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

    pub fn set_parent(&mut self, child_id: u32, parent_id: u32) -> Result<(), JsValue> {
        self.link_parent(child_id, parent_id)?;
        self.elements
            .get_mut(&parent_id)
            .ok_or_else(|| JsValue::from_str("invalid parent element"))?
            .children
            .push(child_id);
        Ok(())
    }

    pub fn insert_child(
        &mut self,
        parent_id: u32,
        child_id: u32,
        index: usize,
    ) -> Result<(), JsValue> {
        self.link_parent(child_id, parent_id)?;
        let parent = self
            .elements
            .get_mut(&parent_id)
            .ok_or_else(|| JsValue::from_str("invalid parent element"))?;
        let index = index.min(parent.children.len());
        parent.children.insert(index, child_id);
        Ok(())
    }

    pub fn replace_child(
        &mut self,
        parent_id: u32,
        old_id: u32,
        new_id: u32,
    ) -> Result<usize, JsValue> {
        let index = self
            .child_index(parent_id, old_id)
            .ok_or_else(|| JsValue::from_str("element is not a child of this group"))?;
        self.clear_parent(old_id);
        self.link_parent(new_id, parent_id)?;
        let parent = self
            .elements
            .get_mut(&parent_id)
            .ok_or_else(|| JsValue::from_str("invalid parent element"))?;
        if index <= parent.children.len() {
            parent.children.insert(index, new_id);
        } else {
            parent.children.push(new_id);
        }
        Ok(index)
    }

    pub fn clear_parent(&mut self, child_id: u32) {
        let parent_id = self.parent_id(child_id);
        if let Some(record) = self.elements.get_mut(&child_id) {
            record.parent_id = None;
        }
        if let Some(pid) = parent_id {
            if let Some(parent) = self.elements.get_mut(&pid) {
                parent.children.retain(|&id| id != child_id);
            }
        }
    }

    pub fn parent_id(&self, id: u32) -> Option<u32> {
        self.elements.get(&id).and_then(|r| r.parent_id)
    }

    pub fn add_listener(&mut self, id: u32, event: &str, handler: JsValue) {
        if handler.is_function() {
            if let Some(record) = self.elements.get_mut(&id) {
                record
                    .listeners
                    .entry(event.to_string())
                    .or_default()
                    .push(handler);
            }
        }
    }

    pub fn remove_listener(&mut self, id: u32, event: Option<&str>, handler: Option<&JsValue>) {
        let Some(record) = self.elements.get_mut(&id) else {
            return;
        };
        match (event, handler) {
            (None, _) => record.listeners.clear(),
            (Some(name), None) => {
                record.listeners.remove(name);
            }
            (Some(name), Some(h)) => {
                if let Some(list) = record.listeners.get_mut(name) {
                    list.retain(|existing| existing != h);
                    if list.is_empty() {
                        record.listeners.remove(name);
                    }
                }
            }
        }
    }

    pub fn listeners(&self, id: u32, event: &str) -> Vec<JsValue> {
        self.elements
            .get(&id)
            .and_then(|r| r.listeners.get(event))
            .cloned()
            .unwrap_or_default()
    }

    pub fn mounted_root_ids(&self, zr_id: u32) -> Vec<u32> {
        self.elements
            .iter()
            .filter(|(_, r)| r.zr_id == Some(zr_id) && r.mounted && r.parent_id.is_none())
            .map(|(id, _)| *id)
            .collect()
    }

    pub fn draggable(&self, id: u32) -> DraggableKind {
        self.elements
            .get(&id)
            .map(|r| r.draggable)
            .unwrap_or_default()
    }

    pub fn set_draggable(&mut self, id: u32, kind: DraggableKind) {
        if let Some(record) = self.elements.get_mut(&id) {
            record.draggable = kind;
        }
    }

    pub fn find_draggable_ancestor(&self, mut id: u32) -> Option<u32> {
        loop {
            if !self.draggable(id).is_none() {
                return Some(id);
            }
            match self.parent_id(id) {
                Some(parent) => id = parent,
                None => return None,
            }
        }
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
        let pending = self
            .elements
            .get(&element_id)
            .and_then(|r| {
                if let PendingData::Group(g) = &r.pending {
                    Some(g.clone())
                } else {
                    None
                }
            })
            .unwrap_or_default();
        let idx = zr.storage.create_group();
        {
            let group = zr.storage.group_mut(idx);
            pending.transform.apply_to_base(&mut group.base);
            group.base.name = pending.name;
            group.base.ignore = pending.ignore;
        }
        if let Some(record) = self.elements.get_mut(&element_id) {
            record.storage_index = Some(idx);
            record.zr_id = Some(zr_id);
        }
        if let Some(clip_id) = pending.clip_element_id {
            self.materialize_element(zr, zr_id, clip_id)?;
            if let Some(clip_idx) = self.storage_index(clip_id) {
                zr.storage.group_mut(idx).clip_path = Some(clip_idx);
            }
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
        pending.transform.apply_to_base(&mut path.base);
        path.silent = pending.silent;
        path.base.name = pending.name;
        path.base.ignore = pending.ignore;

        for (state, patch) in &pending.state_patches {
            path.states.set_state_patch(state, patch.clone());
        }
        if !pending.active_states.is_empty() {
            let names: Vec<&str> = pending.active_states.iter().map(|s| s.as_str()).collect();
            path.use_states(&names);
        }

        if let Some(clip_id) = pending.clip_element_id {
            self.materialize_element(zr, zr_id, clip_id)?;
            if let Some(clip_idx) = self.storage_index(clip_id) {
                path.clip_path = Some(clip_idx);
            }
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
        text.base.ignore = pending.ignore;
        pending.transform.apply_to_base(&mut text.base);

        let idx = zr.storage.create_text(text);
        if let Some(record) = self.elements.get_mut(&element_id) {
            record.storage_index = Some(idx);
            record.zr_id = Some(zr_id);
        }
        if let Some(clip_id) = pending.clip_element_id {
            self.materialize_element(zr, zr_id, clip_id)?;
            if let Some(clip_idx) = self.storage_index(clip_id) {
                zr.storage.text_mut(idx).clip_path = Some(clip_idx);
            }
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
        image.base.ignore = pending.ignore;
        pending.transform.apply_to_base(&mut image.base);

        let idx = zr.storage.create_image(image);
        if let Some(record) = self.elements.get_mut(&element_id) {
            record.storage_index = Some(idx);
            record.zr_id = Some(zr_id);
        }
        if let Some(clip_id) = pending.clip_element_id {
            self.materialize_element(zr, zr_id, clip_id)?;
            if let Some(clip_idx) = self.storage_index(clip_id) {
                zr.storage.image_mut(idx).clip_path = Some(clip_idx);
            }
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

    pub fn apply_path_states(&mut self, element_id: u32, states: &[String]) -> Result<(), JsValue> {
        if self.storage_index(element_id).is_none() {
            if let Some(record) = self.elements.get_mut(&element_id) {
                if let PendingData::Path(pending) = &mut record.pending {
                    pending.active_states = states.to_vec();
                    return Ok(());
                }
            }
            return Err(JsValue::from_str("invalid path element"));
        }

        let zr_id = self
            .zr_id(element_id)
            .ok_or_else(|| JsValue::from_str("element is not attached to a ZRender instance"))?;
        let path_index = self.storage_index(element_id).unwrap();
        let refs: Vec<&str> = states.iter().map(|s| s.as_str()).collect();
        with_zr(zr_id, |zr| {
            zr.set_path_states(path_index, &refs);
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
            let child = reg.child_ref(element_id)?;
            zr.storage.add_root(child);
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
        let child = reg.child_ref(element_id)?;
        reg.mark_unmounted(element_id);
        with_zr(zr_id, |zr| {
            zr.storage.del_root(child);
            Ok(())
        })
    })
}

pub(crate) fn clear_zr(zr_id: u32) -> Result<(), JsValue> {
    let root_ids = ELEMENT_REGISTRY.with(|reg| reg.borrow().mounted_root_ids(zr_id));
    for id in root_ids {
        unmount_element_from_zr(zr_id, &Element::from_id(id))?;
    }
    with_zr(zr_id, |zr| {
        zr.storage.del_all_roots();
        Ok(())
    })?;
    crate::handler::paint_if_bound(zr_id);
    Ok(())
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

pub(crate) fn register_tspan(opts: &JsValue) -> Element {
    let pending = build_pending_text(opts);
    let id = ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut()
            .register(ElementKind::Text, pending, "tspan")
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
    let zr_id = ELEMENT_REGISTRY.with(|reg| -> Result<Option<u32>, JsValue> {
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
            })?;
            Ok(Some(zr_id))
        } else {
            Ok(None)
        }
    })?;
    if let Some(zr_id) = zr_id {
        crate::handler::paint_if_bound(zr_id);
    }
    Ok(())
}

pub(crate) fn group_insert_child(
    group_id: u32,
    child_id: u32,
    index: usize,
) -> Result<(), JsValue> {
    let zr_id = ELEMENT_REGISTRY.with(|reg| -> Result<Option<u32>, JsValue> {
        let mut reg = reg.borrow_mut();
        reg.insert_child(group_id, child_id, index)?;
        if let Some(zr_id) = reg.zr_id(group_id) {
            with_zr(zr_id, |zr| {
                reg.attach_to_zr(child_id, zr_id)?;
                reg.materialize_element(zr, zr_id, child_id)?;
                let group_idx = reg.storage_index(group_id).unwrap();
                let child = reg.child_ref(child_id)?;
                zr.storage.group_insert_child(group_idx, child, index);
                Ok(())
            })?;
            Ok(Some(zr_id))
        } else {
            Ok(None)
        }
    })?;
    if let Some(zr_id) = zr_id {
        crate::handler::paint_if_bound(zr_id);
    }
    Ok(())
}

pub(crate) fn group_replace_child(
    group_id: u32,
    old_id: u32,
    new_id: u32,
) -> Result<(), JsValue> {
    let zr_id = ELEMENT_REGISTRY.with(|reg| -> Result<Option<u32>, JsValue> {
        let mut reg = reg.borrow_mut();
        if reg.parent_id(old_id) != Some(group_id) {
            return Err(JsValue::from_str("element is not a child of this group"));
        }
        let index = reg.replace_child(group_id, old_id, new_id)?;
        if let Some(zr_id) = reg.zr_id(group_id) {
            if let Some(group_idx) = reg.storage_index(group_id) {
                let old_child = reg.child_ref(old_id).ok();
                with_zr(zr_id, |zr| {
                    if let Some(old_child) = old_child {
                        zr.storage.group_remove_child(group_idx, old_child);
                    }
                    reg.attach_to_zr(new_id, zr_id)?;
                    reg.materialize_element(zr, zr_id, new_id)?;
                    let new_child = reg.child_ref(new_id)?;
                    zr.storage.group_insert_child(group_idx, new_child, index);
                    Ok(())
                })?;
            }
            Ok(Some(zr_id))
        } else {
            Ok(None)
        }
    })?;
    if let Some(zr_id) = zr_id {
        crate::handler::paint_if_bound(zr_id);
    }
    Ok(())
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

pub(crate) fn path_use_states(element_id: u32, states: &[String]) -> Result<(), JsValue> {
    ELEMENT_REGISTRY.with(|reg| {
        reg.borrow_mut().apply_path_states(element_id, states)
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

    use crate::bridge::hit::hit_to_hover_result;
    use crate::element::Element;

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
                ..Default::default()
            }),
            style: PathStyle::default(),
            displayable: DisplayableProps::default(),
            silent: false,
            name: String::new(),
            ignore: false,
            ec_data: Default::default(),
            state_patches: HashMap::new(),
            active_states: Vec::new(),
            transform: Default::default(),
            clip_element_id: None,
            draggable: Default::default(),
        });
        let id = reg.register(ElementKind::Path, pending, "rect");
        assert!(reg.can_add_to_zr(id));
        reg.mark_mounted(id, 1).unwrap();
        assert!(reg.is_mounted(id));
        assert!(!reg.can_add_to_zr(id));
    }

    #[test]
    fn mount_group_text_find_hover_end_to_end() {
        use rust_zrender::{
            DisplayableProps, TextAlign, TextBaseline, TextStyle, ZRenderer,
        };

        ELEMENT_REGISTRY.with(|reg| reg.borrow_mut().clear());
        ZR_REGISTRY.with(|reg| reg.borrow_mut().clear());

        let zr = ZRenderer::new(320, 160).unwrap();
        let zr_id = ZR_REGISTRY.with(|reg| reg.borrow_mut().insert(zr));

        let mut reg = ElementRegistry::default();
        let group_id = reg.register(ElementKind::Group, PendingData::group(), "group");
        let pending = PendingData::Text(crate::element::pending::PendingText {
            content: "Hover".into(),
            x: 50.0,
            y: 50.0,
            style: TextStyle {
                font_size: 20.0,
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                ..Default::default()
            },
            displayable: DisplayableProps::default(),
            silent: false,
            name: String::new(),
            ignore: false,
            ec_data: Default::default(),
            transform: Default::default(),
            draggable: Default::default(),
            clip_element_id: None,
        });
        let text_id = reg.register(ElementKind::Text, pending, "text");
        reg.set_parent(text_id, group_id).unwrap();

        ELEMENT_REGISTRY.with(|global| {
            *global.borrow_mut() = reg;
        });

        let group_el = Element::from_id(group_id);
        mount_element_to_zr(zr_id, &group_el).unwrap();

        let hit = with_zr(zr_id, |zr| Ok(zr.find_hover(60.0, 55.0)))
            .unwrap()
            .expect("handler hit");
        assert!(matches!(hit.target, rust_zrender::HitTarget::Text(_)));
        let hover = hit_to_hover_result(&hit).expect("registry maps text storage index");
        assert_eq!(hover.target().element_type(), "text");
    }

    #[test]
    fn text_in_group_mount_and_hover() {
        use rust_zrender::{
            DisplayableProps, TextAlign, TextBaseline, TextStyle, ZRenderer,
        };

        let mut reg = ElementRegistry::default();
        let group_id = reg.register(ElementKind::Group, PendingData::group(), "group");
        let pending = PendingData::Text(crate::element::pending::PendingText {
            content: "Hover Label".into(),
            x: 50.0,
            y: 50.0,
            style: TextStyle {
                font_size: 20.0,
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                ..Default::default()
            },
            displayable: DisplayableProps::default(),
            silent: false,
            name: String::new(),
            ignore: false,
            ec_data: Default::default(),
            transform: Default::default(),
            draggable: Default::default(),
            clip_element_id: None,
        });
        let text_id = reg.register(ElementKind::Text, pending, "text");
        reg.set_parent(text_id, group_id).unwrap();

        let mut zr = ZRenderer::new(320, 160).unwrap();
        reg.materialize_tree(&mut zr, 1, group_id).unwrap();
        let group_idx = reg.storage_index(group_id).unwrap();
        zr.storage
            .add_root(rust_zrender::ChildRef::Group(group_idx));

        let hit = rust_zrender::Handler::find_hover(&mut zr.storage, 60.0, 55.0)
            .expect("text in group should be hittable");
        assert!(matches!(hit.target, rust_zrender::HitTarget::Text(_)));
        assert!(reg.find_by_storage(ElementKind::Text, 0).is_some());
    }

    #[test]
    fn text_mount_and_find_by_storage() {
        use rust_zrender::{
            DisplayableProps, TextAlign, TextBaseline, TextStyle, ZRenderer,
        };

        let mut reg = ElementRegistry::default();
        let pending = PendingData::Text(crate::element::pending::PendingText {
            content: "Hover".into(),
            x: 50.0,
            y: 50.0,
            style: TextStyle {
                font_size: 20.0,
                align: TextAlign::Left,
                baseline: TextBaseline::Top,
                ..Default::default()
            },
            displayable: DisplayableProps::default(),
            silent: false,
            name: String::new(),
            ignore: false,
            ec_data: Default::default(),
            transform: Default::default(),
            draggable: Default::default(),
            clip_element_id: None,
        });
        let element_id = reg.register(ElementKind::Text, pending, "text");
        let mut zr = ZRenderer::new(320, 160).unwrap();
        reg.materialize_tree(&mut zr, 1, element_id).unwrap();
        let child = reg.child_ref(element_id).unwrap();
        zr.storage.add_root(child);

        let hit = rust_zrender::Handler::find_hover(&mut zr.storage, 60.0, 55.0)
            .expect("text should be hittable");
        assert!(matches!(hit.target, rust_zrender::HitTarget::Text(_)));
        assert!(reg.find_by_storage(ElementKind::Text, 0).is_some());
    }

    #[test]
    fn group_children_preserve_insertion_order() {
        let mut reg = ElementRegistry::default();
        let group_id = reg.register(ElementKind::Group, PendingData::group(), "group");
        let a = reg.register(ElementKind::Group, PendingData::group(), "group");
        let b = reg.register(ElementKind::Group, PendingData::group(), "group");
        let c = reg.register(ElementKind::Group, PendingData::group(), "group");
        reg.set_parent(b, group_id).unwrap();
        reg.insert_child(group_id, a, 0).unwrap();
        assert_eq!(reg.children_of(group_id), vec![a, b]);
        let idx = reg.replace_child(group_id, b, c).unwrap();
        assert_eq!(idx, 1);
        assert_eq!(reg.children_of(group_id), vec![a, c]);
        assert_eq!(reg.parent_id(b), None);
        assert_eq!(reg.parent_id(c), Some(group_id));
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
