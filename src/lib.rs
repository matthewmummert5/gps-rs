use std::str::FromStr;

/// A trait for converting an NMEA object to a GP data struct
pub trait FromNmea {
    fn from_nmea(data: &NMEA) -> Self;
}

/// A struct for holding the time of day: hours, minutes, seconds
#[derive(Debug, Copy, Clone)]
pub struct GpsTime {
    pub h: u8,
    pub m: u8,
    pub s: f32,
}

/// A struct for holding the date: month, day, year
#[derive(Debug, Copy, Clone)]
pub struct GpsDate {
    pub mon:  u8,
    pub day:  u8,
    pub year: u16,
}

/// A struct for holding an NMEA sentence
#[derive(Debug)]
pub struct NMEA {
    pub sentence: String,        // The sentence itself
    pub checksum: u8,            // The single byte checksum
    pub sentence_type: NmeaType, // The type of NMEA sentence (GPGGA, GPRMC, GPZDA, etc)
}

/// An enum for the type of an NMEA sentence
#[derive(Debug, PartialEq)]
#[allow(dead_code)]
pub enum NmeaType {
    GPGGA,
    GPRMC,
    GPZDA,
    Unknown,
}

/// An enum for the quality of a GPS fix
#[allow(dead_code)]
#[derive(Debug)]
pub enum GpsFixQuality {
    InvalidFix,
    GPSFix,
    DGPSFix,
    RTKFixed,
    RTKFloat,
    Unknown,
}

/// An enum for the status of a GPRMC message
#[allow(dead_code)]
#[derive(Debug)]
pub enum RmcStatus {
    Active,
    Void,
}

/// All the data fields from a GPGGA sentence
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct GPGGA {
    pub time:             Option<GpsTime>,
    pub latitude:         Option<f64>,
    pub longitude:        Option<f64>,
    pub quality:          Option<GpsFixQuality>,
    pub num_satellites:   Option<u8>,
    pub hdop:             Option<f64>,
    pub altitude:         Option<f64>,
    pub geoid_seperation: Option<f64>,
    pub age:              Option<f64>,
    pub dstation_id:      Option<u16>,
}

/// All the data fields from a GPZDA sentence
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct GPZDA {
    pub time:         Option<GpsTime>,
    pub date:         Option<GpsDate>,
    pub zone_hours:   Option<u8>,
    pub zone_minutes: Option<u8>,
}

/// All the data fields from a GPRMC sentence
#[allow(dead_code)]
#[derive(Debug, Default)]
pub struct GPRMC {
    pub time:       Option<GpsTime>,
    pub rmc_status: Option<RmcStatus>,
    pub latitude:   Option<f64>,
    pub longitude:  Option<f64>,
    pub sog:        Option<f64>,
    pub cog:        Option<f64>,
    pub date:       Option<GpsDate>,
    pub magvar:     Option<f64>,
}

/// Implement some methods for the NMEA struct
#[allow(dead_code)]
impl NMEA {

    /// Basic constructor for the NMEA type
    pub fn new(s: &str) -> Option<NMEA> {
        //To avoid repeated code, just use the FromStr
        //trait implementation by calling parse()
        //and changing the Result<> to Option<>
        s.parse::<NMEA>().ok()
    }
}

/// Implement some methods for the GpsDate struct
#[allow(dead_code)]
impl GpsDate {
    /// A constructor for creating a new GpsDate.
    fn new(mon: u8, day: u8, year: u16) -> Option<GpsDate> {
        if mon > 12 || day > 31 {
            None
        } else {
            Some(GpsDate {mon: mon, day: day, year: year})
        }
    }
}

/// Here we implement FromStr for NMEA, so that we can
/// make an NMEA type by calling parse() on a &str
impl FromStr for NMEA {
    type Err = u32;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use std::u8;

        //Basically, we need to search the input for a valid NMEA string
        //Get the characters between the first '$' and '*'
        let sentence = s.chars()
                            .skip_while(|&x| x != '$') //Search for the first '$'
                            .skip(1)                   //Skip the '$'
                            .take_while(|&x| x != '*') //Get all the characters until the '*'
                            .collect::<String>();      //Collect the result into a string

        //Check if we had any characters returned at all
        if sentence.len() == 0 {
            return Err(1);
        }

        //Get the 1 byte string checksum as a two-character string
        let ch = s.chars()
            .skip_while(|&x| x != '*')
            .skip(1)
            .take(2)
            .collect::<String>();

        //Now convert the hex. Return Err(2) if it fails
        let cs = match u8::from_str_radix(ch.as_str(), 16) {
            Ok(c)  => c,
            Err(_) => return Err(2),
        };

        //Verify the checksum. Return Err(3) if it fails
        if calc_nmea_checksum(&sentence) != cs {
            return Err(3);
        }

        let sentence_type = get_nmea_type(&sentence);

        Ok(NMEA {
            sentence: sentence,
            checksum: cs,
            sentence_type: sentence_type,
        })
    }
}

