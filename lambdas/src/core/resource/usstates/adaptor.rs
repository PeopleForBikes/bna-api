use crate::{
    core::resource::usstates::db::{fetch_us_state, fetch_us_states},
    database_connect, Context, ExecutionError,
};
use entity::us_state;
use tracing::info;

pub async fn get_us_state_adaptor(
    name: &str,
    ctx: Context,
) -> Result<us_state::Model, ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Fetch the us_state model.
    let model = match fetch_us_state(&db, name).await {
        Ok(model) => model,
        Err(e) => {
            info!("{e:?}");
            return Err(e.into());
        }
    };
    match model {
        Some(model) => Ok(model),
        None => Err(ExecutionError::NotFound(
            ctx.request_id(),
            ctx.source(),
            format!("cannot find a US state matching `{name}` (case sensitive)"),
        )),
    }
}

pub async fn get_us_states_adaptor(
    page: u64,
    page_size: u64,
) -> Result<(u64, Vec<entity::us_state::Model>), ExecutionError> {
    // Set the database connection.
    let db = database_connect().await?;

    // Fetch a page of US states.
    Ok(fetch_us_states(&db, page, page_size).await?)
}
