extern crate embedded_hal;

use embedded_hal::digital::v2::OutputPin;

pub struct VirtualPinOutput {
    state: bool
}

impl VirtualPinOutput {
    fn new() -> Self {
        VirtualPinOutput { state: false }
    }
}

impl OutputPin for VirtualPinOutput {
    type Error = ();

    fn set_low(&mut self) -> Result<(), Self::Error> {
        self.state = false;
        Ok(())
    }

    fn set_high(&mut self) -> Result<(), Self::Error> {
        self.state = true;
        Ok(())
    }
}

pub struct MultiplexPinOutput2By2<P1, P2, P3, P4> {
    vgpio: [VirtualPinOutput; 4],

    row: usize,

    p1: P1,
    p2: P2,
    p3: P3,
    p4: P4
}

impl <P1, P2, P3, P4> MultiplexPinOutput2By2<P1, P2, P3, P4> where P1: OutputPin, P2: OutputPin, P3: OutputPin, P4: OutputPin {
    pub fn new(p1: P1, p2: P2, p3: P3, p4: P4) -> Self {
        let vgpio: [VirtualPinOutput; 4] = [
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new()
        ];

        let row = 0;

        MultiplexPinOutput2By2 {
            vgpio, row, p1, p2, p3, p4
        }
    }

    pub fn virtual_outputs(&mut self) -> &mut [VirtualPinOutput] {
        &mut self.vgpio
    }

    pub fn update_display(&mut self) {
        self.set_all_low();

        if self.row == 0 {
            self.p1.set_high();
        } else {
            self.p2.set_high();
        }

        let pin_offset = 2 * self.row;
        if self.vgpio[pin_offset].state {
            self.p3.set_high();
        }

        if self.vgpio[pin_offset + 1].state {
            self.p4.set_high();
        }

        self.row = if self.row == 1 { 0 } else { 1 };
    }

    pub fn set_all_low(&mut self) {
        self.p1.set_low();
        self.p2.set_low();
        self.p3.set_low();
        self.p4.set_low();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut multiplexed = MultiplexPinOutput2By2::new(
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new()
        );

        let pins = multiplexed.virtual_outputs();
        pins[0].set_high();

        assert_eq!(multiplexed.p1.state, false);
        assert_eq!(multiplexed.p2.state, false);
        assert_eq!(multiplexed.p3.state, false);
        assert_eq!(multiplexed.p4.state, false);

        multiplexed.update_display();

        assert_eq!(multiplexed.p1.state, true);
        assert_eq!(multiplexed.p2.state, false);
        assert_eq!(multiplexed.p3.state, true);
        assert_eq!(multiplexed.p4.state, false);

        multiplexed.update_display();

        assert_eq!(multiplexed.p1.state, false);
        assert_eq!(multiplexed.p2.state, true);
        assert_eq!(multiplexed.p3.state, false);
        assert_eq!(multiplexed.p4.state, false);
    }
}
