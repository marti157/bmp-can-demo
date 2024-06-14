#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(impl_trait_in_assoc_type)]

use bmp085_180_rs::BMP;
use ch32_can_rs::{hal, nb, Can, CanFifo, CanFilter, CanFrame, CanMode, StandardId};
use embassy_executor::Spawner;
use embassy_sync::{blocking_mutex::raw::CriticalSectionRawMutex, signal::Signal};
use embassy_time::{Delay, Timer};
use hal::{dma::NoDma, i2c::I2c, peripherals, println, time::Hertz};
use qingke::riscv;
use serde::Serialize;

const TEMP_FRAME_ID: u16 = 0x317;
const PRES_FRAME_ID: u16 = 0x318;
const ALT_FRAME_ID: u16 = 0x319;

#[derive(Serialize, Debug)]
struct SensorData {
    temperature: f32,
    pressure: i32,
    altitude: f32,
}

static SHARED: Signal<CriticalSectionRawMutex, SensorData> = Signal::new();

#[embassy_executor::task]
async fn read_bmp_data(mut bmp180: BMP<I2c<'static, peripherals::I2C2, NoDma>, Delay>) {
    loop {
        let temp = bmp180.read_temperature().unwrap();
        println!("Temperature: {} ºC", temp);

        let pres = bmp180.read_pressure().unwrap();
        println!("Pressure: {} Pa", pres);

        let alt = bmp180.read_altitude().unwrap();
        println!("Altitude: {} m", alt);

        SHARED.signal(SensorData {
            temperature: temp,
            pressure: pres,
            altitude: alt,
        });

        Timer::after_secs(5).await;
    }
}

fn send_sensor_data_via_can(can: &Can<peripherals::CAN1>, sensor_data: &SensorData) {
    let serialized_temp: heapless::Vec<u8, 8> = postcard::to_vec(&sensor_data.temperature).unwrap();
    let serialized_pres: heapless::Vec<u8, 8> = postcard::to_vec(&sensor_data.pressure).unwrap();
    let serialized_alt: heapless::Vec<u8, 8> = postcard::to_vec(&sensor_data.altitude).unwrap();
    let frames = [
        CanFrame::new(StandardId::new(TEMP_FRAME_ID).unwrap(), &serialized_temp).unwrap(),
        CanFrame::new(StandardId::new(PRES_FRAME_ID).unwrap(), &serialized_pres).unwrap(),
        CanFrame::new(StandardId::new(ALT_FRAME_ID).unwrap(), &serialized_alt).unwrap(),
    ];

    for frame in &frames {
        match can.transmit(frame) {
            Ok(_) => println!("Sent CAN frame: {:?}", frame),
            Err(nb::Error::WouldBlock) => {
                println!("Error sending CAN frame, mailboxes are full")
            }
            Err(nb::Error::Other(error)) => println!("Error sending CAN frame: {error}"),
        }
    }
}

#[embassy_executor::main(entry = "qingke_rt::entry")]
async fn main(spawner: Spawner) {
    hal::debug::SDIPrint::enable();
    let mut config = hal::Config::default();
    config.rcc = hal::rcc::Config::SYSCLK_FREQ_96MHZ_HSI;
    let p = hal::init(config);
    hal::embassy::init();

    // If we don't wait, embassy time doesn't set the alarm properly and eventually panics
    // due to multiplication overflow
    riscv::asm::delay(1000000);

    println!("Embassy initialized");

    let scl = p.PB10;
    let sda = p.PB11;
    let i2c = I2c::new(
        p.I2C2,
        scl,
        sda,
        NoDma,
        NoDma,
        Hertz::khz(100),
        Default::default(),
    );
    let mut bmp180 = BMP::new(i2c, Delay, Default::default());

    match bmp180.test_connection() {
        Ok(_) => println!("BMP connected"),
        Err(_) => {
            println!("BMP not found");
            panic!();
        }
    }

    let can = Can::new(
        p.CAN1,
        p.PB8,
        p.PB9,
        CanFifo::Fifo1,
        CanMode::Normal,
        500_000,
    );
    can.add_filter(CanFilter::accept_all());
    println!("CAN init");

    bmp180.init().unwrap();
    println!("BMP init");

    spawner.spawn(read_bmp_data(bmp180)).unwrap();

    loop {
        let sensor_data = SHARED.wait().await;
        println!("Preparing to send data...");
        send_sensor_data_via_can(&can, &sensor_data);
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    println!("\nPanic: {info}");

    loop {}
}
