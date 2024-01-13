use std::sync::Arc;

use anyhow::anyhow;
use sqlx::types::Uuid;
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

    pub(crate) async fn create_recipe(
        &self,
        recipe: CreateRecipe,
    ) -> Result<Uuid, Arc<anyhow::Error>> {
        match (recipe.book, recipe.url) {
            (Some(book), Some(url)) => {
                sqlx::query_file!(
                    "./src/infra/queries/create_recipe_book_url.sql",
                    recipe.name,
                    book.title,
                    book.isbn,
                    url.name,
                    url.url
                )
                .map(|r| r.id)
                .fetch_one(&mut *self.pool.acquire().await.map_err(|e| anyhow!(e))?)
                .await
            }
            (Some(book), None) => {
                sqlx::query_file!(
                    "./src/infra/queries/create_recipe_book.sql",
                    recipe.name,
                    book.title,
                    book.isbn
                )
                .map(|r| r.id)
                .fetch_one(&mut *self.pool.acquire().await.map_err(|e| anyhow!(e))?)
                .await
            }
            (None, Some(url)) => {
                sqlx::query_file!(
                    "./src/infra/queries/create_recipe_url.sql",
                    recipe.name,
                    url.name,
                    url.url
                )
                .map(|r| r.id)
                .fetch_one(&mut *self.pool.acquire().await.map_err(|e| anyhow!(e))?)
                .await
            }
            (None, None) => unreachable!("A recipe must always have a book or URL"),
        }
        .map_err(|e| Arc::new(anyhow!(e)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn mocking_db_repo_works() {
        let mut repo = DbRepo::faux();
        let recipe = CreateRecipe::fixture("test recipe", None, None);

        let uuid = Uuid::new_v4();
        faux::when!(repo.create_recipe(recipe.clone())).then_return(Ok(uuid));
        assert_eq!(repo.create_recipe(recipe).await.unwrap(), uuid);
    }
}
