use sqlx::{Pool, Postgres, query};
use std::time::{SystemTime, UNIX_EPOCH};
use std::fs::metadata;

pub async fn insert_or_update_file(pool: &Pool<Postgres>, path: &str, hash: &str) -> sqlx::Result<()> {
    let meta = metadata(path)?;
    let last_modified = meta.modified()?.duration_since(UNIX_EPOCH)?.as_secs() as i64;
    let file_size = meta.len();

    query!(
        "INSERT INTO synced_files (file_path, file_hash, file_size, last_modified)
         VALUES ($1, $2, $3, $4)
         ON CONFLICT (file_path) DO UPDATE 
         SET file_hash = EXCLUDED.file_hash,
             file_size = EXCLUDED.file_size,
             last_modified = EXCLUDED.last_modified",
        path, hash, file_size, last_modified
    )
    .execute(pool)
    .await?;

    Ok(())
}