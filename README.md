# Ethernet Emulation

This project is a very basic emulation of Ethernet devices and Ethernet II frames.

## What it currently emulates

- An `EthernetFrame` type with:
  - Destination MAC address
  - Source MAC address
  - EtherType
  - Payload data (`Vec<u8>`)
- Basic frame validation when creating a frame:
  - EtherType must be Ethernet II (`>= 1536`)
  - Payload must be `<= 1500` bytes
- A `Device` type with:
  - A device name
  - A MAC address
  - A receive buffer (`received_buffer`) for incoming frames
- Simple direct delivery:
  - One device can send a frame directly to another device
  - The receiver stores that frame in its buffer

## Current demo flow

In [src/main.rs](src/main.rs):

1. Two devices are created with different MAC addresses.
2. One Ethernet frame is built (`EtherType 0x0800`, small payload).
3. Device 1 sends the frame to Device 2.
4. The program prints how many frames Device 2 received and the payload bytes.

This models the core idea of Ethernet communication at a very simple level: create a frame, send it, and receive it.

## Future plan: switch emulation

A future version will emulate Ethernet switches so devices do not send directly to each other.

Planned switch behavior:

- Learn source MAC addresses and map them to switch ports (MAC table)
- Forward unicast frames only to the correct destination port when known
- Flood unknown destination and broadcast frames to all other ports

That will make the simulation closer to real Layer 2 network behavior.
