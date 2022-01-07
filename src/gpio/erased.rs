//! Pin type that exchanges zero size to unify the type and allow grouping
use core::{marker::PhantomData, convert::Infallible};
use embedded_hal::digital::blocking::{OutputPin, InputPin};

use super::*;

/// Type erased pin, allowing for grouping in arrays
pub struct EPin<MODE> {
    pin_index: u8,
    _mode: PhantomData<MODE>,
}

impl<MODE> EPin<MODE> {
    pub fn new(pin_index: u8) -> Self {
        Self { pin_index, _mode: PhantomData }
    }
}

impl<MODE> EPin<Output<MODE>> {
    fn _set_high(&self) {
        let glb = unsafe { &*pac::GLB::ptr() };
        glb.gpio_cfgctl32.modify(|r, w| unsafe { w.bits(r.bits() | (1 << self.pin_index)) });
    }

    fn _set_low(&self) {
        let glb = unsafe { &*pac::GLB::ptr() };
        glb.gpio_cfgctl32.modify(|r, w| unsafe { w.bits(r.bits() & ( !(1 << self.pin_index))) });
    }
}

impl<MODE> EPin<Input<MODE>> {
    fn _is_high(&self) -> bool {
        let glb = unsafe { &*pac::GLB::ptr() };
        glb.gpio_cfgctl30.read().bits() & (1 << self.pin_index) > 0
    }

    fn _is_low(&self) -> bool {
        let glb = unsafe { &*pac::GLB::ptr() };
        glb.gpio_cfgctl30.read().bits() & (1 << self.pin_index) == 0
    }
}

impl<MODE> OutputPin for EPin<Output<MODE>> {
    type Error = Infallible;

    fn set_high(&mut self) -> Result<(), Self::Error> {
        Ok(self._set_high())
    }

    fn set_low(&mut self) -> Result<(), Self::Error> {
        Ok(self._set_low())
    }
}

impl<MODE> InputPin for EPin<Input<MODE>> {
    type Error = Infallible;

    fn is_high(&self) -> Result<bool, Self::Error> {
        Ok(self._is_high())
    }

    fn is_low(&self) -> Result<bool, Self::Error> {
        Ok(self._is_low())
    }
}