read_csr_as_usize!(0x244);
write_csr_as_usize!(0x244);

pub const VSTIP: usize = 0b1 << 5;

pub fn set_vstimer() {
    let vsip = read();
    write(vsip | (VSTIP))
}

pub fn clear_vstimer() {
    let vsip = read();
    write(vsip & !(VSTIP))
}