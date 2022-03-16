#![no_std]
#![no_main]
use cortex_m::peripheral::{syst, Peripherals, SYST};
use cortex_m_rt::entry;
use panic_halt as _;

use rtt_target::{rtt_init_print, rprintln};


fn delay(systick: &mut SYST, delay_ticks: u32) -> () {
    systick.set_reload(delay_ticks);
    systick.clear_current();
    systick.enable_counter();
    
    while !systick.has_wrapped() {
        // Loop
    }
}

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take().unwrap();
    let mut systick = peripherals.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);

    rtt_init_print!();

    rprintln!("It's time!");

    loop {
        delay(&mut systick, 1_000_000_000);

        rprintln!("So long!");
    }
}
