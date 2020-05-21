use swayipc::{Connection, EventType, Fallible};
use swayipc::reply::Event;
use swayipc::reply::WindowChange;
use std::process::Command;



#[async_std::main]
async fn main() -> Fallible<()> {
    let subs = [
        EventType::Window,
    ];
    let mut events = Connection::new().await?.subscribe(&subs).await?;
    while let event = events.next().await {
        if let Event::Window(window) = event.unwrap() {
             let app_id = window.container.app_id;
             let change = window.change;
             if change !=  WindowChange::Focus {
                 continue;
             }

            Command::new("watson").arg("stop").status().expect("");
             if app_id != None {
                Command::new("watson").arg("start").arg(app_id.unwrap()).status().expect("");
             } else {
                 let wp = window.container.window_properties.unwrap();
                 let class = wp.class;

                Command::new("watson").arg("start").arg(class).status().expect("");
             }
        }
    }
    unreachable!();
}
