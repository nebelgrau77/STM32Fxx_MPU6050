//! MPU6050 sensor printing to the console
//! 


#![no_std]
#![no_main]

extern crate cortex_m;
extern crate cortex_m_rt as rt;
//extern crate panic_halt;
extern crate panic_semihosting;
extern crate stm32f0xx_hal as hal;

use cortex_m_semihosting::hprintln;

use cortex_m_rt::entry;

use crate::hal::{
    prelude::*,
    stm32,
    delay::Delay,
    i2c::I2c,
};

use mpu6050::Mpu6050;

const BOOT_DELAY_MS: u16 = 200;

#[entry]

fn main() -> ! {
    
    let mut p = stm32::Peripherals::take().unwrap();
    let mut cp = cortex_m::peripheral::Peripherals::take().unwrap();

    cortex_m::interrupt::free(move |cs| {

        let mut rcc = p.RCC.configure().sysclk(8.mhz()).freeze(&mut p.FLASH);

        let mut delay = Delay::new(cp.SYST, &rcc);

        delay.delay_ms(BOOT_DELAY_MS);

        let gpiob = p.GPIOB.split(&mut rcc);
        let scl = gpiob.pb8.into_alternate_af1(cs);
        let sda = gpiob.pb7.into_alternate_af1(cs);
        let i2c = I2c::i2c1(p.I2C1, (scl, sda), 100.khz(), &mut rcc);

        let mut sensor = Mpu6050::new(i2c, delay);

        sensor.init().unwrap();
        //sensor.soft_calib(Steps(100)).unwrap();
        //sensor.calc_variance(Steps(50)).unwrap();

        loop {

            let angles = sensor.get_acc_angles().unwrap(); // 2 values
            let temp = sensor.get_temp().unwrap(); // 1 value
            let gyro = sensor.get_gyro().unwrap(); // 3 values
            let acc = sensor.get_acc().unwrap(); // 3 values

            hprintln!("angles: ({}, {})\n", angles[0], angles[1]).unwrap();
            hprintln!("gyro: ({}, {}, {})\n", gyro[0], gyro[1], gyro[2]).unwrap();
            hprintln!("acc: ({}, {}, {})\n", acc[0], acc[1], acc[2]).unwrap();
            hprintln!("temp: {}\n", temp).unwrap();


        }

    });


    loop {continue;}

}

