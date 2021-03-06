extern crate futures;
extern crate tokio_core;
extern crate tokio_modbus;
extern crate tokio_serial;

use tokio_core::reactor::Core;
use futures::future::Future;
use tokio_serial::{Serial, SerialPortSettings};
use tokio_modbus::*;

pub fn main() {
    let mut core = Core::new().unwrap();
    let handle = core.handle();
    let port_path = "COM1";
    let server_addr = 0x01;

    let mut settings = SerialPortSettings::default();
    settings.baud_rate = 19200;
    let port = Serial::from_path(port_path, &settings).unwrap(); // v3.1

    let task = Client::connect_rtu(port, server_addr, &handle).and_then(|client| {
        println!("Reading a sensor value");
        client
            .read_holding_registers(0x082B, 2)
            .and_then(move |res| {
                println!("Sensor value is: {:?}", res);
                Ok(())
            })
    });

    core.run(task).unwrap();
}
