pub mod home {
    use crate::devices::Device;

    pub enum SmartHomeError {
        RoomNameIsNotUnique,
        RoomNotFound,
        DeviceIsNotUnique,
        DeviceNotFound,
    }

    pub struct Home {
        id: usize,
        name: String,
        rooms: Vec<Room>,
    }

    pub struct Room {
        id: usize,
        name: String,
        devices: Vec<Box<dyn Device>>,
    }

    impl Home {
        pub fn id(&self) -> usize {
            self.id
        }
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

        pub fn find_room(&self, _id: usize) -> Option<Room> {
            todo!()
        }

        pub fn delete_room(&mut self, _id: usize) -> Result<Room, SmartHomeError> {
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
        pub fn id(&self) -> usize {
            self.id
        }
        pub fn name(&self) -> &str {
            &self.name
        }
        pub fn devices(&self) -> &Vec<Box<dyn Device>> {
            &self.devices
        }

        pub fn add_device(
            &mut self,
            _device: Box<dyn Device>,
        ) -> Result<Box<dyn Device>, SmartHomeError> {
            if !self.is_unique_device(_device) {
                return Err(SmartHomeError::DeviceIsNotUnique);
            }
            todo!()
        }

        pub fn find_device(&self, _id: usize) -> Option<Box<dyn Device>> {
            todo!()
        }

        pub fn delete_device(&mut self, _id: usize) -> Result<Room, SmartHomeError> {
            todo!()
        }

        fn is_unique_device(&self, _device: Box<dyn Device>) -> bool {
            todo!()
        }
    }
}

pub mod devices {
    pub trait Device {
        fn meta(&self) -> &Meta;
    }

    pub struct Meta {
        id: usize,
        name: String,
        description: String,
    }

    pub struct SmartSocket {
        power: f64,
        enabled: bool,
        meta: Meta,
    }

    pub struct Thermometer {
        temperature: f64,
        meta: Meta,
    }

    impl Device for SmartSocket {
        fn meta(&self) -> &Meta {
            &self.meta
        }
    }
    impl Device for Thermometer {
        fn meta(&self) -> &Meta {
            &self.meta
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

    impl Meta {
        pub fn id(&self) -> usize {
            self.id
        }

        pub fn name(&self) -> &str {
            &self.name
        }

        pub fn description(&self) -> &str {
            &self.description
        }
    }

    impl Thermometer {
        pub fn temperature(&self) -> f64 {
            self.temperature
        }
    }
}
