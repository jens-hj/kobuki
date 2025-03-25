use std::cmp::min;

use anyhow::Result;
use kobuki_interface::{
    serial_port::SerialPortHandler,
    tx::{ByteStream, commands},
};

use mio::{Events, Poll, PollOpt, Ready, Token};
use ros2_client::{
    Context, MessageTypeName, Name, Node, NodeName, NodeOptions,
    ros2::{
        Duration, QosPolicies, QosPolicyBuilder,
        policy::{self, Deadline, Lifespan},
    },
};

#[tokio::main]
async fn main() -> Result<()> {
    // Set up the ROS2 subscriber node
    let mut node = create_node();
    let qos_profile = create_qos();
    let topic = node
        .create_topic(
            &Name::new("/", "kobuki/move").unwrap(),
            MessageTypeName::new("geometry_msgs", "Twist"),
            &qos_profile,
        )
        .unwrap();
    let listener = node.create_subscription::<String>(&topic, Some(qos_profile));

    let poll = Poll::new().unwrap();

    let mut events = Events::with_capacity(8);
    let listener = listener.unwrap();

    loop {
        poll.poll(&mut events, None).unwrap();

        for event in events.iter() {
            match event.token() {
                Token(1) => match listener.take() {
                    Ok(Some((message, _message_info))) => {
                        let l = message.len();
                        println!("message len={} : {:?}", l, &message[..min(l, 50)]);
                    }
                    Ok(None) => println!("No message?!"),
                    Err(e) => {
                        println!(">>> error with response handling, e: {:?}", e)
                    }
                },
                _ => println!(">>> Unknown poll token {:?}", event.token()),
            } // match
        } // for
    } // loop
}

fn create_qos() -> QosPolicies {
    let service_qos: QosPolicies = {
        QosPolicyBuilder::new()
            .history(policy::History::KeepLast { depth: 10 })
            .reliability(policy::Reliability::Reliable {
                max_blocking_time: Duration::from_millis(100),
            })
            .durability(policy::Durability::Volatile)
            .deadline(Deadline(Duration::INFINITE))
            .lifespan(Lifespan {
                duration: Duration::INFINITE,
            })
            .liveliness(policy::Liveliness::Automatic {
                lease_duration: Duration::INFINITE,
            })
            .build()
    };
    service_qos
}

fn create_node() -> Node {
    let context = Context::new().unwrap();
    context
        .new_node(
            NodeName::new("/rustdds", "rustdds_listener").unwrap(),
            NodeOptions::new().enable_rosout(true),
        )
        .unwrap()
}
