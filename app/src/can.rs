use socketcan::embedded_can::blocking::Can;
pub use socketcan::Error as CanError;
use socketcan::{CanFrame, CanSocket, Socket};

pub struct CanReceiver {
    can: CanSocket,
}

impl CanReceiver {
    pub fn new(ifname: &str) -> Result<Self, CanError> {
        let socket = CanSocket::open(ifname)?;

        Ok(CanReceiver { can: socket })
    }

    pub fn receive_blocking(&mut self) -> CanFrame {
        self.can.receive().expect("Received error frame!")
    }
}
