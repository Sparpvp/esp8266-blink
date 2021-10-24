#![no_std]
#![no_main]

use esp8266_hal::{
    prelude::*,
    target::Peripherals,
    gpio::{Gpio2, Output, PushPull},
};
use panic_halt as _;
use xtensa_lx::mutex::{CriticalSectionMutex, Mutex};

static LED: CriticalSectionMutex<Option<Gpio2<Output<PushPull>>>> = CriticalSectionMutex::new(None);

fn blink(dp: esp8266_hal::target::Peripherals) -> ! {
    // Acquire GPIO(s) pheriperals
    let pins = dp.GPIO.split();

    let serial = dp
        .UART0
        .serial(pins.gpio1.into_uart(), pins.gpio3.into_uart());

    // Configure GPIO2 pin as a push-pull output
    let mut led = pins.gpio2.into_push_pull_output();

    // Init esp8266's timer
    let (mut timer, _) = dp.TIMER.timers();

    led.set_high().unwrap();
    (&LED).lock(|led_locked| *led_locked = Some(led));

    let _handler = serial.attach_interrupt(|serial| {
        if let Ok(byte) = serial.read() {
            let _ = serial.write(byte);
        }
    });

    loop {
        timer.delay_ms(500);
        (&LED).lock(|led| led.as_mut().unwrap().toggle().unwrap());
    }
}

#[entry]
fn main() -> ! {
    // Get access to the device -> Peripherals
    let dp = Peripherals::take().unwrap();

    blink(dp);
}