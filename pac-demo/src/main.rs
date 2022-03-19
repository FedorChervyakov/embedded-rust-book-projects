#![no_std]
#![no_main]

// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
// use panic_abort as _; // requires nightly
// use panic_itm as _; // logs messages over ITM; requires ITM support
// use panic_semihosting as _; // logs messages to the host stderr; requires a debugger

use cortex_m_rt::entry;
use stm32f4::stm32f429 as f4;
use rtt_target::{rtt_init_print, rprintln};

const XTAL_FREQUENCY: u32 = 8_000_000; // Hz

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let cp = cortex_m::Peripherals::take().unwrap();
    let p = f4::Peripherals::take().unwrap();

    // Critical section, interrupt-free closure
    cortex_m::interrupt::free(|_cs| {
        // TODO: Setup clocks
        // Goes sorta like this ???
        // We start on the HSI
        // 1. Configure PLL
        // 1.1 Select HSE osc as pll source
        // 1.2 Setup division factors
        // 2. Use PLL as the SYSCLK
        //

        // SYSCLK from HSE
        p.RCC.cr.write(|w| w.hseon().on());
        p.RCC.cfgr.write(|w| w.sw().hse());

        // Enable peripherals
        p.RCC.ahb1enr.write(|w| w.gpiogen().set_bit());
        // TODO: enable timer

        // Configure pin PG13 as push-pull output with pull down
        p.GPIOG.moder.write(|w| w.moder13().output()); // set general purpose output mode
        p.GPIOG.otyper.write(|w| w.ot13().clear_bit()); // output push-pull
        p.GPIOG.pupdr.write(|w| w.pupdr13().pull_down()); // enable pull-down

        // TODO: configure timer
    });

    match p.RCC.cfgr.read().sws().variant() {
        Some(f4::rcc::cfgr::SWS_A::HSI) => rprintln!("HSI"),
        Some(f4::rcc::cfgr::SWS_A::HSE) => rprintln!("HSE"),
        Some(f4::rcc::cfgr::SWS_A::PLL) => rprintln!("PLL"),
        None => rprintln!("Else!")
    };

    while p.RCC.cr.read().hserdy().is_not_ready() {}

    match p.RCC.cfgr.read().sws().variant() {
        Some(f4::rcc::cfgr::SWS_A::HSI) => rprintln!("HSI"),
        Some(f4::rcc::cfgr::SWS_A::HSE) => rprintln!("HSE"),
        Some(f4::rcc::cfgr::SWS_A::PLL) => rprintln!("PLL"),
        None => rprintln!("Else!")
    };

    loop {
        // TODO: trigger on timer ??? use interrupts???
        //p.GPIOG.bsrr.write(|w| w.bs13().set_bit());
        //rprintln!("Led ON");

        //p.GPIOG.bsrr.write(|w| w.br13().set_bit());
        //rprintln!("Led OFF");
    }
}
