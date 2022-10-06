use std::collections::HashMap;
use std::net::UdpSocket;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};

use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;

#[derive(Copy, Clone)]
#[pyclass]
pub struct State {
    pub roll: i16,
    pub pitch: i16,
    pub yaw: i16,
    pub ground_velocity_x: i16,
    pub ground_velocity_y: i16,
    pub ground_velocity_z: i16,
    pub temperature_minimum: u8,
    pub temperature_maximum: u8,
    pub tof_value: i16,
    pub height: i16,
    pub battery_percentage: u8,
    pub barometer_height: f32,
    pub time: u16,
    pub ground_acceleration_x: f32,
    pub ground_acceleration_y: f32,
    pub ground_acceleration_z: f32,
}

#[pymethods]
impl State {
    #[new]
    fn new() -> Self {
        State {
            roll: 0,
            pitch: 0,
            yaw: 0,
            ground_velocity_x: 0,
            ground_velocity_y: 0,
            ground_velocity_z: 0,
            temperature_minimum: 0,
            temperature_maximum: 0,
            tof_value: 0,
            height: 0,
            battery_percentage: 0,
            barometer_height: 0.0,
            time: 0,
            ground_acceleration_x: 0.0,
            ground_acceleration_y: 0.0,
            ground_acceleration_z: 0.0,
        }
    }
    #[getter(roll)]
    pub fn get_roll(&self)->i16 {
        self.roll
    }
    #[getter(pitch)]
    pub fn get_pitch(&self)->i16 {
        self.pitch
    }
    #[getter(yaw)]
    pub fn get_yaw(&self)->i16 {
        self.yaw
    }
    #[getter(ground_velocity_x)]
    pub fn get_velocity_x(&self)->i16 {
        self.ground_velocity_x
    }
    #[getter(ground_velocity_y)]
    pub fn get_velocity_y(&self)->i16 {
        self.ground_velocity_y
    }
    #[getter(ground_velocity_z)]
    pub fn get_velocity_z(&self)->i16 {
        self.ground_velocity_z
    }
    #[getter(temperature_minimum)]
    pub fn get_teperature_min(&self)->u8 {
        self.temperature_minimum
    }
    #[getter(temperature_maximum)]
    pub fn get_teperature_max(&self)->u8 {
        self.temperature_maximum
    }
    #[getter(tof_distance)]
    pub fn get_teperature_mix(&self)->u8 {
        self.temperature_minimum
    }
    #[getter(height)]
    pub fn get_height(&self)->i16 {
        self.height
    }
    #[getter(battery_percentage)]
    pub fn get_battery(&self)->u8 {
        self.battery_percentage
    }
    #[getter(barometer_height)]
    pub fn get_barometer_height(&self)->f32 {
        self.barometer_height
    }
    #[getter(time)]
    pub fn get_time(&self)->u16 {
        self.time
    }
    #[getter(ground_accleration_x)]
    pub fn get_accel_x(&self)->f32 {
        self.ground_acceleration_x
    }
    #[getter(ground_accleration_y)]
    pub fn get_accel_y(&self)->f32 {
        self.ground_acceleration_y
    }
    #[getter(ground_accleration_z)]
    pub fn get_accel_z(&self)->f32 {
        self.ground_acceleration_z
    }
}

/// A Class representing a Tello Drone
#[pyclass]
pub struct Tello {
    command_socket: Arc<Mutex<UdpSocket>>,
    state_socket: Arc<Mutex<UdpSocket>>,
    command_thread: Option<JoinHandle<()>>,
    state_thread: Option<JoinHandle<()>>,
    running: Arc<AtomicBool>,
    state: Arc<Mutex<State>>,
    drone_acked: Arc<AtomicBool>,
}
#[pyclass]
#[derive(Clone,Copy)]
pub enum Flip {
    Left,
    Right,
    Forward,
    Backward,
}


#[derive(Debug)]
pub enum TelloError {
    IO(std::io::Error),
    AckNotReceived
}

impl From<TelloError> for PyErr {
    fn from(err: TelloError) -> PyErr {
        PyRuntimeError::new_err(format!("{:?}",err))
    }
}

impl Flip {
    fn value(self) -> char {
        match self {
            Flip::Left => 'l',
            Flip::Right => 'r',
            Flip::Forward => 'f',
            Flip::Backward => 'b',
        }
    }
}

#[pymethods]
impl Tello {
    /// create a new drone class, this will automatically create the UDP sockets
    /// errors if the sockets cannot be unblocked
    #[new]
    pub fn new() -> Result<Self,std::io::Error> {
        let command_socket = UdpSocket::bind("0.0.0.0:9009")?;
        let state_socket = UdpSocket::bind("0.0.0.0:8890")?;

        command_socket.set_nonblocking(true)?;
        state_socket.set_nonblocking(true)?;

        Ok(Tello {
            command_socket: Arc::new(Mutex::new(command_socket)),
            state_socket: Arc::new(Mutex::new(state_socket)),
            command_thread: None,
            state_thread: None,
            running: Arc::new(AtomicBool::new(false)),
            state: Arc::new(Mutex::new(State::new())),
            drone_acked: Arc::new(AtomicBool::new(false)),
        })
    }

