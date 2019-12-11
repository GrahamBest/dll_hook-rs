/*
    Rust code explaining how direct function detours work
*/ 

#![feature(const_fn)]
#![feature(libc)]
 
extern crate winapi;
extern crate kernel32;
#[macro_use] extern crate detour;
#[macro_use] extern crate lazy_static;
extern crate libc;

use detour::*;
use winapi::{HWND, LPCSTR, UINT, c_int};

type createmove_fn = fn(f32, *mut UserCmd) -> bool;

struct UserCmd {
    /* dscode here */
}

struct FunctionPtrAddress {
    addy: createmove_fn
}

lazy_static! {
    static ref fn_ptrs: FunctionPtrAddress = FunctionPtrAddress {
        addy: unsafe {
            std::mem::transmute::<usize, createmove_fn>(0x10111790)
        }
    };
}

static_detours! {
    struct CreateMoveDetour: fn(f32, *mut UserCmd) -> bool;
}

// entry point
#[no_mangle]
#[allow(non_snake_case, unused_variables)]
pub extern "system" fn DllMain(
    dll_module: winapi::HINSTANCE,
    call_reason: winapi::DWORD,
    reserved: winapi::LPVOID)
    -> winapi::BOOL
{
    const DLL_PROCESS_ATTACH: winapi::DWORD = 1;
    const DLL_PROCESS_DETACH: winapi::DWORD = 0;

    match call_reason {
        DLL_PROCESS_ATTACH => init(),
        DLL_PROCESS_DETACH => (),
        _ => ()
    }

    return winapi::TRUE;
}
    
// init
fn init() {
    unsafe { 
        kernel32::AllocConsole() 
    };

    println!("Initializing...");

    let closure_for_createmove = |input_sample_time, cmd| {
        println!("heres the detour. put your code in here");
    
        return (fn_ptrs.addy)(input_sample_time, cmd);
    };  

    let mut hook = unsafe { 
        CreateMoveDetour.initialize(createmove_hook, closure_for_createmove).unwrap() 
    };

    unsafe {
        hook.enable().unwrap();
    }

    createmove_hook(1.0, std::ptr::null_mut()); // call this so hook.call works
    hook.call(100.0, std::ptr::null_mut());
}

fn createmove_hook(input_sample_time: f32, cmd: *mut UserCmd) -> bool {
    println!("original function");

    return (fn_ptrs.addy)(input_sample_time, cmd);
}
