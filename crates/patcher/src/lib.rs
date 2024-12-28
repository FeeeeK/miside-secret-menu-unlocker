use std::thread::{self, yield_now};
use std::time::{Duration, Instant};

use pelite::pattern;
use pelite::{
    pattern::Atom,
    pe::{Pe, PeView},
};
use tracing::{error, info};
use tracing_panic::panic_hook;
use tracing_subscriber;
use windows::core::{PCSTR, PCWSTR};
use windows::Win32::System::LibraryLoader::GetModuleHandleA;
use windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS;
use windows::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR};

const PATTERN: &[Atom] = pattern!(
    "
    e8 ? ? ? ?
    83 f8 64
    ' 75 ?
    33 d2
    48 8b cb
    e8 ? ? ? ?
    "
);

pub fn mod_panic_hook(panic_info: &std::panic::PanicHookInfo) {
    panic_hook(panic_info);
    let message;
    let title = "MiSide secret menu unlocker error";
    let reason = panic_info.payload().downcast_ref::<&str>();
    let github_issues_url = "https://github.com/feeeek/miside-secret-menu-unlocker/issues";
    message = format!(
        "A panic occurred\nReason: {}\n\nPlease report this to the developer: {}",
        reason.map_or("Unknown", |v| v),
        github_issues_url,
    );

    let mut message_utf16: Vec<u16> = message.encode_utf16().collect();
    message_utf16.push(0);
    let mut title_utf16: Vec<u16> = title.encode_utf16().collect();
    title_utf16.push(0);

    unsafe {
        MessageBoxW(
            None,
            PCWSTR(message_utf16.as_ptr()),
            PCWSTR(title_utf16.as_ptr()),
            MB_ICONERROR,
        );
    }
    std::process::abort();
}

fn wait_for_module(module_name: &[u8]) -> PeView {
    let start = Instant::now();

    let timeout = Duration::from_secs(10);
    let sleep_duration = Duration::from_millis(50);

    loop {
        match unsafe { GetModuleHandleA(PCSTR(module_name.as_ptr() as _)) } {
            Ok(module) => {
                info!("Module {:?} found", module_name);
                return unsafe { PeView::module(module.0 as *const u8) };
            }
            Err(_) => {
                error!(
                    "Failed to get module handle for {:?}, retrying...",
                    module_name
                );
            }
        }
        if start.elapsed() >= timeout {
            panic!("Timeout: 10 seconds elapsed while waiting for singleton instance");
        }
        yield_now();
        thread::sleep(sleep_duration);
    }
}

#[no_mangle]
#[allow(non_snake_case)]
fn Patch() {
    let mut matches = [0; 2];

    let view = wait_for_module(b"GameAssembly.dll\0");

    if !view.scanner().finds_code(PATTERN, &mut matches) {
        panic!("Failed to find pattern in GameAssembly.dll");
    }
    info!("Pattern found at offsets: {:?}", matches);

    let va = view
        .rva_to_va(matches[1])
        .expect("Failed to convert rva to va");
    info!("Virtual address: {:x}", va);

    let addr = va as *mut u8;
    unsafe {
        let mut old_protect = PAGE_PROTECTION_FLAGS::default();
        windows::Win32::System::Memory::VirtualProtect(
            addr as _,
            2,
            windows::Win32::System::Memory::PAGE_READWRITE,
            &mut old_protect,
        )
        .unwrap();
        info!("Memory protection changed to PAGE_READWRITE");

        std::ptr::write(addr, 0x90);
        std::ptr::write(addr.add(1), 0x90);
        info!("Memory patched at address: {:x}", addr as usize);

        windows::Win32::System::Memory::VirtualProtect(addr as _, 2, old_protect, &mut old_protect)
            .unwrap();
        info!("Memory protection restored");
    }
}

#[no_mangle]
#[allow(non_snake_case)]
unsafe extern "C-unwind" fn DllMain(_hinst: usize, reason: u32, _reserved: usize) -> bool {
    match reason {
        1 => {
            std::fs::remove_file("SecretMenu.log").ok();
            let appender = tracing_appender::rolling::never("./", "SecretMenu.log");
            tracing_subscriber::fmt().with_writer(appender).init();

            info!("DLL loaded, spawning patch thread");
            std::thread::spawn(|| Patch());
        }
        _ => {}
    }
    true
}
