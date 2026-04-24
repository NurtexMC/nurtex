use crate::bot::BotComponents;
use nurtex_protocol::connection::NurtexConnection;
use std::io;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Функция временного захвата подключения
pub async fn capture_connection<F>(connection: &Arc<RwLock<Option<NurtexConnection>>>, f: F) -> io::Result<()>
where
  F: AsyncFnOnce(&NurtexConnection) -> io::Result<()>,
{
  let guard = connection.read().await;
  if let Some(conn) = guard.as_ref() {
    f(conn).await?;
  }
  Ok(())
}

/// Функция временного захвата компонентов
pub async fn capture_components<F>(components: &Arc<RwLock<BotComponents>>, f: F) -> io::Result<()>
where
  F: AsyncFnOnce(&mut BotComponents) -> io::Result<()>,
{
  let mut guard = components.write().await;
  f(&mut *guard).await
}
