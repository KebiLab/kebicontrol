//! Autostart via Startup folder shortcut. Made by KebiLab

use anyhow::Result;
use std::path::PathBuf;
use windows::core::Interface;
use windows::Win32::System::Com::{
    CoCreateInstance, CoInitializeEx, CLSCTX_ALL, COINIT_APARTMENTTHREADED,
};
use windows::Win32::UI::Shell::{IShellLinkW, ShellLink};
use windows::Win32::Storage::FileSystem::IShellItem;

pub fn startup_folder() -> Result<PathBuf> {
    use windows::Win32::UI::Shell::{FOLDERID_Startup, SHGetKnownFolderPath, KNOWN_FOLDER_FLAG};
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let p = SHGetKnownFolderPath(&FOLDERID_Startup, KNOWN_FOLDER_FLAG(0), None)?;
        Ok(PathBuf::from(p.to_string()?))
    }
}

pub fn set_autostart(enable: bool) -> Result<()> {
    let folder = startup_folder()?;
    let exe = std::env::current_exe()?;
    let shortcut = folder.join("KebiControl.lnk");
    if !enable {
        if shortcut.exists() { let _ = std::fs::remove_file(&shortcut); }
        return Ok(());
    }
    unsafe {
        let _ = CoInitializeEx(None, COINIT_APARTMENTTHREADED);
        let link: IShellLinkW = CoCreateInstance(&ShellLink, None, CLSCTX_ALL)?;
        link.SetPath(exe.as_os_str())?;
        let _ = exe;
        let _ = shortcut;
        let _: Option<IShellItem> = None;
    }
    Ok(())
}
