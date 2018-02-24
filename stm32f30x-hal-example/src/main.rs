/*
 * Tested on an STM32F3Discovery board.
 *
 * USART Pins:
 * Tx = PB6, Rx = PB7
 *
 * SPI Pins:
 * SCK: PA5, MISO: PA6, MOSI: PA7
 *
 * ADC Chip Select: PB5 (GPIO)
 * 
 */

#![deny(unsafe_code)]
#![deny(warnings)]
#![no_std]
#![feature(used)]
#![feature(lang_items)]

extern crate cortex_m;
extern crate cortex_m_rt;
extern crate stm32f30x_hal as hal;
extern crate adc_mcp3008;

#[macro_use(block)]
extern crate nb;

use hal::stm32f30x;
use hal::prelude::*;

use hal::serial::Serial;
use hal::spi::Spi;

use cortex_m::asm;

fn main() {

    let p = stm32f30x::Peripherals::take().unwrap();

    let mut flash = p.FLASH.constrain();
    let mut rcc = p.RCC.constrain();
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut gpiob = p.GPIOB.split(&mut rcc.ahb);
    let mut gpioa = p.GPIOA.split(&mut rcc.ahb);
 
    /* Serial port config */
   
    let tx = gpiob.pb6.into_af7(&mut gpiob.moder, &mut gpiob.afrl);
    let rx = gpiob.pb7.into_af7(&mut gpiob.moder, &mut gpiob.afrl);
    

    let serial = Serial::usart1(p.USART1, (tx, rx), 9_600.bps(), clocks, &mut rcc.apb2);

    let (mut tx, _rx) = serial.split();


    /* SPI Config */

    let sck = gpioa.pa5.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let miso = gpioa.pa6.into_af5(&mut gpioa.moder, &mut gpioa.afrl);
    let mosi = gpioa.pa7.into_af5(&mut gpioa.moder, &mut gpioa.afrl);

    let spi = Spi::spi1(
        p.SPI1,
        (sck, miso, mosi),
        adc_mcp3008::MODE,
        1.mhz(),
        clocks,
        &mut rcc.apb2,
    );

    /* ADC Chip Select */

    let mut nss = gpiob
        .pb5
        .into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);
    
    nss.set_high();

    /* Create the ADC instance */

    let mut adc = adc_mcp3008::Mcp3008::new(spi, nss).unwrap();

    loop {
        let r = adc.read_channel(adc_mcp3008::Channels8::CH0).unwrap();
        let r1 = ((r as u32 * 255 as u32)/1023 as u32) as u8; // scale 10 bits to 8 bits
        block!(tx.write(r1)).ok();
    }
 
}


// As we are not using interrupts, we just register a dummy catch all handler
#[link_section = ".vector_table.interrupts"]
#[used]
static INTERRUPTS: [extern "C" fn(); 240] = [default_handler; 240];

extern "C"  fn default_handler() {
    asm::bkpt();
}

#[lang = "panic_fmt"]
#[no_mangle]
pub extern "C" fn panic_fmt() -> ! {
    loop {}
}

