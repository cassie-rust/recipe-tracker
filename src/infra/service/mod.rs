use std::sync::Arc;

use sqlx::types::Uuid;

use super::repository::DbRepo;
use crate::models::CreateRecipe;

#[derive(Clone)]
pub struct DbService {
    repo: DbRepo,
}

impl From<DbRepo> for DbService {
    fn from(repo: DbRepo) -> Self {
        Self { repo }
    }
}

impl DbService {
    pub async fn create_recipe(&self, recipe: CreateRecipe) -> Result<Uuid, Arc<anyhow::Error>> {
        self.repo.create_recipe(recipe).await
    }
}
