/**
 * 
 *  RustTypes for ROS 2 message sets
 * 
 */
pub mod source_generator;


pub mod basic_data_types {
    use serde::{Deserialize, Serialize};


    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Vector3 {
        x: f64,
        y: f64,
        z: f64,
    }
}


pub mod geometry_msgs  {
    pub mod msg {
        use serde::{Deserialize, Serialize};
        use crate::basic_data_types;


        #[derive(Debug, Clone, Serialize, Deserialize)]
        pub struct Twist {
            linear: basic_data_types::Vector3,
            angular: basic_data_types::Vector3,
        }
    }
    
}
