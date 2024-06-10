use crate::can::{CanError, CanReceiver};
use async_channel::Sender;
use socketcan::{EmbeddedFrame, Frame};

const TEMP_FRAME_ID: u32 = 0x317;
const PRES_FRAME_ID: u32 = 0x318;
const ALT_FRAME_ID: u32 = 0x319;

#[derive(Default)]
pub struct SensorData {
    temperature: f32,
    pressure: i32,
    altitude: f32,
}

pub struct BmpCan {
    sender: Sender<String>,
    can: CanReceiver,
    sensor_data: SensorData,
}

impl BmpCan {
    pub fn new(sender: Sender<String>) -> Result<Self, CanError> {
        match CanReceiver::new("can0") {
            Ok(can) => Ok(BmpCan {
                sender,
                can,
                sensor_data: SensorData::default(),
            }),
            Err(error) => {
                sender
                    .send_blocking(format!("{error}"))
                    .expect("The channel needs to be open.");
                Err(error)
            }
        }
    }

    pub fn run<F>(&mut self, should_keep_running: F)
    where
        F: Fn() -> bool,
    {
        while should_keep_running() {
            let frame = self.can.receive_blocking();
            println!("{}", frame_to_string(&frame));

            if frame.raw_id() == TEMP_FRAME_ID {
                self.sensor_data.temperature = postcard::from_bytes::<f32>(frame.data()).unwrap();
                self.sender
                    .send_blocking("got value".to_string())
                    .expect("The channel needs to be open.");
            }
        }
    }
}

fn frame_to_string<F: Frame>(frame: &F) -> String {
    let id = frame.raw_id();
    let data_string = frame
        .data()
        .iter()
        .fold(String::from(""), |a, b| format!("{} {:02x}", a, b));

    format!("{:X}  [{}] {}", id, frame.dlc(), data_string)
}
