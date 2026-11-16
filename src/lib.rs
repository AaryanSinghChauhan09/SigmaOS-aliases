// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-aliases: aliases component model.
//!
//! Real implementation: aliases_items, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AliasesItem {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AliasesError {
    EmptyAliasesItemName,
    DuplicateAliasesItem(String),
    EmptyEventSubject,
}

impl fmt::Display for AliasesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyAliasesItemName => write!(f, "aliases_item name must not be empty"),
            Self::DuplicateAliasesItem(id) => write!(f, "duplicate aliases_item {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for AliasesError {}

pub struct Aliases {
    pub name: String,
    aliases_items: BTreeMap<String, AliasesItem>,
    events: BTreeMap<String, Event>,
}

impl Aliases {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            aliases_items: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_aliases_item(&mut self, item: AliasesItem) -> Result<(), AliasesError> {
        if item.name.trim().is_empty() {
            return Err(AliasesError::EmptyAliasesItemName);
        }
        let id = item.id.clone();
        if self.aliases_items.contains_key(&id) {
            return Err(AliasesError::DuplicateAliasesItem(id.clone()));
        }
        self.aliases_items.insert(id, item);
        Ok(())
    }

    pub fn aliases_item_count(&self) -> usize {
        self.aliases_items.len()
    }

    pub fn get_aliases_item(&self, id: &str) -> Option<&AliasesItem> {
        self.aliases_items.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), AliasesError> {
        if event.subject.trim().is_empty() {
            return Err(AliasesError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_aliases_item(id: &str, name: &str) -> AliasesItem {
        AliasesItem {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = Aliases::new("Aliases");
        assert_eq!(m.name, "Aliases");
        assert_eq!(m.aliases_item_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_aliases_item() {
        let mut m = Aliases::new("Aliases");
        m.add_aliases_item(sample_aliases_item("a1", "Alpha")).unwrap();
        assert_eq!(m.aliases_item_count(), 1);
        assert!(m.get_aliases_item("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = Aliases::new("Aliases");
        assert!(m.add_aliases_item(sample_aliases_item("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = Aliases::new("Aliases");
        m.add_aliases_item(sample_aliases_item("x", "One")).unwrap();
        assert!(m.add_aliases_item(sample_aliases_item("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = Aliases::new("Aliases");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
