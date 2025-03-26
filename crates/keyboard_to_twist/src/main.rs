use std::{io::stdout, thread, time::Duration};

use crossterm::{
    cursor::{Hide, MoveTo},
    execute,
    style::Print,
    terminal::{Clear, ClearType, EnterAlternateScreen},
};
use device_query::{DeviceQuery, DeviceState, Keycode};
use geometry_msgs::Twist;
use ros2_client::{MessageTypeName, Name};
use ros2_utils::{create_node, create_qos};

fn main() -> std::io::Result<()> {
    // ROS
    let mut node = create_node();
    let qos = create_qos();
    let topic = node
        .create_topic(
            &Name::new("/kobuki", "cmd_vel").unwrap(),
            MessageTypeName::new("geometry_msgs", "Twist"),
            &qos,
        )
        .unwrap();
    let publisher = node.create_publisher::<Twist>(&topic, Some(qos)).unwrap();

    // Keyboard listening
    let mut stdout = stdout();
    let device_state = DeviceState::new();

    // Print instructions using crossterm
    execute!(stdout, EnterAlternateScreen, Hide)?;
    execute!(
        stdout,
        MoveTo(0, 0),
        Print("Up/Down or W/S: Linear velocity\n"),
        Print("Left/Right or A/D: Angular velocity\n\n")
    )?;

    let mut last_msg = Twist::ZERO;

    loop {
        let keys: Vec<Keycode> = device_state.get_keys();
        let mut msg = Twist::ZERO;
        for key in keys.iter() {
            match key {
                // Forwards
                Keycode::Up => msg.linear.x += 1.0,
                Keycode::W => msg.linear.x += 1.0,
                // Backwards
                Keycode::Down => msg.linear.x -= 1.0,
                Keycode::S => msg.linear.x -= 1.0,
                // Left
                Keycode::Left => msg.angular.z += 1.0,
                Keycode::A => msg.angular.z += 1.0,
                // Right
                Keycode::Right => msg.angular.z -= 1.0,
                Keycode::D => msg.angular.z -= 1.0,
                _ => {}
            }
        }

        // Update display using crossterm
        execute!(
            stdout,
            MoveTo(0, 4),
            Clear(ClearType::FromCursorDown),
            Print(format!(
                "Linear velocity: {:.2}, Angular velocity: {:.2}",
                msg.linear.x, msg.angular.z
            ))
        )?;

        // Only publish if message is different from last message
        if msg != last_msg {
            publisher.publish(msg.clone()).unwrap();
            last_msg = msg.clone();
        }

        // Sleep for 10ms
        thread::sleep(Duration::from_millis(1000 / 60));
    }
}
