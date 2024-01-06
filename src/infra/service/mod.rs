use std::sync::Arc;

use crate::models::CreateRecipe;

use super::repository::DbRepo;

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
    pub async fn create_recipe(&self, recipe: CreateRecipe) -> Result<String, Arc<anyhow::Error>> {
        self.repo.create_recipe(recipe).await
    }
}