    pub fn connect(&mut self) -> Result<usize,TelloError> {
        //! connects to the drone
        //! also setups the async threads for allowing values to change
        //! errors if the drone does not respond in a set ammount of time
        //! or some other std::io::Error (rust)
        self.running.store(true, Ordering::SeqCst);

        let command_running = self.running.clone();
        let command_socket = self.command_socket.clone();
        let command_acked = self.drone_acked.clone();
        self.command_thread = Some(std::thread::spawn(move || {
            while command_running.load(Ordering::SeqCst) {
                let mut buffer: [u8; 1500] = [0; 1500];
                {
                    let socket = command_socket.lock().unwrap();
                    let result = socket.recv_from(&mut buffer);
                    if let Ok((size,_)) = result {
                        let response = std::str::from_utf8(&buffer[..size])
                            .unwrap_or_default()
                            .trim();
                        if response == "ok" {
                            command_acked.store(true, Ordering::SeqCst);
                        }
                    }
                }

                std::thread::sleep(Duration::from_millis(20));
            }
        }));

        let state_running = self.running.clone();
        let state_socket = self.state_socket.clone();
        let state = self.state.clone();
        self.state_thread = Some(std::thread::spawn(move || {
            while state_running.load(Ordering::SeqCst) {
                let mut buffer: [u8; 1500] = [0; 1500];
                {
                    let socket = state_socket.lock().unwrap();

                    if let Ok(size) = socket.recv(&mut buffer) {
                        let string = std::str::from_utf8(&buffer[..size - 1])
                            .unwrap_or_default()
                            .trim();
                        let parts: Vec<&str> = string.split(';').collect();
                        let mut parameter_map: HashMap<&str, &str> = HashMap::new();
                        for parameter in &parts {
                            if parameter.len() <= 1 || !parameter.contains(':') {
                                continue;
                            }
                            let parameter_parts: Vec<&str> = (*parameter).split(':').collect();
                            parameter_map.insert(parameter_parts[0], parameter_parts[1]);
                        }
                        let mut state = state.lock().unwrap();
                        state.roll = parameter_map["roll"].parse().unwrap();
                        state.pitch = parameter_map["pitch"].parse().unwrap();
                        state.yaw = parameter_map["yaw"].parse().unwrap();
                        state.ground_velocity_x = parameter_map["vgx"].parse().unwrap();
                        state.ground_acceleration_y = parameter_map["vgy"].parse().unwrap();
                        state.ground_velocity_z = parameter_map["vgz"].parse().unwrap();
                        state.temperature_minimum = parameter_map["templ"].parse().unwrap();
                        state.temperature_maximum = parameter_map["temph"].parse().unwrap();
                        state.tof_value = parameter_map["tof"].parse().unwrap();
                        state.height = parameter_map["h"].parse().unwrap();
                        state.battery_percentage = parameter_map["bat"].parse().unwrap();
                        state.barometer_height = parameter_map["baro"].parse().unwrap();
                        state.time = parameter_map["time"].parse().unwrap();
                        state.ground_acceleration_x = parameter_map["agx"].parse().unwrap();
                        state.ground_acceleration_y = parameter_map["agy"].parse().unwrap();
                        state.ground_acceleration_z = parameter_map["agz"].parse().unwrap();
                    }
                }
                std::thread::sleep(Duration::from_millis(50));
            }
        }));

        self.send_command("command", true)
    }

    pub fn disconnect(&mut self) {
        //! disconnect from the drone
        self.running.store(true, Ordering::SeqCst);
        self.state_thread = None;
        self.command_thread = None;
    }

    pub fn send_command(&self, command: &str, acked: bool) -> Result<usize, TelloError> {
        //! send a direct command, blocks while waiting for response
        //! DO NOT USE IT UNLESS TOLD TO
        if command.contains("wifi") {
            return Ok(0)
        }
        {
            let mutex = self.command_socket.clone();
            let socket = mutex.lock().unwrap();
            #[allow(clippy::redundant_closure)]
            socket.send_to(command.as_bytes(), "192.168.10.1:8889").map_err(|err| TelloError::IO(err))?;
        }

        if !acked {
            return Ok(command.len());
        }

        let sending_acked = self.drone_acked.clone();
        let now = Instant::now();
        while now.elapsed() < Duration::from_millis(10000) {
            if sending_acked.load(Ordering::SeqCst) {
                sending_acked.store(false, Ordering::SeqCst);
                return Ok(command.len());
            }

            std::thread::sleep(Duration::from_millis(50));
        }
        Err(TelloError::AckNotReceived)
    }

