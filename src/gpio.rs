use peripheral::digital_io::*;
use peripheral::Result;
use peripheral::Error;

use sysfs_gpio::{Direction, Pin};
use sysfs_gpio::Error as _Error;

// Note: We can't used std from trait, as Error is not declared
// in this crate
fn from(err: _Error) -> Error {
    match err {
        _Error::Io(_) => Error::IOError,
        _Error::Unexpected(_) => Error::Unexpected,
        _Error::InvalidPath(_) => Error::Invalid,
    }
}

// As we can't implement from(_Error) for Error (see above),
// we can't use Rust try! macro 
macro_rules! etry {
    ($e:expr) => (match $e {
        Ok(val) => val,
        Err(err) => return Err(from(err)),
    });
}

pub struct GPIO {
    pin: Pin,
    was_exported: bool
}

impl GPIO {
    pub fn new(num: u64) -> Result<GPIO>{
        let mut io = GPIO {
            pin: Pin::new(num),
            was_exported: false,
        };

        io.was_exported = io.pin.is_exported();

        etry!(io.pin.export());

        Ok(io)
    }

}

impl DigitalIn for GPIO {
    fn read(&self) -> Result<bool> {
        match etry!(self.pin.get_value()) {
            0 => Ok(false),
            1 => Ok(true),
            _ => unreachable!(),
        }
    }

    fn set_to_input(&mut self) -> Result<()> {
        etry!(self.pin.set_direction(Direction::In));
        Ok(())
    }
}

impl DigitalOut for GPIO {
    fn set_low(&mut self) -> Result<()> {
        etry!(self.pin.set_value(0));
        Ok(())
    }

    fn set_high(&mut self) -> Result<()> {
        etry!(self.pin.set_value(1));
        Ok(())
    }

    fn set_to_output(&mut self) -> Result<()> {
        etry!(self.pin.set_direction(Direction::Out));
        Ok(())
    }
}

impl Drop for GPIO {
    fn drop(&mut self) {
        if self.was_exported {
            let _ = self.pin.unexport();
        }
    }
}

impl DigitalIO for GPIO {}