use pelite::pattern;
use pelite::{
    pattern::Atom,
    pe::{Pe, PeView},
};
use windows::core::PCSTR;
use windows::Win32::System::Memory::PAGE_PROTECTION_FLAGS;

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

pub fn panic_hook(panic_info: &std::panic::PanicHookInfo) {
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
        windows::Win32::UI::WindowsAndMessaging::MessageBoxW(
            None,
            windows::core::PCWSTR(message_utf16.as_ptr()),
            windows::core::PCWSTR(title_utf16.as_ptr()),
            windows::Win32::UI::WindowsAndMessaging::MB_ICONERROR,
        );
    }
    std::process::abort();
}

fn get_module(module_name: &[u8]) -> PeView {
    let module = unsafe {
        windows::Win32::System::LibraryLoader::GetModuleHandleA(PCSTR(module_name.as_ptr() as _))
    };

    match module {
        Ok(module) => return unsafe { PeView::module(module.0 as *const u8) },
        Err(_) => panic!("Failed to get module handle"),
    }
}
#[no_mangle]
#[allow(non_snake_case)]
fn Patch() {
    // apply panic hook
    std::panic::set_hook(Box::new(panic_hook));

    let mut matches = [0; 2];

    let view = get_module(b"GameAssembly.dll\0");

    if !view.scanner().finds_code(PATTERN, &mut matches) {
        panic!("Failed to find pattern in GameAssembly.dll");
    }
    let va = view
        .rva_to_va(matches[1])
        .expect("Failed to convert rva to va");
    let addr: *mut u8 = unsafe { std::mem::transmute(va) };
    unsafe {
        let mut old_protect = PAGE_PROTECTION_FLAGS::default();
        windows::Win32::System::Memory::VirtualProtect(
            addr as _,
            2,
            windows::Win32::System::Memory::PAGE_READWRITE,
            &mut old_protect,
        )
        .unwrap();
        *addr = 0x90;
        *addr.add(1) = 0x90;
        windows::Win32::System::Memory::VirtualProtect(addr as _, 2, old_protect, &mut old_protect)
            .unwrap();
    }
}
