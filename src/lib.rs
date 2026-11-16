// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-service-dependency: service dependency component model.
//!
//! Real implementation: dependencys, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
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
pub enum ServiceDependencyError {
    EmptyDependencyName,
    DuplicateDependency(String),
    EmptyEventSubject,
}

impl fmt::Display for ServiceDependencyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyDependencyName => write!(f, "dependency name must not be empty"),
            Self::DuplicateDependency(id) => write!(f, "duplicate dependency {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ServiceDependencyError {}

pub struct ServiceDependency {
    pub name: String,
    dependencys: BTreeMap<String, Dependency>,
    events: BTreeMap<String, Event>,
}

impl ServiceDependency {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            dependencys: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_dependency(&mut self, item: Dependency) -> Result<(), ServiceDependencyError> {
        if item.name.trim().is_empty() {
            return Err(ServiceDependencyError::EmptyDependencyName);
        }
        let id = item.id.clone();
        if self.dependencys.contains_key(&id) {
            return Err(ServiceDependencyError::DuplicateDependency(id.clone()));
        }
        self.dependencys.insert(id, item);
        Ok(())
    }

    pub fn dependency_count(&self) -> usize {
        self.dependencys.len()
    }

    pub fn get_dependency(&self, id: &str) -> Option<&Dependency> {
        self.dependencys.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ServiceDependencyError> {
        if event.subject.trim().is_empty() {
            return Err(ServiceDependencyError::EmptyEventSubject);
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

    fn sample_dependency(id: &str, name: &str) -> Dependency {
        Dependency {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ServiceDependency::new("ServiceDependency");
        assert_eq!(m.name, "ServiceDependency");
        assert_eq!(m.dependency_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_dependency() {
        let mut m = ServiceDependency::new("ServiceDependency");
        m.add_dependency(sample_dependency("a1", "Alpha")).unwrap();
        assert_eq!(m.dependency_count(), 1);
        assert!(m.get_dependency("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ServiceDependency::new("ServiceDependency");
        assert!(m.add_dependency(sample_dependency("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ServiceDependency::new("ServiceDependency");
        m.add_dependency(sample_dependency("x", "One")).unwrap();
        assert!(m.add_dependency(sample_dependency("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ServiceDependency::new("ServiceDependency");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
