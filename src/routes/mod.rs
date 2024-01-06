use axum::extract::State;
use axum::Json;
use serde_json::json;

use crate::AppState;

use crate::models::CreateRecipe;

pub async fn create_recipe(
    State(state): State<AppState>,
    Json(recipe): Json<CreateRecipe>,
) -> String {
    // TODO:
    // let res = &state.svc.create_recipe(recipe).await;

    // Hackz
    json!(recipe).to_string()
}
