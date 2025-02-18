pub mod cpuid;

extern "C" {
    pub fn asm_halt() -> !;
    pub fn asm_inb(port: u16) -> u8;
    pub fn asm_outb(port: u16, val: u8);
}
