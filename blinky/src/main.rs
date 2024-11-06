#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use ra4m1_pac::Peripherals; // Ensure you're importing from your PAC crate

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();

    // Your code here

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
