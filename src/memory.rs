use std::arch::asm;
use winapi::ctypes::c_void;

extern "stdcall" {
    fn HeapCreate(options: u32, initial_size: usize, max_size: usize) -> *mut std::ffi::c_void;
    fn HeapAlloc(heap: *mut std::ffi::c_void, flags: u32, bytes: usize) -> *mut std::ffi::c_void;
    fn GetLastError() -> u32;
}

pub unsafe fn asm_heap_create(options: u32, initial_size: usize, max_size: usize) -> *mut std::ffi::c_void {
    let heap: *mut std::ffi::c_void;
    asm!(
    "
        mov rcx, {0}
        mov rdx, {1}
        mov r8, {2}
        sub rsp, 40
        call {3}
        add rsp, 40
        ",
    in(reg) options,
    in(reg) initial_size,
    in(reg) max_size,
    in(reg) HeapCreate,
    out("rax") heap,
    options(nostack)
    );
    heap
}

pub unsafe fn asm_heap_alloc(heap: *mut std::ffi::c_void, flags: u32, bytes: usize) -> *mut std::ffi::c_void {
    let memory: *mut std::ffi::c_void;
    asm!(
    "
        mov rcx, {0}
        mov rdx, {1}
        mov r8, {2}
        sub rsp, 40
        call {3}
        add rsp, 40
        ",
    in(reg) heap,
    in(reg) flags,
    in(reg) bytes,
    in(reg) HeapAlloc,
    out("rax") memory,
    options(nostack)
    );
    memory
}
