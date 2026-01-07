use winreg::enums::HKEY_CURRENT_USER;
use winreg::RegKey;

const INTERNET_SETTINGS: &str = r"Software\Microsoft\Windows\CurrentVersion\Internet Settings";

pub fn set_proxy(enable: bool, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    let (key, _) = hkcu.create_subkey(INTERNET_SETTINGS)?;

    if enable {
        key.set_value("ProxyEnable", &1u32)?;
        key.set_value("ProxyServer", &format!("127.0.0.1:{}", port))?;
    } else {
        key.set_value("ProxyEnable", &0u32)?;
    }

    // 通知系统代理设置已更改
    notify_proxy_change();

    Ok(())
}

pub fn is_enabled() -> bool {
    let hkcu = RegKey::predef(HKEY_CURRENT_USER);
    if let Ok(key) = hkcu.open_subkey(INTERNET_SETTINGS) {
        if let Ok(value) = key.get_value::<u32, _>("ProxyEnable") {
            return value == 1;
        }
    }
    false
}

fn notify_proxy_change() {
    #[cfg(windows)]
    {
        use windows::Win32::Networking::WinInet::{
            InternetSetOptionW, INTERNET_OPTION_REFRESH, INTERNET_OPTION_SETTINGS_CHANGED,
        };
        unsafe {
            // 通知 IE/系统 代理设置已更改
            let _ = InternetSetOptionW(
                None,
                INTERNET_OPTION_SETTINGS_CHANGED,
                Some(std::ptr::null() as *const std::ffi::c_void),
                0,
            );
            let _ = InternetSetOptionW(
                None,
                INTERNET_OPTION_REFRESH,
                Some(std::ptr::null() as *const std::ffi::c_void),
                0,
            );
        }
    }
}
