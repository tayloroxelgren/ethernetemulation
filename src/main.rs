mod devices;
use devices::Device;
use devices::EthernetFrame;



fn main() {
    let device1: Device = Device::new("Device1".to_string(), (0x00, 0x11, 0x22, 0x33, 0x44, 0x55));
    let mut device2: Device = Device::new("Device2".to_string(), (0x66, 0x77, 0x88, 0x99, 0xAA, 0xBB));


    let frame = EthernetFrame::new(
        device2.mac_address,
        device1.mac_address,
        0x0800, 
        vec![1, 2, 4, 8, 16, 32, 64, 128],
    ).expect("Failed to create Ethernet frame");

    device1.send_frame(&mut device2, frame);

    println!("{} received {} frames", device2.name, device2.received_buffer.len());
    for frame in device2.received_buffer.iter() {
        for byte in frame.payload.iter() {
            print!("{} ", byte);
        }
        println!();
    }
}
