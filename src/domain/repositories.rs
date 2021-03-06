use anyhow::Result;

use crate::domain::models::entities::*;

pub trait Repository {
    fn create_entity(&self, entity: Entity) -> Result<Entity>;
    fn select_entity(&self, id: u32) -> Result<Entity>;
    fn update_entity(&self, entity: Entity) -> Result<Entity>;
    fn delete_entity(&self, id: u32) -> Result<()>;
}
