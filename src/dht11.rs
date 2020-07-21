extern crate stm32f1xx_hal as hal;
use hal::delay::Delay;
use hal::prelude::*;
use hal::stm32;

pub fn dht11  (delay:&mut Delay,pin: &mut hal::gpio::gpiob::PB9<hal::gpio::Output<hal::gpio::OpenDrain>>)  
-> (u8,u8,u8,u8){
    // Initlialize Delay
    let mut delay = delay;
    // Initliaze Pin PA3
    let mut pa3 = pin; 
    // Set Pin High
    pa3.set_high();
    // Delay for 100 microSecond
    delay.delay_us(100u32);
    // Set Pin Low 
    pa3.set_low();
    // Delay for 18 microSecond
    delay.delay_ms(18u32);
    // Set Pin High
    pa3.set_high();

    // Check Set bit
    while set_bit(pa3){}
     // Check Clear bit
    while clear_bit(pa3){}
    // Check Set bit
    while set_bit(pa3){}
    // Get Data for humidaty Intiger Data
    let mut hum_int = response(&mut delay,pa3);
    // Get Data for humidaty Float Data
    let mut hum_float = response(&mut delay,pa3);
    // Get Data for Temp intiger Data
    let mut temp_int = response(&mut delay,pa3);
    // Get Data for Temp Float Data
    let mut temp_float = response(&mut delay,pa3);
    // Get Data for Check Sum 
    let mut check_sum = response(&mut delay,pa3);

    // Return The Data
    (convert_bit(&mut hum_int),convert_bit(&mut hum_float),convert_bit(&mut temp_int),convert_bit(&mut temp_float))
}

// // check bit set or not
fn set_bit(pin:& dyn hal::prelude::_embedded_hal_digital_InputPin)->bool{

    // unsafe{
    //     let gpioa_pin = &*GPIOA::ptr();
    //     gpioa_pin.idr.read().idr3().bit_is_set()
    // }
    
    pin.is_high()
}
// // check bit is clear or not
fn clear_bit(pin:& dyn hal::prelude::_embedded_hal_digital_InputPin)->bool{
    // unsafe{
    //     let gpioa_pin = &*GPIOA::ptr();
    //     gpioa_pin.idr.read().idr3().bit_is_clear()
    // }
    pin.is_low()
}
// Geting Response and convert it to Array
fn response(delay:&mut Delay,pin:& dyn hal::prelude::_embedded_hal_digital_InputPin)->[u8;8]{
    let mut data = [0u8;8];
    let delay = delay;
    for byte in data.iter_mut(){
        while clear_bit(pin){}
        delay.delay_us(25u32);
        if set_bit(pin){
            *byte = 1
        }else{
            *byte = 0;
        }
         while set_bit(pin){}
    }
    data
}

// Convert bit to Byte 
fn convert_bit(data:&mut[u8;8]) -> u8{

    let arr = [128,64,32,16,8,4,2,1];
    let vec = data;
    let mut int = 0;
    for i in 0..8{
        int = int + arr[i]*vec[i]
    }
    int
}