    #[getter(state)]
    pub fn get_state(&self) -> State {
        //! get the state of the drone and return it
        return *self.state.lock().unwrap();
    }

    #[getter(running)]
    pub fn get_running(&self) -> bool {
        //! get whether or not the drone is running
        self.running.load(Ordering::Relaxed)
    }

    pub fn take_off(&self) -> Result<usize, TelloError> {
        //! perform the automatic takeoff
        self.send_command("takeoff", false)
    }

    pub fn land(&self) -> Result<usize, TelloError> {
        //! perform the automated landing
        self.send_command("land", false)
    }

    pub fn stream_on(&self) -> Result<usize, TelloError> {
        //! start capturing data from the camera
        self.send_command("streamon", true)
    }

    pub fn stream_off(&self) -> Result<usize, TelloError> {
        //! stop capturing data from the camera
        self.send_command("streamoff", true)
    }

    pub fn emergency(&self) -> Result<usize, TelloError> {
        //! immedietly turn off all the propellers
        self.send_command("emergency", true)
    }

    pub fn up(&self, distance_cm: u16) -> Result<usize, TelloError> {
        //! move the drone up the specified distance in cm
        self.send_command(format!("up {}", distance_cm).as_str(), true)
    }

    pub fn down(&self, distance_cm: u16) -> Result<usize, TelloError> {
        //! move the drone down the specified distance in cm
        self.send_command(format!("down {}", distance_cm).as_str(), true)
    }

    pub fn left(&self, distance_cm: u16) -> Result<usize, TelloError> {
        //! move the drone left the specified distance in cm
        self.send_command(format!("left {}", distance_cm).as_str(), true)
    }

    pub fn right(&self, distance_cm: u16) -> Result<usize, TelloError> {
        //! move the drone right the specified distance in cm
        self.send_command(format!("right {}", distance_cm).as_str(), true)
    }

    pub fn forward(&self, distance_cm: u16) -> Result<usize, TelloError> {
        //! move the drone forward the specified distance in cm
        self.send_command(format!("forward {}", distance_cm).as_str(), true)
    }

    pub fn back(&self, distance_cm: u16) -> Result<usize, TelloError> {
        //! move the back down the specified distance in cm
        self.send_command(format!("back {}", distance_cm).as_str(), true)
    }

    pub fn cw(&self, angle_millidegs: u16) -> Result<usize, TelloError> {
        //! rotate the drone a specified milidegres(0.001 degrees) clockwise
        self.send_command(format!("cw {}", angle_millidegs).as_str(), true)
    }

    pub fn ccw(&self, angle_millidegs: u16) -> Result<usize, TelloError> {
        //! rotate the drone a specified milidegres(0.001 degrees) counter-clockwise
        self.send_command(format!("ccw {}", angle_millidegs).as_str(), true)
    }

    pub fn flip(&self, flip: Flip) -> Result<usize, TelloError> {
        self.send_command(format!("flip {}", flip.value()).as_str(), true)
    }

    pub fn go(&self, x_cm: u16, y_cm: u16, z_cm: u16, speed: u8) -> Result<usize, TelloError> {
        //! go to a relative position to the drone, limited to a 1000x1000 cm cube centered on the drone
        //! also includes a speed
        self.send_command(format!("go {} {} {} {}", x_cm, y_cm, z_cm, speed).as_str(), true)
    }

    #[allow(clippy::too_many_arguments)]
    pub fn curve(
        &self,
        x1_cm: u16,
        y1_cm: u16,
        z1_cm: u16,
        x2_cm: u16,
        y2_cm: u16,
        z2_cm: u16,
        speed: u8,
    ) -> Result<usize, TelloError> {
        //! (i had to silence format warnings)
        //! curves around from point 1 to point 2
        self.send_command(
            format!(
                "curve {} {} {} {} {} {} {}",
                x1_cm, y1_cm, z1_cm, x2_cm, y2_cm, z2_cm, speed
            )
            .as_str(), true
        )
    }

    pub fn rc(
        &self,
        left_right: i8,
        forward_backward: i8,
        up_down: i8,
        yaw: i8,
    ) -> Result<usize, TelloError> {
        //! simulated remote controll inputs
        self.send_command(
            format!("rc {} {} {} {}", left_right, forward_backward, up_down, yaw).as_str(), false
        )
    }

    pub fn speed(&self, speed_cms: u8) -> Result<usize, TelloError> {
        //! sets the speed for many other commands
        self.send_command(format!("speed {}", speed_cms).as_str(), true)
    }

    /*
    pub fn wifi(&self, ssid: &str, password: &str) -> Result<usize, TelloError> {
        //! changes the wifi name/password, if you change the password we will kill you
        self.send_command(format!("wifi {} {}", ssid, password).as_str(), true)
    }
    */
}
