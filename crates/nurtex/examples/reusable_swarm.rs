use std::sync::Arc;
use std::time::Duration;

use nurtex::bot::{Bot, BotChatExt};
use nurtex::swarm::{JoinDelay, Swarm};

#[tokio::main]
async fn main() -> std::io::Result<()> {
  // Создаём рой и список ботов
  let mut swarm = Swarm::create();
  let mut bots = Vec::new();

  // Добавляем ботов в наш список
  for i in 0..6 {
    bots.push(Arc::new(Bot::create(format!("nurtex_bot_{}", i))));
  }

  // Создаём цикл на 3 повторения
  for i in 0..3 {
    // Добавляем наших ботов.
    // Важно: Нужно добавлять ботов каждый раз после `shutdown` (выключения роя)
    for bot in &bots {
      swarm.add_arc_bot(Arc::clone(bot));
    }
    
    // Запускаем ботов на сервер с фиксированной задержкой в 200мс
    swarm.launch("localhost", 25565, JoinDelay::fixed(200)).await;

    // Ждём немножко
    tokio::time::sleep(Duration::from_secs(3)).await;

    // Параллельно проходимся по всем ботам из роя
    swarm.for_each_parallel(async |bot| {
      // Отправляем сообщение в чат и игнорируем возможные ошибки
      let _ = bot.chat_message("Привет, мир!").await;
    });

    // Ждём немножко
    tokio::time::sleep(Duration::from_secs(1)).await;

    // Отключаем и очищаем рой
    swarm.shutdown().await?;
    
    // Ждём перед следующим запуском (за исключением последнего запуска)
    if i != 2 {
      tokio::time::sleep(Duration::from_secs(2)).await;
    }
  }

  Ok(())
}