/// Here we implement the FromStr trait for NmeaType so that we can determine
/// the type of NMEA string it is by calling parse() in get_nmea_type()
impl FromStr for NmeaType {
    type Err = u32;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "GPGGA" => NmeaType::GPGGA,
            "GPRMC" => NmeaType::GPRMC,
            "GPZDA" => NmeaType::GPZDA,
            _ => NmeaType::Unknown,
        })
    }
}

/// Here we implement the FromStr trait for RmcStatus so that we can determine
/// the status of the GPRMC fix by calling parse()
impl FromStr for RmcStatus {
    type Err = u32;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(RmcStatus::Active),
            "V" => Ok(RmcStatus::Void),
            _   => Err(1),
        }
    }
}

/// Here we implement the FromStr trait for GpsFixQuality so that we can determine
/// the type of GPS fix we have by calling parse()
impl FromStr for GpsFixQuality {
    type Err = u32;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s.parse::<i32>() {
            Ok(0) => GpsFixQuality::InvalidFix,
            Ok(1) => GpsFixQuality::GPSFix,
            Ok(2) => GpsFixQuality::DGPSFix,
            Ok(4) => GpsFixQuality::RTKFixed,
            Ok(5) => GpsFixQuality::RTKFloat,
            _ => GpsFixQuality::Unknown,
        })
    }
}

/// Here we implement the FromStr trait for GpsTime so that we can determine
/// the time of the GPS fix by calling parse()
impl FromStr for GpsTime {
    type Err = u32;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        //Get the hours
        let h = match s.chars().take(2).collect::<String>().parse::<u8>() {
            Ok(c) => c,
            Err(_) => return Err(1),
        };

        //Get the minutes
        let m = match s.chars().skip(2).take(2).collect::<String>().parse::<u8>() {
            Ok(c) => c,
            Err(_) => return Err(2),
        };

        //Get the seconds
        let sec = match s.chars().skip(4).collect::<String>().parse::<f32>() {
            Ok(c) => c,
            Err(_) => return Err(3),
        };

        //Check if the values correspond to a valid time
        if h > 23 || m > 59 || sec >= 60.0 {
            return Err(4);
        }

        Ok(GpsTime {
            h: h,
            m: m,
            s: sec,
        })
    }
}

/// Here we implement the FromStr trait for GpsTime so that we can determine
/// the date of the GPS fix by calling parse()
impl FromStr for GpsDate {
    type Err = u32;
    fn from_str(s: &str) -> Result<Self, Self::Err> {

        //Get the day
        let day = match s.chars().take(2).collect::<String>().parse::<u8>() {
            Ok(c) => c,
            Err(_) => return Err(1),
        };

        //Get the month
        let mon = match s.chars().skip(2).take(2).collect::<String>().parse::<u8>() {
            Ok(c) => c,
            Err(_) => return Err(2),
        };

        //Get the year
        let year = match s.chars().skip(4).collect::<String>().parse::<u16>() {
            Ok(c) => c,
            Err(_) => return Err(3),
        };

        //Return the parsed date, and an error if the date is invalid
        GpsDate::new(mon, day, year + 1900).ok_or(4)
    }
}


impl FromNmea for GPGGA {
    fn from_nmea(nmea_string: &NMEA) -> GPGGA {

        //There should be exactly 14 fields in a GPGGA sentence. So count the commas to ensure this
        if nmea_count_fields(&nmea_string.sentence) != 14 {
            //Return a GPGGA struct with all None values
            return GPGGA::default();
        }

        //Also check the NMEA sentence NmeaType
        if nmea_string.sentence_type != NmeaType::GPGGA {
            return GPGGA::default();
        }

        let mut x = nmea_string.sentence.split(",").skip(1);

        /*
         * Since we already verified the number of commas in the GPGGA sentence,
         * all the nth() and next() calls in the rest of the function will never return None.
         * Therefore, none of the unwrap() calls will ever panic.
         */

        let time             = x.next().unwrap().parse::<GpsTime>().ok();
        let latitude         = parse_nmea_lat(x.next().unwrap(), x.next().unwrap());
        let longitude        = parse_nmea_lon(x.next().unwrap(), x.next().unwrap());
        let quality          = x.next().unwrap().parse::<GpsFixQuality>().ok();
        let num_satellites   = x.next().unwrap().parse::<u8>().ok();
        let hdop             = x.next().unwrap().parse::<f64>().ok();
        let altitude         = x.next().unwrap().parse::<f64>().ok();
        let geoid_seperation = x.nth(1).unwrap().parse::<f64>().ok();
        let age              = x.nth(1).unwrap().parse::<f64>().ok();
        let dstation_id      = x.next().unwrap().parse::<u16>().ok();

        GPGGA {
            time:             time,
            latitude:         latitude,
            longitude:        longitude,
            quality:          quality,
            num_satellites:   num_satellites,
            hdop:             hdop,
            altitude:         altitude,
            geoid_seperation: geoid_seperation,
            age:              age,
            dstation_id:      dstation_id,
        }
    }
}

