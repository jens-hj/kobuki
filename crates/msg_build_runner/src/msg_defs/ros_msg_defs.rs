pub mod common_msgs {
pub mod actionlib_msgs {
pub mod msg {
    pub struct GoalID {
        stamp: usize,
        id: String,
    }
    pub struct GoalStatus {
        goal_id: GoalID,
        status: u8,
        PENDING: u8,
        ACTIVE: u8,
        PREEMPTED: u8,
        SUCCEEDED: u8,
        ABORTED: u8,
        REJECTED: u8,
        PREEMPTING: u8,
        RECALLING: u8,
        RECALLED: u8,
        LOST: u8,
        text: String,
    }
    pub struct GoalStatusArray {
        header: usize,
        status_list: Vec<GoalStatus>,
    }
}
}
pub mod diagnostic_msgs {
pub mod msg {
    pub struct DiagnosticArray {
        header: usize,
        status: Vec<DiagnosticStatus>,
    }
    pub struct DiagnosticStatus {
        OK: u8,
        WARN: u8,
        ERROR: u8,
        STALE: u8,
        level: u8,
        name: String,
        message: String,
        hardware_id: String,
        values: Vec<KeyValue>,
    }
    pub struct KeyValue {
        key: String,
        value: String,
    }
}
}
pub mod geometry_msgs {
pub mod msg {
    pub struct Accel {
        linear: Vector3,
        angular: Vector3,
    }
    pub struct AccelStamped {
        header: usize,
        accel: Accel,
    }
    pub struct AccelWithCovariance {
        accel: Accel,
        covariance: Vec<f64>,
    }
    pub struct AccelWithCovarianceStamped {
        header: usize,
        accel: AccelWithCovariance,
    }
    pub struct Inertia {
        m: f64,
        com: geometry_msgs::Vector3,
        ixx: f64,
        ixy: f64,
        ixz: f64,
        iyy: f64,
        iyz: f64,
        izz: f64,
    }
    pub struct InertiaStamped {
        header: usize,
        inertia: Inertia,
    }
    pub struct Point {
        x: f64,
        y: f64,
        z: f64,
    }
    pub struct Point32 {
        x: f32,
        y: f32,
        z: f32,
    }
    pub struct PointStamped {
        header: usize,
        point: Point,
    }
    pub struct Polygon {
        points: Vec<Point32>,
    }
    pub struct PolygonStamped {
        header: usize,
        polygon: Polygon,
    }
    pub struct Pose {
        position: Point,
        orientation: Quaternion,
    }
    pub struct Pose2D {
        x: f64,
        y: f64,
        theta: f64,
    }
    pub struct PoseArray {
        header: usize,
        poses: Vec<Pose>,
    }
    pub struct PoseStamped {
        header: usize,
        pose: Pose,
    }
    pub struct PoseWithCovariance {
        pose: Pose,
        covariance: Vec<f64>,
    }
    pub struct PoseWithCovarianceStamped {
        header: usize,
        pose: PoseWithCovariance,
    }
    pub struct Quaternion {
        x: f64,
        y: f64,
        z: f64,
        w: f64,
    }
    pub struct QuaternionStamped {
        header: usize,
        quaternion: Quaternion,
    }
    pub struct Transform {
        translation: Vector3,
        rotation: Quaternion,
    }
    pub struct TransformStamped {
        header: usize,
        child_frame_id: String,
        transform: Transform,
    }
    pub struct Twist {
        linear: Vector3,
        angular: Vector3,
    }
    pub struct TwistStamped {
        header: usize,
        twist: Twist,
    }
    pub struct TwistWithCovariance {
        twist: Twist,
        covariance: Vec<f64>,
    }
    pub struct TwistWithCovarianceStamped {
        header: usize,
        twist: TwistWithCovariance,
    }
    pub struct Vector3 {
        x: f64,
        y: f64,
        z: f64,
    }
    pub struct Vector3Stamped {
        header: usize,
        vector: Vector3,
    }
    pub struct Wrench {
        force: Vector3,
        torque: Vector3,
    }
    pub struct WrenchStamped {
        header: usize,
        wrench: Wrench,
    }
}
}
pub mod nav_msgs {
pub mod msg {
    pub struct GridCells {
        header: usize,
        cell_width: f32,
        cell_height: f32,
        cells: Vec<geometry_msgs::Point>,
    }
    pub struct MapMetaData {
        map_load_time: usize,
        resolution: f32,
        width: u32,
        height: u32,
        origin: geometry_msgs::Pose,
    }
    pub struct OccupancyGrid {
        header: usize,
        info: MapMetaData,
        data: Vec<i8>,
    }
    pub struct Odometry {
        header: usize,
        child_frame_id: String,
        pose: geometry_msgs::PoseWithCovariance,
        twist: geometry_msgs::TwistWithCovariance,
    }
    pub struct Path {
        header: usize,
        poses: Vec<geometry_msgs::PoseStamped>,
    }
}
}
pub mod sensor_msgs {
pub mod msg {
    pub struct BatteryState {
        POWER_SUPPLY_STATUS_UNKNOWN: u8,
        POWER_SUPPLY_STATUS_CHARGING: u8,
        POWER_SUPPLY_STATUS_DISCHARGING: u8,
        POWER_SUPPLY_STATUS_NOT_CHARGING: u8,
        POWER_SUPPLY_STATUS_FULL: u8,
        POWER_SUPPLY_HEALTH_UNKNOWN: u8,
        POWER_SUPPLY_HEALTH_GOOD: u8,
        POWER_SUPPLY_HEALTH_OVERHEAT: u8,
        POWER_SUPPLY_HEALTH_DEAD: u8,
        POWER_SUPPLY_HEALTH_OVERVOLTAGE: u8,
        POWER_SUPPLY_HEALTH_UNSPEC_FAILURE: u8,
        POWER_SUPPLY_HEALTH_COLD: u8,
        POWER_SUPPLY_HEALTH_WATCHDOG_TIMER_EXPIRE: u8,
        POWER_SUPPLY_HEALTH_SAFETY_TIMER_EXPIRE: u8,
        POWER_SUPPLY_TECHNOLOGY_UNKNOWN: u8,
        POWER_SUPPLY_TECHNOLOGY_NIMH: u8,
        POWER_SUPPLY_TECHNOLOGY_LION: u8,
        POWER_SUPPLY_TECHNOLOGY_LIPO: u8,
        POWER_SUPPLY_TECHNOLOGY_LIFE: u8,
        POWER_SUPPLY_TECHNOLOGY_NICD: u8,
        POWER_SUPPLY_TECHNOLOGY_LIMN: u8,
        header: usize,
        voltage: f32,
        temperature: f32,
        current: f32,
        charge: f32,
        capacity: f32,
        design_capacity: f32,
        percentage: f32,
        power_supply_status: u8,
        power_supply_health: u8,
        power_supply_technology: u8,
        present: bool,
        cell_voltage: Vec<f32>,
        cell_temperature: Vec<f32>,
        location: String,
        serial_number: String,
    }
    pub struct CameraInfo {
        header: usize,
        height: u32,
        width: u32,
        distortion_model: String,
        D: Vec<f64>,
        K: Vec<f64>,
        R: Vec<f64>,
        P: Vec<f64>,
        binning_x: u32,
        binning_y: u32,
        roi: RegionOfInterest,
    }
    pub struct ChannelFloat32 {
        name: String,
        values: Vec<f32>,
    }
    pub struct CompressedImage {
        header: usize,
        format: String,
        data: Vec<u8>,
    }
    pub struct FluidPressure {
        header: usize,
        fluid_pressure: f64,
        variance: f64,
    }
    pub struct Illuminance {
        header: usize,
        illuminance: f64,
        variance: f64,
    }
    pub struct Image {
        header: usize,
        height: u32,
        width: u32,
        encoding: String,
        is_bigendian: u8,
        step: u32,
        data: Vec<u8>,
    }
    pub struct Imu {
        header: usize,
        orientation: geometry_msgs::Quaternion,
        orientation_covariance: Vec<f64>,
        angular_velocity: geometry_msgs::Vector3,
        angular_velocity_covariance: Vec<f64>,
        linear_acceleration: geometry_msgs::Vector3,
        linear_acceleration_covariance: Vec<f64>,
    }
    pub struct JointState {
        header: usize,
        name: Vec<String>,
        position: Vec<f64>,
        velocity: Vec<f64>,
        effort: Vec<f64>,
    }
    pub struct Joy {
        header: usize,
        axes: Vec<f32>,
        buttons: Vec<i32>,
    }
    pub struct JoyFeedback {
        TYPE_LED: u8,
        TYPE_RUMBLE: u8,
        TYPE_BUZZER: u8,
        _type: u8,
        id: u8,
        intensity: f32,
    }
    pub struct JoyFeedbackArray {
        array: Vec<JoyFeedback>,
    }
    pub struct LaserEcho {
        echoes: Vec<f32>,
    }
    pub struct LaserScan {
        header: usize,
        angle_min: f32,
        angle_max: f32,
        angle_increment: f32,
        time_increment: f32,
        scan_time: f32,
        range_min: f32,
        range_max: f32,
        ranges: Vec<f32>,
        intensities: Vec<f32>,
    }
    pub struct MagneticField {
        header: usize,
        magnetic_field: geometry_msgs::Vector3,
        magnetic_field_covariance: Vec<f64>,
    }
    pub struct MultiDOFJointState {
        header: usize,
        joint_names: Vec<String>,
        transforms: Vec<geometry_msgs::Transform>,
        twist: Vec<geometry_msgs::Twist>,
        wrench: Vec<geometry_msgs::Wrench>,
    }
    pub struct MultiEchoLaserScan {
        header: usize,
        angle_min: f32,
        angle_max: f32,
        angle_increment: f32,
        time_increment: f32,
        scan_time: f32,
        range_min: f32,
        range_max: f32,
        ranges: Vec<LaserEcho>,
        intensities: Vec<LaserEcho>,
    }
    pub struct NavSatFix {
        header: usize,
        status: NavSatStatus,
        latitude: f64,
        longitude: f64,
        altitude: f64,
        position_covariance: Vec<f64>,
        COVARIANCE_TYPE_UNKNOWN: u8,
        COVARIANCE_TYPE_APPROXIMATED: u8,
        COVARIANCE_TYPE_DIAGONAL_KNOWN: u8,
        COVARIANCE_TYPE_KNOWN: u8,
        position_covariance_type: u8,
    }
    pub struct NavSatStatus {
        STATUS_NO_FIX: i8,
        STATUS_FIX: i8,
        STATUS_SBAS_FIX: i8,
        STATUS_GBAS_FIX: i8,
        status: i8,
        SERVICE_GPS: uint16,
        SERVICE_GLONASS: uint16,
        SERVICE_COMPASS: uint16,
        SERVICE_GALILEO: uint16,
        service: uint16,
    }
    pub struct PointCloud {
        header: usize,
        points: Vec<geometry_msgs::Point32>,
        channels: Vec<ChannelFloat32>,
    }
    pub struct PointCloud2 {
        header: usize,
        height: u32,
        width: u32,
        fields: Vec<PointField>,
        is_bigendian: bool,
        point_step: u32,
        row_step: u32,
        data: Vec<u8>,
        is_dense: bool,
    }
    pub struct PointField {
        INT8: u8,
        UINT8: u8,
        INT16: u8,
        UINT16: u8,
        INT32: u8,
        UINT32: u8,
        FLOAT32: u8,
        FLOAT64: u8,
        name: String,
        offset: u32,
        datatype: u8,
        count: u32,
    }
    pub struct Range {
        header: usize,
        ULTRASOUND: u8,
        INFRARED: u8,
        radiation_type: u8,
        field_of_view: f32,
        min_range: f32,
        max_range: f32,
        range: f32,
    }
    pub struct RegionOfInterest {
        x_offset: u32,
        y_offset: u32,
        height: u32,
        width: u32,
        do_rectify: bool,
    }
    pub struct RelativeHumidity {
        header: usize,
        relative_humidity: f64,
        variance: f64,
    }
    pub struct Temperature {
        header: usize,
        temperature: f64,
        variance: f64,
    }
    pub struct TimeReference {
        header: usize,
        time_ref: usize,
        source: String,
    }
}
}
pub mod shape_msgs {
pub mod msg {
    pub struct Mesh {
        triangles: Vec<MeshTriangle>,
        vertices: Vec<geometry_msgs::Point>,
    }
    pub struct MeshTriangle {
        vertex_indices: Vec<u32>,
    }
    pub struct Plane {
        coef: Vec<f64>,
    }
    pub struct SolidPrimitive {
        BOX: u8,
        SPHERE: u8,
        CYLINDER: u8,
        CONE: u8,
        _type: u8,
        dimensions: Vec<f64>,
        BOX_X: u8,
        BOX_Y: u8,
        BOX_Z: u8,
        SPHERE_RADIUS: u8,
        CYLINDER_HEIGHT: u8,
        CYLINDER_RADIUS: u8,
        CONE_HEIGHT: u8,
        CONE_RADIUS: u8,
    }
}
}
pub mod stereo_msgs {
pub mod msg {
    pub struct DisparityImage {
        header: usize,
        image: sensor_msgs::Image,
        f: f32,
        T: f32,
        valid_window: sensor_msgs::RegionOfInterest,
        min_disparity: f32,
        max_disparity: f32,
        delta_d: f32,
    }
}
}
pub mod trajectory_msgs {
pub mod msg {
    pub struct JointTrajectory {
        header: usize,
        joint_names: Vec<String>,
        points: Vec<JointTrajectoryPoint>,
    }
    pub struct JointTrajectoryPoint {
        positions: Vec<f64>,
        velocities: Vec<f64>,
        accelerations: Vec<f64>,
        effort: Vec<f64>,
        time_from_start: duration,
    }
    pub struct MultiDOFJointTrajectory {
        header: usize,
        joint_names: Vec<String>,
        points: Vec<MultiDOFJointTrajectoryPoint>,
    }
    pub struct MultiDOFJointTrajectoryPoint {
        transforms: Vec<geometry_msgs::Transform>,
        velocities: Vec<geometry_msgs::Twist>,
        accelerations: Vec<geometry_msgs::Twist>,
        time_from_start: duration,
    }
}
}
pub mod visualization_msgs {
pub mod msg {
    pub struct ImageMarker {
        CIRCLE: u8,
        LINE_STRIP: u8,
        LINE_LIST: u8,
        POLYGON: u8,
        POINTS: u8,
        ADD: u8,
        REMOVE: u8,
        header: usize,
        ns: String,
        id: i32,
        _type: i32,
        action: i32,
        position: geometry_msgs::Point,
        scale: f32,
        outline_color: std_msgs::ColorRGBA,
        filled: u8,
        fill_color: std_msgs::ColorRGBA,
        lifetime: duration,
        points: Vec<geometry_msgs::Point>,
        outline_colors: Vec<std_msgs::ColorRGBA>,
    }
    pub struct InteractiveMarker {
        header: usize,
        pose: geometry_msgs::Pose,
        name: String,
        description: String,
        scale: f32,
        menu_entries: Vec<MenuEntry>,
        controls: Vec<InteractiveMarkerControl>,
    }
    pub struct InteractiveMarkerControl {
        name: String,
        orientation: geometry_msgs::Quaternion,
        INHERIT: u8,
        FIXED: u8,
        VIEW_FACING: u8,
        orientation_mode: u8,
        NONE: u8,
        MENU: u8,
        BUTTON: u8,
        MOVE_AXIS: u8,
        MOVE_PLANE: u8,
        ROTATE_AXIS: u8,
        MOVE_ROTATE: u8,
        MOVE_3D: u8,
        ROTATE_3D: u8,
        MOVE_ROTATE_3D: u8,
        interaction_mode: u8,
        always_visible: bool,
        markers: Vec<Marker>,
        independent_marker_orientation: bool,
        description: String,
    }
    pub struct InteractiveMarkerFeedback {
        header: usize,
        client_id: String,
        marker_name: String,
        control_name: String,
        KEEP_ALIVE: u8,
        POSE_UPDATE: u8,
        MENU_SELECT: u8,
        BUTTON_CLICK: u8,
        MOUSE_DOWN: u8,
        MOUSE_UP: u8,
        event_type: u8,
        pose: geometry_msgs::Pose,
        menu_entry_id: u32,
        mouse_point: geometry_msgs::Point,
        mouse_point_valid: bool,
    }
    pub struct InteractiveMarkerInit {
        server_id: String,
        seq_num: u64,
        markers: Vec<InteractiveMarker>,
    }
    pub struct InteractiveMarkerPose {
        header: usize,
        pose: geometry_msgs::Pose,
        name: String,
    }
    pub struct InteractiveMarkerUpdate {
        server_id: String,
        seq_num: u64,
        KEEP_ALIVE: u8,
        UPDATE: u8,
        _type: u8,
        markers: Vec<InteractiveMarker>,
        poses: Vec<InteractiveMarkerPose>,
        erases: Vec<String>,
    }
    pub struct Marker {
        ARROW: u8,
        CUBE: u8,
        SPHERE: u8,
        CYLINDER: u8,
        LINE_STRIP: u8,
        LINE_LIST: u8,
        CUBE_LIST: u8,
        SPHERE_LIST: u8,
        POINTS: u8,
        TEXT_VIEW_FACING: u8,
        MESH_RESOURCE: u8,
        TRIANGLE_LIST: u8,
        ADD: u8,
        MODIFY: u8,
        DELETE: u8,
        DELETEALL: u8,
        header: usize,
        ns: String,
        id: i32,
        _type: i32,
        action: i32,
        pose: geometry_msgs::Pose,
        scale: geometry_msgs::Vector3,
        color: std_msgs::ColorRGBA,
        lifetime: duration,
        frame_locked: bool,
        points: Vec<geometry_msgs::Point>,
        colors: Vec<std_msgs::ColorRGBA>,
        text: String,
        mesh_resource: String,
        mesh_use_embedded_materials: bool,
    }
    pub struct MarkerArray {
        markers: Vec<Marker>,
    }
    pub struct MenuEntry {
        id: u32,
        parent_id: u32,
        title: String,
        command: String,
        FEEDBACK: u8,
        ROSRUN: u8,
        ROSLAUNCH: u8,
        command_type: u8,
    }
}
}
}
