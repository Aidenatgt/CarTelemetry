#![no_std]
#![no_main]

use panic_halt as _;

macro_rules! delay {
    ($ns:expr) => {
        arduino_hal::delay_ms($ns);
    };
    ($ns:expr, $us:expr) => {
        arduino_hal::delay_us($ms * 1000 + $us);
    };
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    /*
     * For examples (and inspiration), head to
     *
     *     https://github.com/Rahix/avr-hal/tree/main/examples
     *
     * NOTE: Not all examples were ported to all boards!  There is a good chance though, that code
     * for a different board can be adapted for yours.  The Arduino Uno currently has the most
     * examples available.
     */

    let mut led = pins.d13.into_output();
    led.set_low();

    loop {
        led.toggle();
        arduino_hal::delay_ms(200);
        led.toggle();
        arduino_hal::delay_ms(300);
        led.toggle();
        delay!(500);
        led.toggle();
        delay!(1000);
        led.toggle();
        delay!(1000);
        led.toggle();
    }
}
