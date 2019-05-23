#![feature(lang_items)]
#![feature(core_intrinsics)]
#![feature(const_fn)]
#![feature(asm)]
#![feature(optin_builtin_traits)]
#![feature(decl_macro)]
#![feature(repr_align)]
#![feature(attr_literals)]
#![feature(never_type)]
#![feature(ptr_internals)]
#![feature(pointer_methods)]
#![feature(compiler_builtins_lib)]
#![no_builtins]

extern crate pi;
extern crate stack_vec;

pub mod lang_items;
pub mod mutex;
pub mod console;
pub mod shell;

use pi::timer::spin_sleep_ms;

const GPIO_BASE: usize = 0x3F000000 + 0x200000;

const GPIO_FSEL1: *mut u32 = (GPIO_BASE + 0x04) as *mut u32;
const GPIO_SET0: *mut u32 = (GPIO_BASE + 0x1C) as *mut u32;
const GPIO_CLR0: *mut u32 = (GPIO_BASE + 0x28) as *mut u32;

#[no_mangle]
pub extern "C" fn kmain() {
    unsafe {
        // STEP 1: Set GPIO Pin 16 as output.
        GPIO_FSEL1.write_volatile(1 << 18);
        // STEP 2: Continuously set and clear GPIO 16.
        loop {
            GPIO_SET0.write_volatile(1 << 16);
            spin_sleep_ms(256);
            GPIO_CLR0.write_volatile(1 << 16);
            spin_sleep_ms(256);
        }
    }
}
