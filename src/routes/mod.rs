use axum::extract::State;
use axum::Json;

use crate::AppState;

use crate::models::CreateRecipe;

pub async fn create_recipe(
    State(state): State<AppState>,
    Json(recipe): Json<CreateRecipe>,
) -> String {
    let res = &state.svc.create_recipe(recipe).await;
    match res {
        Ok(s) => s.to_string(),
        Err(e) => e.to_string(),
    }
}
