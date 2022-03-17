#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m::peripheral::{
    syst,
    SYST,
};
use cortex_m_rt::entry;
use stm32f4::stm32f429;
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
    rtt_init_print!();

    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f429::Peripherals::take().unwrap();

    let mut systick = cp.SYST;
    systick.set_clock_source(syst::SystClkSource::Core);

    p.RCC.ahb1enr.write(|w| w.gpiogen().set_bit());

    let gpiog = p.GPIOG;

    gpiog.moder.write(|w| w.moder13().output()); // set general purpose output mode
    gpiog.otyper.write(|w| w.ot13().clear_bit());  // output push-pull
    gpiog.pupdr.write(|w| unsafe { w.pupdr13().bits(2) }); // enable pull-down

    loop {
        gpiog.bsrr.write(|w| w.bs13().set_bit());
        rprintln!("Led ON");

        delay(&mut systick, 100_000_000);

        gpiog.bsrr.write(|w| w.br13().set_bit());
        rprintln!("Led OFF");

        delay(&mut systick, 100_000_000);
    }
}
