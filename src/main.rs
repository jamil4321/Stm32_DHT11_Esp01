#![no_main]
#![no_std]

use panic_halt as _;
mod dht11;


// extern crate cortex_m;
extern crate cortex_m_rt as rt;
use cortex_m_semihosting::hprintln;
// extern crate panic_semihosting;
extern crate stm32f1xx_hal as hal;

#[macro_use(block)]
extern crate nb;

use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32;
use hal::serial::Serial;
use rt::{entry, exception, ExceptionFrame};

// Import for convert u8 to String
use core::fmt::Write;
use heapless::consts::*;
use heapless::String;

#[entry]
fn main()->!{
    let dp = stm32::Peripherals::take().unwrap();
    let cp = cortex_m::Peripherals::take().unwrap();

    let mut flash = dp.FLASH.constrain();
    let mut rcc = dp.RCC.constrain();

    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    let mut afio = dp.AFIO.constrain(&mut rcc.apb2);

    let mut gpioa = dp.GPIOA.split(&mut rcc.apb2);

    let tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let rx = gpioa.pa10;

    hprintln!("Serial initialized");

    let mut serial = Serial::usart1(dp.USART1, (tx,rx),&mut afio.mapr, 9_600.bps(), clocks, &mut rcc.apb2);

    let (mut tx,mut rx) = serial.split();

    let mut delay = Delay::new(cp.SYST, clocks);

    let mut gpiob = dp.GPIOB.split(&mut rcc.apb2);
    let mut dht_open_drain = gpiob.pb9.into_open_drain_output(&mut gpiob.crh);

    let connect = b"AT+CWJAP=\"Dejavu865\",\"shahid1234\"\r\n";
    let api = b"AT+CIPSTART=\"TCP\",\"192.168.0.104\",5000\r\n";
    let charactor = b"AT+CIPSEND=130\r\n";
    let data = b"POST / HTTP/1.1\r\nHost: 192.168.0.104\r\nContent-Type: application/json\r\nContent-Length: 32\r\n\r\n{";
    let end = b"}\r\n\r\n\r\n";
    for byte in connect.iter(){
        block!(tx.write(*byte)).ok();
    };
    delay.delay_ms(3_000_u16);
    

    loop{
        delay.delay_ms(2_000_u16);
        let (hum,_,temp,_) = dht11::dht11(&mut delay, &mut dht_open_drain);
        hprintln!("Tempreture {}*C & Humadity{}%",temp,hum);
        // Varibale For Temperature 
        let mut tempString = String::<U32>::from("\"Temperature\":");
        // Convert temp u8 value to String and push it to tempString Varible
        let _ = write!(tempString, "{},", temp);
        // Varibale For Humidity
        let mut humString = String::<U32>::from("\"Humidity\":");
        // Convert hum u8 value to String and push it to humString Varible
        let _ = write!(humString, "{}", hum);
        
        for byte in api.iter(){
            block!(tx.write(*byte)).ok();
        };
        delay.delay_ms(1000u32);
        for byte in charactor.iter(){
            block!(tx.write(*byte)).ok();
        };
        delay.delay_ms(1000u32);
        for byte in data.iter(){
            block!(tx.write(*byte)).ok();
        };

        for byte in tempString.as_bytes(){
            block!(tx.write(*byte)).ok();
        };

        for byte in humString.as_bytes(){
            block!(tx.write(*byte)).ok();
        };
        for byte in end.iter(){
            block!(tx.write(*byte)).ok();
        };
        delay.delay_ms(1_000_u16);
       
    }
}
