use std::{rc::Rc, cell::Cell, cell::RefCell};
use std::fs::File;
pub struct SpiderRobot {
    species: String,
    web_enabled: bool,
    //leg_devices: [fd::FileDesc;8],
    hardware_error_count: Cell<u32>,
    log_file: RefCell<File>
}

pub struct SpiderSenses {
    robot: Rc<SpiderRobot>,
    // eyes: [Camera; 32]
}

impl SpiderRobot {
    pub fn new() -> SpiderRobot {
        let f = File::open("").unwrap();

        SpiderRobot {
            species: "Tarantula".to_string(),
            web_enabled: true,
            hardware_error_count: Cell::new(0),
            log_file: RefCell::new(f)
        }
    }

    pub fn add_hardware_error(&self) {
        let n = self.hardware_error_count.get();
        self.hardware_error_count.set(n + 1);
    }

    pub fn has_hardware_errors(&self)-> bool {
        self.hardware_error_count.get() > 0
    }
}