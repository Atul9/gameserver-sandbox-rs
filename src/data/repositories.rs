use anyhow::Result;
use sled::Config;

use crate::data;
use crate::domain;
use crate::data::queries;

#[derive(Clone, Debug)]
pub struct Repository {
    connection: sled::Db
}

impl Repository {
    pub fn new() -> Result<Repository> {
        let config = Config::new().temporary(true);
        Ok(Repository { connection: config.open()? })
    }

    pub fn conn(&self) -> &sled::Db {
        &self.connection
    }
}

impl domain::repositories::Repository for Repository {
    fn create_entity(&self, entity: domain::models::entities::Entity) -> Result<domain::models::entities::Entity> {
        use data::models::entities::NewEntity;

        let entity = NewEntity{ id: entity.id, pos: entity.pos };
        let result = queries::entities::insert(&self, entity)?;

        Ok(domain::models::entities::Entity { id: result.id, pos: result.pos })
    }

    fn select_entity(&self, id: u32) -> Result<domain::models::entities::Entity> {
        let result = queries::entities::select_one(&self, id)?;

        Ok(domain::models::entities::Entity { id: result.id, pos: result.pos })
    }

    fn update_entity(&self, entity: domain::models::entities::Entity) -> Result<domain::models::entities::Entity> {
        use data::models::entities::UpdateEntity;

        let entity = UpdateEntity{ id: entity.id, pos: entity.pos };
        let result = queries::entities::update(&self, entity)?;

        Ok(domain::models::entities::Entity { id: result.id, pos: result.pos })
    }

    fn delete_entity(&self, id: u32) -> Result<()> {
        Ok(queries::entities::delete(&self, id)?)
    }
}
