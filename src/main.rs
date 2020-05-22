use swayipc::{Connection, EventType, Fallible};
use swayipc::reply::Event;
use swayipc::reply::WindowChange;
use std::process::Command;
use std::process;


use std::{thread};
use signal_hook::{iterator::Signals, SIGTERM, SIGINT};



#[async_std::main]
async fn main() -> Fallible<()> {
    let subs = [
        EventType::Window,
    ];
    let signals = Signals::new(&[SIGTERM, SIGINT])?;

        thread::spawn(move || {
        for sig in signals.forever() {
            println!("stoping watson {:?}\n", sig);
            Command::new("watson").arg("stop").status().expect("");
            process::exit(0);
        }
    });


    let mut events = Connection::new().await?.subscribe(&subs).await?;
    loop {
        let event = events.next().await;
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
}
