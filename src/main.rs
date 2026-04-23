use sysinfo::System;
use active_win_pos_rs::get_active_window;

#[cfg(windows)]
use winapi::um::tlhelp32::{CreateToolhelp32Snapshot, Module32First, Module32Next, MODULEENTRY32, TH32CS_SNAPMODULE};
#[cfg(windows)]
use winapi::um::handleapi::{CloseHandle, INVALID_HANDLE_VALUE};
#[cfg(windows)]
use std::mem;

const RED_TARGETS: &[(&str, &str)] = &[
    ("solara", "SOLARA"), ("xeno", "XENO"), ("wave", "WAVE"), ("synapse", "SYNAPSE"),
    ("synapse z", "SYNAPSE_Z"), ("potassium", "POTASSIUM"), ("volt", "VOLT"),
    ("seliware", "SELIWARE"), ("cosmic", "COSMIC"), ("isaeva", "ISAEVA"),
    ("volcano", "VOLCANO"), ("velocity", "VELOCITY"), ("bunni", "BUNNI"),
    ("bunni.fun", "BUNNI_FUN"), ("sirhurt", "SIRHURT"), ("krnl", "KRNL"),
    ("fluxus", "FLUXUS"), ("delta", "DELTA"), ("vega x", "VEGA_X"),
    ("vega", "VEGA_X"), ("codex", "CODEX"), ("hydrogen", "HYDROGEN"),
    ("arceus x", "ARCEUS_X"), ("arceus", "ARCEUS_X"), ("nihon", "NIHON"),
    ("comet", "COMET"), ("evon", "EVON"), ("electron", "ELECTRON"),
    ("scriptware", "SCRIPTWARE"), ("krampus", "KRAMPUS"), ("lunar", "LUNAR"),
    ("carbon", "CARBON"), ("domain", "DOMAIN"), ("furk", "FURK"),
    ("trigon", "TRIGON"), ("zenith", "ZENITH"), ("viper", "VIPER"),
    ("nexus", "NEXUS"), ("jjsploit", "JJSPLOIT"), ("kiwi x", "KIWI_X"),
    ("kiwi", "KIWI_X"), ("celery", "CELERY"), ("sygil", "SYGIL"),
    ("macsploit", "MACSPLOIT"), ("opiumware", "OPIUMWARE"), ("cryptic", "CRYPTIC"),
];

const YELLOW_TARGETS: &[(&str, &str)] = &[
    ("bloxstrap", "BLOXSTRAP"), ("fishtrap", "FISHTRAP"), ("froststrap", "FROSTSTRAP"),
    ("voidstrap", "VOIDSTRAP"), ("complexity", "COMPLEXITY"), ("luczystrap", "LUCZYSTRAP"),
    ("chevsblox", "CHEVSBLOX"), ("serotonin", "SEROTONIN"), ("severe", "SEVERE"),
    ("rbxcii", "RBXCII"), ("ronin", "RONIN"), ("matcha", "MATCHA"),
    ("matrix hub", "MATRIX_HUB"), ("photon", "PHOTON"), ("dx9ware", "DX9WARE"),
    ("dx9ware v2", "DX9WARE_V2"), ("cheatengine", "CHEAT_ENGINE"),
    ("processhacker", "PROCESS_HACKER"), ("ollydbg", "OLLYDBG"), ("x64dbg", "X64DBG"),
];

const BLACKLISTED_DLLS: &[&str] = &[
    "krnl.dll", "fluxus.dll", "solara.dll", "xeno.dll", "synapse.dll", "wave.dll",
    "potassium.dll", "volt.dll", "seliware.dll", "cosmic.dll", "isaeva.dll",
    "volcano.dll", "velocity.dll", "bunni.dll", "sirhurt.dll", "hydrogen.dll",
    "delta.dll", "vega.dll", "codex.dll", "arceus.dll", "nihon.dll", "comet.dll",
    "evon.dll", "electron.dll", "scriptware.dll", "krampus.dll", "lunar.dll",
    "carbon.dll", "domain.dll", "macsploit.dll", "opiumware.dll", "cryptic.dll",
    "celery.dll", "sygil.dll", "kiwi.dll", "jjsploit.dll", "furk.dll", "trigon.dll",
    "zenith.dll", "viper.dll", "nexus.dll",
];

#[cfg(windows)]
fn find_roblox_pid() -> Option<u32> {
    let mut system = System::new_all();
    system.refresh_all();
    for (pid, process) in system.processes() {
        if process.name().to_lowercase().contains("robloxplayerbeta") {
            return Some(pid.as_u32());
        }
    }
    None
}

#[cfg(windows)]
fn enumerate_process_modules(pid: u32) -> Vec<String> {
    let mut modules = Vec::new();
    unsafe {
        let snapshot = CreateToolhelp32Snapshot(TH32CS_SNAPMODULE, pid);
        if snapshot == INVALID_HANDLE_VALUE { return modules; }
        let mut me32: MODULEENTRY32 = mem::zeroed();
        me32.dwSize = mem::size_of::<MODULEENTRY32>() as u32;
        if Module32First(snapshot, &mut me32) != 0 {
            loop {
                let wide_name_ptr = me32.szModule.as_ptr() as *const u16;
                let len = (0..260).take_while(|&i| *wide_name_ptr.add(i) != 0).count();
                let wide_slice = std::slice::from_raw_parts(wide_name_ptr, len);
                modules.push(String::from_utf16_lossy(wide_slice).to_lowercase());
                if Module32Next(snapshot, &mut me32) == 0 { break; }
            }
        }
        CloseHandle(snapshot);
    }
    modules
}

fn main() {
    println!("Sentinel Scanner (open-source edition)");
    println!("Scanning every 5 seconds. Press Ctrl+C to stop.\n");

    loop {
        let mut system = System::new_all();
        system.refresh_all();

        for process in system.processes().values() {
            let name = process.name().to_lowercase();
            for (target, code) in RED_TARGETS {
                if name.contains(target) {
                    println!("[RED] Process {} detected ({})", code, process.name());
                }
            }
            for (target, code) in YELLOW_TARGETS {
                if name.contains(target) {
                    println!("[YELLOW] Process {} detected ({})", code, process.name());
                }
            }
        }

        if let Ok(window) = get_active_window() {
            let title = window.title.to_lowercase();
            for (target, code) in RED_TARGETS {
                if title.contains(target) {
                    println!("[RED] Window {} detected (\"{}\")", code, window.title);
                }
            }
        }

        #[cfg(windows)]
        {
            use std::time::{Instant, Duration};
            static mut LAST_DLL: Option<Instant> = None;
            unsafe {
                if LAST_DLL.is_none() || LAST_DLL.unwrap().elapsed() > Duration::from_secs(10) {
                    if let Some(pid) = find_roblox_pid() {
                        let modules = enumerate_process_modules(pid);
                        for dll in BLACKLISTED_DLLS {
                            if modules.iter().any(|m| m.contains(dll)) {
                                println!("[RED] DLL {} detected in Roblox", dll);
                            }
                        }
                    }
                    LAST_DLL = Some(Instant::now());
                }
            }
        }

        std::thread::sleep(std::time::Duration::from_secs(5));
    }
}
