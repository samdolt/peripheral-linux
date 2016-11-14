extern crate peripheral;

extern crate sysfs_gpio;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}



pub mod gpio;
pub use gpio::GPIO;

