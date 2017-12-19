# gps-rs
A Rust library for parsing GPS strings

### Example usage:

```Rust
extern crate gps;

use gps::FromNmea;

fn main() {
    let message_gga = "$GPGGA,184901.50,3256.3952158,N,11701.6490440,W,1,16,0.8,260.760,M,-32.661,M,,*57";
    let message_zda = "$GPZDA,184901.50,01,12,2017,00,00*60";
    let message_rmc = "$GPRMC,184902.00,A,3256.3952143,N,11701.6490461,W,0.05,106.83,011217,11.5,E,A,S*61";

    let gga = gps::NMEA::new(message_gga).expect("No valid NMEA message in first string");
    let zda = gps::NMEA::new(message_zda).expect("No valid NMEA message in second string");
    let rmc = gps::NMEA::new(message_rmc).expect("No valid NMEA message in third string");


    println!("{:?}", gps::GPGGA::from_nmea(&gga));
    println!("{:?}", gps::GPZDA::from_nmea(&zda));
    println!("{:?}", gps::GPRMC::from_nmea(&rmc));
}

```
