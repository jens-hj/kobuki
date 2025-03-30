use ros_msg::source_generator::build_msg_libs;


mod msg_defs;

fn main() {
    build_msg_libs("C:/git/ros/common_msgs", "C:/git/kobuki/crates/msg_build_runner/src/msg_defs");
}