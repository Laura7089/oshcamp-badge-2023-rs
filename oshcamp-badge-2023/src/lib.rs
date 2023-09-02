#![no_std]

use esp8266_hal::prelude::*;

use esp8266_hal::gpio::{
    Gpio0, Gpio12, Gpio13, Gpio14, Gpio2, Gpio4, Gpio5, Input, Output, PullUp, PushPull,
};
use esp8266_hal::target::Peripherals as InnerPerips;

pub struct Peripherals {
    pub buzzer: Gpio13<Output<PushPull>>,
    pub buttons: Buttons,
}

type GpioIn = Input<PullUp>;

#[allow(non_snake_case)]
pub struct Buttons {
    /// Note that this is also the RGB LED pin
    pub SW1: Gpio5<GpioIn>,
    pub SW2: Gpio4<GpioIn>,
    pub SW3: Gpio0<GpioIn>,
    pub SW4: Gpio2<GpioIn>,
    pub SW5: Gpio12<GpioIn>,
    pub SW6: Gpio14<GpioIn>,
}

impl Peripherals {
    pub fn take() -> Self {
        let inner = InnerPerips::take().unwrap();
        let pins = inner.GPIO.split();
        let buzzer = pins.gpio13.into_push_pull_output();

        let buttons = Buttons {
            SW1: pins.gpio5.into_pull_up_input(),
            SW2: pins.gpio4.into_pull_up_input(),
            SW3: pins.gpio0.into_pull_up_input(),
            SW4: pins.gpio2.into_pull_up_input(),
            SW5: pins.gpio12.into_pull_up_input(),
            SW6: pins.gpio14.into_pull_up_input(),
        };

        Self { buzzer, buttons }
    }
}
