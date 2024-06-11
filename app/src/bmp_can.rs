use crate::can::{CanError, CanReceiver};
use async_channel::Sender;
use socketcan::{EmbeddedFrame, Frame};

const TEMP_FRAME_ID: u32 = 0x317;
const PRES_FRAME_ID: u32 = 0x318;
const ALT_FRAME_ID: u32 = 0x319;

#[derive(Clone, Default)]
pub struct SensorData {
    pub temperature: f32,
    pub pressure: i32,
    pub altitude: f32,
}

pub struct BmpCan {
    sender: Sender<SensorData>,
    can: CanReceiver,
    sensor_data: SensorData,
}

impl BmpCan {
    fn update_main(&self) {
        self.sender
            .send_blocking(self.sensor_data.clone())
            .expect("Data channel must be open.");
    }

    pub fn new(sender: Sender<SensorData>) -> Result<Self, CanError> {
        let can = CanReceiver::new("can0")?;
        Ok(BmpCan {
            sender,
            can,
            sensor_data: SensorData::default(),
        })
    }

    pub fn run<F>(&mut self, should_keep_running: F)
    where
        F: Fn() -> bool,
    {
        while should_keep_running() {
            let frame = self.can.receive_blocking();
            println!("{}", frame_to_string(&frame));

            match frame.raw_id() {
                TEMP_FRAME_ID => {
                    self.sensor_data.temperature =
                        postcard::from_bytes::<f32>(frame.data()).unwrap()
                }
                PRES_FRAME_ID => {
                    self.sensor_data.pressure = postcard::from_bytes::<i32>(frame.data()).unwrap()
                }
                ALT_FRAME_ID => {
                    self.sensor_data.altitude = postcard::from_bytes::<f32>(frame.data()).unwrap()
                }
                unmatched_id => {
                    println!("bmp_can: Unexpected CAN frame identifier: {unmatched_id}")
                }
            }

            self.update_main();
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
