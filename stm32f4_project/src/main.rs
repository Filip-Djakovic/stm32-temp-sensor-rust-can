#![no_std]
#![no_main]

use core::borrow::BorrowMut;

use bxcan::FilterOwner;
use bxcan::StandardId;
use bxcan::RegisterBlock;
use cortex_m::asm;
use cortex_m::peripheral::{SYST, syst::SystClkSource};
use stm32f4xx_hal::pac::ADC1;
use cortex_m_rt::exception;
use stm32f4xx_hal::pac::TIM2;
use stm32f4xx_hal::rcc::Rcc;
use stm32f4xx_hal::stm32::CAN1;
use stm32f4xx_hal::timer::Timer;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use panic_halt as _;
use stm32f4xx_hal::timer::Event;
use stm32f4xx_hal::{
    adc::{config::AdcConfig, Adc, Temperature}, gpio::{Alternate, AF9, gpiod::{PD0, PD1}}, pac::{self}, rcc::RccExt,prelude::*
};
use stm32f4xx_hal::gpio::{Output, PushPull};
use stm32f4xx_hal::gpio::gpiob::PB7;
pub struct Can1Wrapper(pub CAN1);

unsafe impl bxcan::Instance for Can1Wrapper {
    const REGISTERS: *mut RegisterBlock = CAN1::ptr() as *mut RegisterBlock;

}

unsafe impl FilterOwner for Can1Wrapper {
    const NUM_FILTER_BANKS: u8 = 28;
}

#[entry]
fn main() -> ! {
    //can();
    sleep_wakeup();
    loop {
    }
}
fn button_click() -> bool //method for check is button clicked
{ 
    let p = pac::Peripherals::take().unwrap();
    let gpioc = p.GPIOC.split();
    let button = gpioc.pc13;

    loop {
        if button.is_high(){
            hprintln!("Clicked button").unwrap();
        }
        else {
            hprintln!("Not clicked").unwrap();
        }
    }   
}
fn led() //basic method for blinking led diode
{
    let p = pac::Peripherals::take().unwrap();
    let gpiob = p.GPIOB.split();
    let mut led:
    PB7<Output<PushPull>> = gpiob.pb7.into_push_pull_output();
    led.set_low();

    loop {
        led.set_high();
        for _ in 1..10_000{            
        }
        led.set_low();
        for _ in 1..100_000{
        }
    }; 
}

fn temp( adc:&mut Adc<ADC1>) -> f32 //method for read temperature 
{
    hprintln!("Calibration values").unwrap();
    //let mut adc = Adc::adc1(adc, true, AdcConfig::default());
    adc.enable_temperature_and_vref();

    let mut tmp = Temperature;
    let adc_value: u16 = match adc.read(&mut tmp) {
        Ok(value) => value,
        Err(_) => {
            hprintln!("Error reading from ADC").unwrap();
            return 0.0;
        } 
    };

    let ts_cal1: u16 = unsafe { *(0x1FFF_7A2C as *const u16) };
    let ts_cal2: u16 = unsafe { *(0x1FFF_7A2E as *const u16) };
    hprintln!("TS_CAL1: {}", ts_cal1).unwrap();
    hprintln!("TS_CAL2: {}", ts_cal2).unwrap();
    hprintln!("ADC VaLue {}",adc_value).unwrap();

    let temperature_celsius: f32 = 30.0
        + ((adc_value  as f32 - ts_cal1 as f32) * (110.0 - 30.0))
            / (ts_cal2 as f32 - ts_cal1 as f32);
    hprintln!("Temperature is {:?}", temperature_celsius).unwrap();
    return temperature_celsius;
}

fn _temp( adc : ADC1) -> f32
{
    hprintln!("Calibration values").unwrap();
    let mut adc = Adc::adc1(adc, true, AdcConfig::default());
    adc.enable_temperature_and_vref();

    let mut tmp = Temperature;
    let adc_value: u16 = match adc.read(&mut tmp) {
        Ok(value) => value,
        Err(_) => {
            hprintln!("Error reading from ADC").unwrap();
            return 0.0;
        } 
    };

    let ts_cal1: u16 = unsafe { *(0x1FFF_7A2C as *const u16) };
    let ts_cal2: u16 = unsafe { *(0x1FFF_7A2E as *const u16) };
    hprintln!("TS_CAL1: {}", ts_cal1).unwrap();
    hprintln!("TS_CAL2: {}", ts_cal2).unwrap();
    hprintln!("ADC VaLue {}",adc_value).unwrap();

    let temperature_celsius: f32 = 30.0
        + ((adc_value  as f32 - ts_cal1 as f32) * (110.0 - 30.0))
            / (ts_cal2 as f32 - ts_cal1 as f32);
    hprintln!("Temperature is {:?}", temperature_celsius).unwrap();
    return temperature_celsius;
}

