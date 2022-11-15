//! Rust wrappers around the x86-family I/O instructions.

use core::arch::asm; // Rust 1.59.0

/// Read a `u8`-sized value from `port`.
pub unsafe fn inb(port: u16) -> u8 {
    // The registers for the `in` and `out` instructions are always the
    // same: `a` for value, and `d` for the port address.
    let result: u8;
    // llvm_asm!("inb %dx, %al" : "={al}"(result) : "{dx}"(port) :: "volatile");
    asm!(
        "inb {dx:x}, {al}",
        dx = in(reg) port,
        al = out(reg_byte) result
    );
    result
}

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outb(value: u8, port: u16) {
    // llvm_asm!("outb %al, %dx" :: "{dx}"(port), "{al}"(value) :: "volatile");
    asm!(
        "outb {al}, {dx:x}",
        al = in(reg_byte) value,
        dx = in(reg) port
    );
}

/// Read a `u16`-sized value from `port`.
pub unsafe fn inw(port: u16) -> u16 {
    let result: u16;
    // llvm_asm!("inw %dx, %ax" : "={ax}"(result) : "{dx}"(port) :: "volatile");
    asm!(
        "inw {dx:x}, {ax:x}",
        dx = in(reg) port,
        ax = out(reg) result
    );
    result
}

/// Write a `u8`-sized `value` to `port`.
pub unsafe fn outw(value: u16, port: u16) {
    // llvm_asm!("outw %ax, %dx" :: "{dx}"(port), "{ax}"(value) :: "volatile");
    asm!(
        "outw {ax:x}, {dx:x}",
        ax = in(reg) value,
        dx = in(reg) port
    );
}

/// Read a `u32`-sized value from `port`.
pub unsafe fn inl(port: u16) -> u32 {
    let result: u32;
    // llvm_asm!("inl %dx, %eax" : "={eax}"(result) : "{dx}"(port) :: "volatile");
    asm!(
        "inl {dx:x}, {eax:e}",
        dx  = in(reg) port,
        eax = out(reg) result
    );
    result
}

/// Write a `u32`-sized `value` to `port`.
pub unsafe fn outl(value: u32, port: u16) {
    // llvm_asm!("outl %eax, %dx" :: "{dx}"(port), "{eax}"(value) :: "volatile");
    asm!(
        "outl {eax:e}, {dx:x}",
        eax = in(reg) value,
        dx  = in(reg) port
    );
}
