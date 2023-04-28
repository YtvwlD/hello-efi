#![no_main]
#![no_std]

use uefi::prelude::*;
use uefi_services::println;

#[entry]
fn main(_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi_services::init(&mut system_table).unwrap();
    println!("Hello, world!");
    println!(
        "(running on {} {} which claims to be compatible to UEFI {})",
        system_table.firmware_vendor(),
        system_table.firmware_revision(),
        system_table.uefi_revision(),
    );
    println!("looping...");
    loop {
        core::hint::spin_loop()
    }
}
