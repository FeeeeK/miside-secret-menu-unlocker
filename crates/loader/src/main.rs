use dll_syringe::{process::OwnedProcess, Syringe};
use secretmenu_native::mod_panic_hook;
use std::ffi::CString;
use tracing::info;
use tracing_subscriber;
use windows::core::{PCSTR, PSTR};
use windows::Win32::System::Threading::{
    CreateProcessA, CREATE_SUSPENDED, PROCESS_INFORMATION, STARTUPINFOA,
};

fn main() {
    std::fs::remove_file("SecretMenuLoader.log").ok();
    let appender = tracing_appender::rolling::never("./", "SecretMenuLoader.log");
    tracing_subscriber::fmt().with_writer(appender).init();
    info!("Loader started");
    std::panic::set_hook(Box::new(mod_panic_hook));

    let exe_path = "MiSideFull.exe";
    if !std::path::Path::new(exe_path).exists() {
        panic!("MiSideFull.exe not found, please copy the loader to the game directory");
    }
    let exe_path_cstr = CString::new(exe_path).expect("CString::new failed");

    let dll_path = "SecretMenu/secretmenu_native.dll";
    if !std::path::Path::new(dll_path).exists() {
        panic!("SecretMenu/secretmenu_native.dll not found, please, copy mod folder to the game directory");
    }

    let mut startup_info = STARTUPINFOA::default();
    let mut process_info = PROCESS_INFORMATION::default();

    unsafe {
        CreateProcessA(
            PCSTR(exe_path_cstr.as_ptr() as *const u8),
            PSTR::null(),
            None,
            None,
            false,
            CREATE_SUSPENDED,
            None,
            None,
            &mut startup_info,
            &mut process_info,
        )
    }
    .unwrap();

    let process = OwnedProcess::find_first_by_name("MiSideFull").expect("Failed to find process");
    let syringe = Syringe::for_process(process);

    syringe.inject(dll_path).expect("Failed to inject DLL");

    unsafe { windows::Win32::System::Threading::ResumeThread(process_info.hThread) };

    info!("DLL injected successfully");
}
