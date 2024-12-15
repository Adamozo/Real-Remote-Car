#![no_std]
#![no_main]

use arduino_hal::prelude::_embedded_hal_serial_Read;
use panic_halt as _;

const LCD_ADDRESS: u8 = 0x27;
const LCD_BACKLIGHT: u8 = 0x08;
const LCD_ENABLE: u8 = 0x04;
const LCD_COMMAND: u8 = 0x00;
const LCD_DATA: u8 = 0x01;

struct LCD<T> {
    i2c: T,
    backlight: u8,
}

impl<T: embedded_hal::blocking::i2c::Write> LCD<T> {
    fn new(i2c: T) -> Self {
        let mut lcd = LCD {
            i2c,
            backlight: LCD_BACKLIGHT,
        };
        lcd.init();
        lcd
    }

    fn init(&mut self) {
        arduino_hal::delay_ms(50);
        self.write_4bits(0x03);
        arduino_hal::delay_ms(5);
        self.write_4bits(0x03);
        arduino_hal::delay_ms(1);
        self.write_4bits(0x03);
        arduino_hal::delay_ms(1);
        self.write_4bits(0x02);
        self.command(0x28);
        self.command(0x0C);
        self.command(0x06);
        self.command(0x01);
        arduino_hal::delay_ms(2);
    }

    fn write_4bits(&mut self, value: u8) {
        let _ = self.i2c_write(value << 4);
        let _ = self.pulse_enable(value << 4);
    }

    fn command(&mut self, value: u8) {
        let _ = self.write(value, LCD_COMMAND);
    }

    fn write(&mut self, value: u8, mode: u8) -> Result<(), T::Error> {
        let high_bits = value & 0xf0;
        let low_bits = (value << 4) & 0xf0;
        self.i2c_write(high_bits | mode)?;
        self.pulse_enable(high_bits | mode)?;
        self.i2c_write(low_bits | mode)?;
        self.pulse_enable(low_bits | mode)?;
        Ok(())
    }

    fn i2c_write(&mut self, data: u8) -> Result<(), T::Error> {
        self.i2c.write(LCD_ADDRESS, &[data | self.backlight])
    }

    fn pulse_enable(&mut self, data: u8) -> Result<(), T::Error> {
        self.i2c_write(data | LCD_ENABLE)?;
        arduino_hal::delay_us(1);
        self.i2c_write(data & !LCD_ENABLE)?;
        arduino_hal::delay_us(50);
        Ok(())
    }

    fn print_str(&mut self, string: &str) {
        for c in string.bytes() {
            let _ = self.write(c, LCD_DATA);
        }
    }

    fn clear(&mut self) {
        self.command(0x01);
        arduino_hal::delay_ms(2);
    }
}

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);

    let i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    let mut serial = arduino_hal::default_serial!(dp, pins, 57600);

    let mut lcd = LCD::new(i2c);
    let mut buffer = [0u8; 16];
    let mut idx = 0;

    loop {
        if let Ok(b) = nb::block!(serial.read()) {
            if b == b'\n' {
                // Full line received
                if idx > 0 {
                    lcd.clear();
                    lcd.print_str(core::str::from_utf8(&buffer[..idx]).unwrap());
                    idx = 0;
                }
            } else if b != b'\r' && idx < buffer.len() {
                buffer[idx] = b;
                idx += 1;
            }
        }

        arduino_hal::delay_ms(10);
    }
}
