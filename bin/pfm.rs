#![no_std]
#![no_main]

use cortex_m_semihosting::hprintln;
// pick a panicking behavior
use panic_halt as _; // you can put a breakpoint on `rust_begin_unwind` to catch panics
                     // use panic_abort as _; // requires nightly
                     // use panic_itm as _; // logs messages over ITM; requires ITM support
                     // use panic_semihosting as _; // logs messages to the host stderr; requires a debugger
use cortex_m_rt::entry;
use pfm as _;

// LD3
const LED: u32 = 14;

#[entry]
fn main() -> ! {
    let cp = cortex_m::Peripherals::take().unwrap();
    let p = stm32f7::stm32f7x7::Peripherals::take().unwrap();

    let mut delay = cortex_m::delay::Delay::new(cp.SYST, 8_000_000);

    // TODO: workaround about timer for led blink
    let pins = p.GPIOB;

    let clear_mask_one_bit: u32 = 0xffffffff ^ (0b1 << LED);
    let clear_mask_two_bit: u32 = 0xffffffff ^ (0b11 << (LED * 2));

    // Set OSPEEDR to low speed for PB7(LD2)
    pins.ospeedr
        .modify(|r, w| unsafe { w.bits(r.bits() & clear_mask_two_bit) });

    // Set OTYPER to b0 for PB7(LD2)
    pins.otyper
        .modify(|r, w| unsafe { w.bits(r.bits() & clear_mask_one_bit | (0b1 << LED)) });

    // Set push-pull output
    pins.pupdr
        .modify(|r, w| unsafe { w.bits(r.bits() & clear_mask_two_bit) });

    // Set MODE to b01 for PB7(LD2)
    pins.moder
        .modify(|r, w| unsafe { w.bits(r.bits() & clear_mask_two_bit | (0b01 << (LED * 2))) });

    hprintln!("Hello, world!").unwrap();

    loop {
        pins.odr
            .modify(|r, w| unsafe { w.bits(r.bits() & clear_mask_one_bit) });
        pins.bsrr.write(|w| unsafe { w.bits(1 << LED) });
        hprintln!("-").unwrap();
        delay.delay_ms(1000);

        pins.odr
            .modify(|r, w| unsafe { w.bits(r.bits() | (1 << LED)) });
        pins.bsrr.write(|w| unsafe { w.bits(1 << LED) });
        hprintln!("+").unwrap();
        delay.delay_ms(1000);
    }
}
