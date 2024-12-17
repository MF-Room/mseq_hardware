//TODO: Implement DMA for read & write, c.f. github for example

use stm32f4xx_hal::{
    pac::{self, USART1},
    prelude::*,
    serial::{
        config::{DmaConfig, StopBits::STOP1},
        Config, Serial,
    },
};

pub struct Midi {
    serial: Serial<USART1>,
}

impl Midi {
    //TODO: Remove unwraps
    pub fn new() -> Midi {
        let dp = pac::Peripherals::take().unwrap();
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.freeze();
        let gpioa = dp.GPIOA.split();
        let rx_1 = gpioa.pa10.into_alternate();
        let tx_1 = gpioa.pa9.into_alternate();

        let serial = Serial::new(
            dp.USART1,
            (tx_1, rx_1),
            Config::default()
                .baudrate(31250.bps())
                .wordlength_8()
                .parity_none()
                .stopbits(STOP1)
                .dma(DmaConfig::None),
            &clocks,
        )
        .unwrap();

        Midi { serial }
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        let res = self.serial.read();
        if let Ok(res) = res {
            Some(res)
        } else {
            None
        }
    }
}
