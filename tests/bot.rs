#[cfg(test)]
mod tests {
  use std::io;

  use nurtex::bot::Bot;
  use nurtex::events::EventInvoker;

  #[tokio::test]
  async fn launch_bot() -> io::Result<()> {
    let bot = Bot::new("NurtexBot");

    let mut event_invoker = EventInvoker::new();

    event_invoker.on_spawn(|terminal| async move {
      println!("Бот {} заспавнился!", terminal.receiver);
    });

    event_invoker.on_chat(|terminal, payload| async move {
      println!(
        "Бот {} получил сообщение: {}",
        terminal.receiver, payload.message
      );
    });

    event_invoker.on_disconnect(|terminal, payload| async move {
      println!(
        "Бот {} отключился по причине: {}",
        terminal.receiver, payload.reason
      );
    });

    bot
      .set_event_invoker(event_invoker)
      .connect_to("localhost", 25565)
      .await?;

    Ok(())
  }
}
