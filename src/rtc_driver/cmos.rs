use crate::io::outb::outb;
use crate::io::inb::inb;

const CMOS_REGISTER_PORT : u16 = 0x70;
const CMOS_REGISTER_CHOOSE : u16 = 0x71;


pub unsafe fn read_register (register: u8) -> u8 {
    outb(CMOS_REGISTER_PORT, register );
    inb(CMOS_REGISTER_CHOOSE)

}