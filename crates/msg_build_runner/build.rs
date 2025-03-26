

// build script to generate msgs as source

use std::io::Result;
use ros_msg::source_generator::generate_sources;

fn main() -> Result<()> {

    // sources should refer to a directory where there are msg definitions, output is where you would like the resultant library saved
    // change the sources path to one of the msg directories in a ros msg set (e.g https://github.com/ros/std_msgs) and a file with the rust structs for it should pop out
    generate_sources(vec!["../../crates/msg_build_runner/definition_target/input"], "../../crates/msg_build_runner/definition_target/output");
    
    Ok(())
}