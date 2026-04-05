pub struct EthernetFrame {
    destination_mac: (u8, u8, u8, u8, u8, u8),
    source_mac: (u8, u8, u8, u8, u8, u8),
    ethertype: u16,
    pub payload: Vec<u8>,
}

impl EthernetFrame {
    pub fn new(
        destination_mac: (u8, u8, u8, u8, u8, u8),
        source_mac: (u8, u8, u8, u8, u8, u8),
        ethertype: u16,
        payload: Vec<u8>,
    ) -> Result<Self, String> {
        if ethertype < 1536 {
            return Err("EtherType must be Ethernet II".to_string());
        }

        if payload.len() > 1500 {
            return Err("Payload exceeds maximum Ethernet frame size".to_string());
        }

        Ok(Self {
            destination_mac,
            source_mac,
            ethertype,
            payload,
        })
    }
}

pub struct Device {
    pub name: String,
    pub mac_address: (u8, u8, u8, u8, u8, u8),
    pub received_buffer: Vec<EthernetFrame>,
}
impl Device {
    pub fn new(name: String, mac_address: (u8, u8, u8, u8, u8, u8)) -> Self {
        Device {
            name,
            mac_address,
            received_buffer: Vec::new(),
        }
    }

    pub fn send_frame(&self, device_receiver: &mut Device, frame: EthernetFrame) {
        device_receiver.received_buffer.push(frame);
    }
}