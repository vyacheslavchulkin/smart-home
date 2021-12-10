pub mod home {
    use crate::devices::Device;

    pub enum SmartHomeError {
        RoomNameIsNotUnique,
        RoomNotFound,
        DeviceIsNotUnique,
        DeviceNotFound,
    }

    pub struct Home {
        name: String,
        rooms: Vec<Room>,
    }

    pub struct Room {
        name: String,
        devices: Vec<Device>,
    }

    impl Home {
        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn rooms(&self) -> &Vec<Room> {
            &self.rooms
        }

        pub fn add_room(&mut self, room: Room) -> Result<Room, SmartHomeError> {
            if !self.is_unique_room(&room.name) {
                return Err(SmartHomeError::RoomNameIsNotUnique);
            }
            todo!()
        }

        pub fn find_room(&self, _name: &str) -> Option<Room> {
            todo!()
        }

        pub fn delete_room(&mut self, _name: &str) -> Result<Room, SmartHomeError> {
            todo!()
        }

        pub fn generate_report(&self) -> Result<String, SmartHomeError> {
            todo!()
        }

        fn is_unique_room(&self, _room_name: &str) -> bool {
            todo!()
        }
    }

    impl Room {
        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn devices(&self) -> &Vec<Device> {
            &self.devices
        }

        pub fn add_device(&mut self, _device: Device) -> Result<Device, SmartHomeError> {
            if !self.is_unique_device(_device) {
                return Err(SmartHomeError::DeviceIsNotUnique);
            }
            todo!()
        }

        pub fn find_device(&self, _name: &str) -> Option<Device> {
            todo!()
        }

        pub fn delete_device(&mut self, _name: &str) -> Result<Room, SmartHomeError> {
            todo!()
        }

        fn is_unique_device(&self, _device: Device) -> bool {
            todo!()
        }
    }
}

pub mod devices {
    pub struct Device {
        name: String,
        description: String,
        device_type: DeviceType,
    }

    pub enum DeviceType {
        SmartSocket(SmartSocket),
        Thermometer(Thermometer),
    }

    pub struct SmartSocket {
        power: f64,
        enabled: bool,
    }

    pub struct Thermometer {
        temperature: f64,
    }

    impl Device {
        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn description(&self) -> &str {
            &self.description
        }

        pub fn device_type(&self) -> &DeviceType {
            &self.device_type
        }
    }

    impl SmartSocket {
        pub fn power(&self) -> f64 {
            self.power
        }

        pub fn is_enabled(&self) -> bool {
            self.enabled
        }

        pub fn enable(&mut self) {
            self.enabled = true
        }

        pub fn disable(&mut self) {
            self.enabled = false
        }
    }

    impl Thermometer {
        pub fn temperature(&self) -> f64 {
            self.temperature
        }
    }
}
