use crate::{string_to_field, EResult, State};
use std::env::var;
use std::sync::Arc;
use tokio::{
    fs::{create_dir_all, File},
    io::{AsyncSeekExt, AsyncWriteExt, SeekFrom},
    sync::RwLock,
    time::{sleep, Duration},
};
use tracing::{debug, instrument};

const DEFAULT_DIRECTORY: &'static str = "fields";
const FILE_REFRESH_RATE: u64 = 10;

#[instrument(skip(shared_state))]
pub async fn handler(shared_state: Arc<RwLock<State>>) -> EResult<()> {
    let dir = var("KOUNT_DIR").unwrap_or(DEFAULT_DIRECTORY.to_string());
    create_dir_all(&dir).await?;
    debug!({ ?dir }, "created counter directory");

    tokio::select! {
        res = tokio::spawn(field_handler(shared_state.clone(), dir.clone(), "keyboard")) => { res??; }
    }
    Ok(())
}

#[instrument(skip(shared_state))]
async fn field_handler(
    shared_state: Arc<RwLock<State>>,
    directory: String,
    field: &str,
) -> EResult<()> {
    let mut file = File::create(format!("{}/{}.txt", directory, field)).await?;
    loop {
        let field_count = {
            let state = shared_state.read().await;
            string_to_field!(field, state).unwrap()
        };
        file.seek(SeekFrom::Start(0)).await?;
        file.write(field_count.to_string().as_bytes()).await?;
        file.flush().await?;

        sleep(Duration::from_millis(1000 / FILE_REFRESH_RATE)).await;
    }
}