impl FromNmea for GPZDA {
    fn from_nmea(nmea_string: &NMEA) -> GPZDA {

        //There should be exactly 6 fields in a GPZDA sentence. So count the commas to ensure this
        if nmea_count_fields(&nmea_string.sentence) != 6 {
            //Return a GPZDA struct with all None values
            return GPZDA::default();
        }

        //Also check the NMEA sentence NmeaType
        if nmea_string.sentence_type != NmeaType::GPZDA {
            return GPZDA::default();
        }

        let mut x = nmea_string.sentence.split(",").skip(1);

        /*
         * Since we already verified the number of commas in the GPZDA sentence,
         * all the next() calls in the rest of the function will never return None.
         * Therefore, none of the unwrap() calls will ever panic.
         */

        let time         = x.next().unwrap().parse::<GpsTime>().ok();
        let day          = x.next().unwrap().parse::<u8>().ok();
        let month        = x.next().unwrap().parse::<u8>().ok();
        let year         = x.next().unwrap().parse::<u16>().ok();
        let zone_hours   = x.next().unwrap().parse::<u8>().ok();
        let zone_minutes = x.next().unwrap().parse::<u8>().ok();

        let date = match (day, month, year) {
            (Some(day), Some(month), Some(year)) => GpsDate::new(month, day, year),
            _ => None,
        };

        GPZDA {
            time:         time,
            date:         date,
            zone_hours:   zone_hours,
            zone_minutes: zone_minutes,
        }
    }
}

impl FromNmea for GPRMC {
    fn from_nmea(nmea_string: &NMEA) -> GPRMC {

        //There should be at least 11 fields in a GPRMC sentence. So count the commas to ensure this
        if nmea_count_fields(&nmea_string.sentence) < 11 {
            //Return a GPRMC struct with all None values
            return GPRMC::default();
        }

        //Also check the NMEA sentence NmeaType
        if nmea_string.sentence_type != NmeaType::GPRMC {
            return GPRMC::default();
        }

        let mut x = nmea_string.sentence.split(",").skip(1);

        /*
         * Since we already verified the number of commas in the GPRMC sentence,
         * all the next() calls in the rest of the function will never return None.
         * Therefore, none of the unwrap() calls will ever panic.
         */

         let time       = x.next().unwrap().parse::<GpsTime>().ok();
         let rmc_status = x.next().unwrap().parse::<RmcStatus>().ok();
         let latitude   = parse_nmea_lat(x.next().unwrap(), x.next().unwrap());
         let longitude  = parse_nmea_lon(x.next().unwrap(), x.next().unwrap());
         let sog        = x.next().unwrap().parse::<f64>().ok();
         let cog        = x.next().unwrap().parse::<f64>().ok();
         let date       = x.next().unwrap().parse::<GpsDate>().ok();
         let magvar     = parse_nmea_magvar(x.next().unwrap(), x.next().unwrap());

         GPRMC {
             time:       time,
             rmc_status: rmc_status,
             latitude:   latitude,
             longitude:  longitude,
             sog:        sog,
             cog:        cog,
             date:       date,
             magvar:     magvar,
         }
    }
}


fn get_nmea_type(nmea: &str) -> NmeaType {
    //The unwrap() used here is completely safe and will never cause a panic
    //because the FromStr implemntation for <NmeaType> never returns an error
    nmea.chars()                    //Make an iterator of all the characters in the sentence
        .take_while(|&x| x != ',')  //Get all the characters up until the first ',' (comma)
        .collect::<String>()        //Collect those characters into a string
        .parse::<NmeaType>()        //Parse that string into an NmeaType
        .unwrap()
}

fn calc_nmea_checksum(nmea: &str) -> u8 {
    nmea.bytes().fold(0, |acc, x| acc ^ x)
}

fn parse_nmea_lat(latval: &str, dir: &str) -> Option<f64> {
    let degrees = latval.chars().take(2).collect::<String>().parse::<f64>().ok()?;
    let minutes = latval.chars().skip(2).collect::<String>().parse::<f64>().ok()?;

    //Convert latval from degrees and minutes to decimal degrees,
    //And take the North vs South of the equator into consideration
    match dir {
        "N" => Some((degrees + (minutes / 60.0))),
        "S" => Some(-1.0 * (degrees + minutes / 60.0)),
        _   => None,
    }
}

fn parse_nmea_lon(lonval: &str, dir: &str) -> Option<f64> {
    let degrees = lonval.chars().take(3).collect::<String>().parse::<f64>().ok()?;
    let minutes = lonval.chars().skip(3).collect::<String>().parse::<f64>().ok()?;

    //Convert lonval from degrees and minutes to decimal degrees,
    //And take the East vs West of the Prime Meridian into consideration
    match dir {
        "E" => Some((degrees + (minutes / 60.0))),
        "W" => Some(-1.0 * (degrees + minutes / 60.0)),
        _   => None,
    }
}

fn parse_nmea_magvar(magvar_val: &str, dir: &str) -> Option<f64> {
    let magvar = magvar_val.parse::<f64>().ok()?;
    match dir {
        "E" => Some(magvar),
        "W" => Some(-1.0 * magvar),
        _   => None,
    }
}

fn nmea_count_fields(s: &str) -> u32 {
    s.chars().fold(0, |acc, x| {
        match x {
            ',' => acc + 1,
            _   => acc,
        }
    })
}
