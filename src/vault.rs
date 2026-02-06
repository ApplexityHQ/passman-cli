use crate::models::PasswordEntry;
use anyhow::{Result, bail};

#[derive(Debug)]
pub struct Vault {
    pub(crate) entries: Vec<PasswordEntry>,
}

impl Vault {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
        }
    }

    pub fn add_entry(&mut self, entry: PasswordEntry) -> Result<()> {
        if self.entries.iter().any(|e| e.service == entry.service) {
            bail!("Service already exists");
        }

        self.entries.push(entry);
        Ok(())
    }


    pub fn get_entry(&self, service: &str) -> Option<&PasswordEntry> {
        self.entries.iter().find(|e| e.service == service)
    }

    pub fn delete_entry(&mut self, service: &str) -> Result<()> {
        let index = self
            .entries
            .iter()
            .position(|e| e.service == service)
            .ok_or_else(|| anyhow::anyhow!("Service not found"))?;

        self.entries.remove(index);
        Ok(())
    }

    pub fn list_services(&self) -> Vec<&str> {
        self.entries
            .iter()
            .map(|e| e.service.as_str())
            .collect()
    }

}







/*
    Business logic
    core domain logic
*/