fn led2(mut gpiob: stm32f4xx_hal::gpio::gpiob::Parts){
    let mut led:PB7<Output<PushPull>> = gpiob.pb7.into_push_pull_output();
    led.set_high();
}
fn led3(pb7: &mut stm32f4xx_hal::gpio::Pin<stm32f4xx_hal::gpio::Output<stm32f4xx_hal::gpio::PushPull>, 'B', 7>){
    pb7.set_high();
}
fn can(temp:f32, can_p: CAN1, pb7: &mut stm32f4xx_hal::gpio::Pin<stm32f4xx_hal::gpio::Output<stm32f4xx_hal::gpio::PushPull>, 'B', 7>,
        mut gpiod: stm32f4xx_hal::gpio::gpiod::Parts, rcc: Rcc) //method for sent can message
{
    //let dp = pac::Peripherals::take().unwrap();
    //let rcc = dp.RCC.constrain();
    rcc.cfgr.freeze();
    unsafe {
        let rcc = &(*pac::RCC::ptr());
        rcc.apb1enr.modify(|_, w| w.can1en().set_bit());
    }
    //let gpiod = dp.GPIOD.split();
    let rx: PD0<Alternate<AF9>> = gpiod.pd0.into_alternate();
    let tx: PD1<Alternate<AF9>> = gpiod.pd1.into_alternate();
    //let can1 = Can1Wrapper(dp.CAN1);
    let can1 = Can1Wrapper(can_p);
    let mut can_builder = bxcan::Can::builder(can1);
    can_builder = can_builder.set_bit_timing(0x001c0013);
    hprintln!("Enable bus").unwrap();
    let mut can = can_builder.enable();   
    let mut filters = can.modify_filters();
    filters.enable_bank(0, bxcan::Fifo::Fifo0, bxcan::filter::Mask32::accept_all()); 
    hprintln!("Detect bus").unwrap();
    drop(filters);

    let data = temp.to_le_bytes();
    let id = StandardId::new(0x123).expect("Failed with create ID");
    let frame = bxcan::Frame::new_data(bxcan::Id::Standard(id), data);

     match nb::block!(can.transmit(&frame)) {
        Ok(_) => {
            hprintln!("Message is sent!").unwrap();
        },
        Err(e) => {
            hprintln!("Error: {:?}", e).unwrap();
        },
    }
     hprintln!("Message is recived").unwrap();
    //loop {
        match can.receive() {
            Ok(msg) => { 
                if let Some(data) = msg.data() {
                    let bytes: [u8; 4] = data[..4].try_into().unwrap();
                    let num = u32::from_le_bytes(bytes);
                    if num==10 {
                       //led3(&mut pb7);
                       led3(pb7);
                    }
                    else {
                        
                    }
                    hprintln!("Message recived is number: {}", num).unwrap();
                } else {
                    hprintln!("Message recived is without data").unwrap();
                }
            }
            Err(nb::Error::WouldBlock) => {
                hprintln!("Without messages").unwrap();
            }
            Err(e) => {
                hprintln!("Error, message recived {:?}", e).unwrap();
            }
        }
    } 
//}
fn sleep_wakeup() //sleep and wakeup method
{
    let dp = pac::Peripherals::take().unwrap();
    let mut cp = cortex_m::Peripherals::take().unwrap();

    let rcc = dp.RCC.constrain();
    //let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
    
    /*cp.SYST.set_clock_source(SystClkSource::Core);
    cp.SYST.set_reload(32_000_000); // Set value for period
    cp.SYST.clear_current();
    cp.SYST.enable_counter();
    cp.SYST.enable_interrupt(); // Enable interrupt*/
    let adc = dp.ADC1;
    let can1 = dp.CAN1;
    let gpiob = dp.GPIOB.split();
    let mut pb7 = gpiob.pb7.into_push_pull_output();
    let gpiod = dp.GPIOD.split();
    let mut adc1 = Adc::adc1(adc, true, AdcConfig::default());
    let mut t: f32  = 0.0;
    t = temp(&mut adc1);
    /*loop {
        //t = temp(&mut adc1);
        //hprintln!("Temperature {}", t).unwrap();
        if t < 25.0{
            asm::wfi();            
        }
        else {
            break;
        }
        hprintln!("Check Temperature").unwrap();
    } */

       can(t,can1,&mut pb7, gpiod, rcc);
   
    


    
}
#[exception]
fn SysTick() {
    hprintln!("Interrupt processing").unwrap();
}