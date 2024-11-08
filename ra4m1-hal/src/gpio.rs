pub struct Input;
pub struct Output;

macro_rules! define_pins {
    (port_num: $port_num:tt, pins: [$($pin_num:tt),* $(,)?]) => {
        use core::marker::PhantomData;
        // TODO: FIgure out how to properly import local crate in macro call
        use crate::gpio::{Input,Output};

        $(
            paste::paste! {
                pub struct [<P $port_num $pin_num>]<MODE> {
                    pub _mode: PhantomData<MODE>,
                }

                impl embedded_hal::digital::ErrorType for [<P $port_num $pin_num>]<Output> {
                    type Error = core::convert::Infallible;
                }

                impl embedded_hal::digital::ErrorType for [<P $port_num $pin_num>]<Input> {
                    type Error = core::convert::Infallible;
                }

                impl embedded_hal::digital::OutputPin for [<P $port_num $pin_num>]<Output> {
                    fn set_high(&mut self) -> Result<(), Self::Error> {
                        let port = unsafe { &*ra4m1_pac::[<PORT $port_num>]::ptr() };
                        port.podr()
                            .write(|w| unsafe { w.bits(port.podr().read().bits() | (1 << $pin_num)) });
                        Ok(())
                    }

                    fn set_low(&mut self) -> Result<(), Self::Error> {
                        let port = unsafe { &*ra4m1_pac::[<PORT $port_num>]::ptr() };
                        port.podr()
                            .write(|w| unsafe { w.bits(port.podr().read().bits() & !(1 << $pin_num)) });
                        Ok(())
                    }
                }

                impl embedded_hal::digital::InputPin for [<P $port_num $pin_num>]<Input> {
                    fn is_high(&mut self) -> Result<bool, Self::Error> {
                        let port = unsafe { &*ra4m1_pac::[<PORT $port_num>]::ptr() };
                        Ok((port.pidr().read().bits() & (1 << $pin_num)) != 0)
                    }

                    fn is_low(&mut self) -> Result<bool, Self::Error> {
                        Ok(!self.is_high()?)
                    }
                }
            }
        )*
    };
}

pub mod port0 {
    define_pins!(
        port_num: 0,
        pins: [00,01,02,03,04,10,11,12,13,14,15]
    );
}

pub mod port1 {
    define_pins!(
        port_num: 1,
        pins: [00,01,02,03,04,05,06,07,08,09,10,11,12,13]
    );
}

pub mod port2 {
    define_pins!(
        port_num: 2,
        pins: [00,01,04,05,06,12,13,14,15]
    );
}

pub mod port3 {
    define_pins!(
        port_num: 3,
        pins: [00,01,02,03,04]
    );
}

pub mod port4 {
    define_pins!(
        port_num: 4,
        pins: [00,01,02,07,08,09,10,11]
    );
}

pub mod port5 {
    define_pins!(
        port_num: 5,
        pins: [00,01,02]
    );
}

pub mod port9 {
    define_pins!(
        port_num: 9,
        pins: [14,15]
    );
}
