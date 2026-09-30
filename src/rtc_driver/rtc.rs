use crate::rtc_driver::cmos::read_register;

const SECOND_BYTE : u8 = 0x00;
const MINUTE_BYTE : u8 = 0x02;
const HOUR_BYTE : u8 = 0x04;
const MONTH_BYTE : u8 = 0x08;
const YEAR_BYTE : u8 = 0x09;

fn bcd_to_binary(bcd: u8) -> u8 {
    ((bcd & 0xF0) >> 4) * 10 + (bcd & 0x0F)
}

pub unsafe fn get_second() -> u8 { 
   bcd_to_binary(read_register(SECOND_BYTE))
}

pub unsafe fn get_minute() -> u8 { 
    bcd_to_binary(read_register(MINUTE_BYTE))
}

pub unsafe fn get_hour() -> u8 {
    bcd_to_binary(read_register(HOUR_BYTE))
}

pub unsafe fn get_year() -> u8 { 
    bcd_to_binary(read_register(YEAR_BYTE))
}

pub unsafe fn get_month() -> u8 { 
    bcd_to_binary(read_register(MONTH_BYTE))
}