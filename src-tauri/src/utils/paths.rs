use std::env;
use std::path::PathBuf;

/// 获取 NetProxy 数据目录
pub fn get_data_dir() -> PathBuf {
    if cfg!(debug_assertions) {
        // 开发环境：使用 src-tauri/resources 目录
        PathBuf::from(r"D:\workspaces\CodeProjects\windows\NetProxy\src-tauri\resources\NetProxy")
    } else {
        // 生产环境：使用 exe 同级的 NetProxy 目录
        env::current_exe()
            .unwrap_or_default()
            .parent()
            .unwrap_or(&PathBuf::new())
            .join("NetProxy")
    }
}

/// 获取 xray.exe 路径
pub fn get_xray_path() -> PathBuf {
    get_data_dir().join("bin").join("xray.exe")
}

/// 获取 proxylink.exe 路径
pub fn get_proxylink_path() -> PathBuf {
    get_data_dir()
        .join("tools")
        .join("proxylink")
        .join("proxylink.exe")
}

/// 获取 confdir 目录
pub fn get_confdir() -> PathBuf {
    get_data_dir().join("config").join("xray").join("confdir")
}

/// 获取 outbounds 目录
pub fn get_outbounds_dir() -> PathBuf {
    get_data_dir().join("config").join("xray").join("outbounds")
}
