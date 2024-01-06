use std::sync::Arc;

use sqlx::PgPool;

use crate::models::CreateRecipe;

/// Repository for database operations.
#[cfg_attr(test, faux::create)]
#[derive(Clone)]
pub struct DbRepo {
    pool: PgPool,
}

#[cfg_attr(test, faux::methods)]
impl DbRepo {
    pub(crate) fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub(crate) async fn create_recipe(&self, recipe: CreateRecipe) -> Result<String, Arc<anyhow::Error>> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mocking_db_repo_works() {
        let mut repo = DbRepo::faux();
        let recipe = CreateRecipe::fixture("test recipe", None, None);

        faux::when!(repo.create_recipe(recipe.clone())).then_return(Ok("OK!".to_string()));
        assert_eq!(repo.create_recipe(recipe).await.unwrap(), "OK!");
    }
}
