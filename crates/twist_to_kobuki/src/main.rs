use std::io::stdout;

use anyhow::Result;
// use kobuki_interface::{
//     serial_port::SerialPortHandler,
//     tx::{ByteStream, commands},
// };

use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen},
};
use geometry_msgs::Twist;
use mio::{Events, Poll, PollOpt, Ready, Token};
use ros2_client::{MessageTypeName, Name};
use ros2_utils::{create_node, create_qos};

fn main() -> Result<()> {
    // Print with crossterm
    let mut stdout = stdout();
    execute!(stdout, EnterAlternateScreen, Hide)?;
    execute!(stdout, MoveTo(0, 0), Print("Latest twist message:\n"))?;

    // Set up Kobuki
    // let port = tokio_serial::new("/dev/kobuki", 115200)
    //     .timeout(Duration::from_millis(1024))
    //     .open_native_async()?;
    // let serial = SerialPortHandler::new(port);

    // Set up the ROS2 subscriber node
    let mut node = create_node();
    let qos_profile = create_qos();
    let topic = node
        .create_topic(
            &Name::new("/kobuki", "cmd_vel").unwrap(),
            MessageTypeName::new("geometry_msgs", "Twist"),
            &qos_profile,
        )
        .unwrap();
    let listener = node
        .create_subscription::<Twist>(&topic, Some(qos_profile))
        .unwrap();

    let poll = Poll::new().unwrap();

    poll.register(&listener, Token(1), Ready::readable(), PollOpt::edge())
        .unwrap();

    let mut events = Events::with_capacity(8);
    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                Token(1) => match listener.take() {
                    Ok(Some((message, _message_info))) => {
                        // Update display using crossterm
                        execute!(
                            stdout,
                            MoveTo(0, 4),
                            Clear(ClearType::FromCursorDown),
                            Print(format!("{:?}", message))
                        )?;
                    }
                    Ok(None) => {
                        execute!(stdout, MoveTo(0, 4), Clear(ClearType::FromCursorDown))?;
                    }
                    Err(e) => {
                        execute!(
                            stdout,
                            MoveTo(0, 4),
                            Print(format!(">>> error with response handling, e: {:?}", e))
                        )?;
                    }
                },
                _ => println!(">>> Unknown poll token {:?}", event.token()),
            } // match
        } // for
    } // loop
}
