extern crate embedded_hal;

use embedded_hal::digital::v2::OutputPin;

#[derive(Clone, Copy)]
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

macro_rules! define_multiplex {
    ($name:ident, $rows:expr, $cols:expr,
     row_names [$($r_index:expr => [$r_name:ident, $r_type:ident]),*],
     col_names [$($c_index:expr => [$c_name:ident, $c_type:ident]),*]) => {
        pub struct $name<$( $r_type, )* $( $c_type, )*> {
            vgpio: [VirtualPinOutput; ($rows * $cols)],
            row: usize,
            $( $r_name: $r_type, )*
            $( $c_name: $c_type, )*
        }

        impl <$( $r_type, )* $( $c_type, )*> $name<$( $r_type, )* $( $c_type, )*>
            where $( $r_type: OutputPin, )* $( $c_type: OutputPin, )* {
            pub fn new($( $r_name: $r_type, )* $( $c_name: $c_type, )*) -> Self {
                let vgpio: [VirtualPinOutput; ($rows * $cols)] = [VirtualPinOutput::new(); ($rows * $cols)];

                Self {
                    vgpio, row: 0, $( $r_name, )* $( $c_name, )*
                }
            }

            pub fn virtual_outputs(&mut self) -> &mut [VirtualPinOutput] {
                &mut self.vgpio
            }

            pub fn update_display(&mut self) {
                self.set_all_low();

                $( if self.row == $r_index { self.$r_name.set_high(); } )*

                let pin_offset = $cols * self.row;

                $(
                    if self.vgpio[pin_offset + $c_index].state {
                        self.$c_name.set_high();
                    }
                )*

                // Iterate and wrap around if needed
                self.row += 1;
                if self.row == $rows {
                    self.row = 0;
                }
            }

            pub fn set_all_low(&mut self) {
                $(self.$r_name.set_low();)*
                $(self.$c_name.set_low();)*
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_2_by_2() {
        define_multiplex!(
            MultiplexPinOutput2By2, 2, 2,
            row_names [
                0 => [p1, P1],
                1 => [p2, P2]
            ],
            col_names [
                0 => [p3, P3],
                1 => [p4, P4]
            ]
        );

        let mut multiplexed = MultiplexPinOutput2By2::new(
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new()
        );

        let pins = multiplexed.virtual_outputs();
        assert_eq!(pins.len(), 4);
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

    #[test]
    fn test_4_by_4() {
        define_multiplex!(
            MultiplexPinOutput4By4, 4, 4,
            row_names [
                0 => [p1, P1],
                1 => [p2, P2],
                2 => [p3, P3],
                3 => [p4, P4]
            ],
            col_names [
                0 => [p5, P5],
                1 => [p6, P6],
                2 => [p7, P7],
                3 => [p8, P8]
            ]
        );

        let mut multiplexed = MultiplexPinOutput4By4::new(
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new(),
            VirtualPinOutput::new()
        );

        let pins = multiplexed.virtual_outputs();
        assert_eq!(pins.len(), 16);

        pins[0].set_high();
        pins[1].set_high();

        assert_eq!(multiplexed.p1.state, false);
        assert_eq!(multiplexed.p2.state, false);
        assert_eq!(multiplexed.p3.state, false);
        assert_eq!(multiplexed.p4.state, false);
        assert_eq!(multiplexed.p5.state, false);
        assert_eq!(multiplexed.p6.state, false);
        assert_eq!(multiplexed.p7.state, false);
        assert_eq!(multiplexed.p8.state, false);

        multiplexed.update_display();

        assert_eq!(multiplexed.p1.state, true);
        assert_eq!(multiplexed.p2.state, false);
        assert_eq!(multiplexed.p3.state, false);
        assert_eq!(multiplexed.p4.state, false);
        assert_eq!(multiplexed.p5.state, true);
        assert_eq!(multiplexed.p6.state, true);
        assert_eq!(multiplexed.p7.state, false);
        assert_eq!(multiplexed.p8.state, false);

        multiplexed.update_display();

        assert_eq!(multiplexed.p1.state, false);
        assert_eq!(multiplexed.p2.state, true);
        assert_eq!(multiplexed.p3.state, false);
        assert_eq!(multiplexed.p4.state, false);
        assert_eq!(multiplexed.p5.state, false);
        assert_eq!(multiplexed.p6.state, false);
        assert_eq!(multiplexed.p7.state, false);
        assert_eq!(multiplexed.p8.state, false);
    }
}


