//! 1. [uno-datasheet](https://docs.arduino.cc/resources/datasheets/ABX00087-datasheet.pdf) page 13, 18.
//! 2. [uno-schematic](https://docs.arduino.cc/resources/schematics/ABX00087-schematics.pdf) page 1.
//! 3. [ra4m1-datasheet](https://www.renesas.com/us/en/document/mah/renesas-ra4m1-group-users-manual-hardware) page 369-374.
#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use ra4m1_pac::Peripherals;

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    loop {
        // Test blinking a LED on D7 (P112)
        peripherals
            .PORT1
            .pdr()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 12)) });
        peripherals
            .PORT1
            .podr()
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << 12)) });
    }
}
