use sysinfo::{ProcessesToUpdate, ProcessRefreshKind, System, UpdateKind}; 

pub fn get_lcu_token()-> Option<(u16, String)> {
    let mut sys = System::new();
    
    sys.refresh_processes_specifics(
        ProcessesToUpdate::All,
        true,
        ProcessRefreshKind::nothing().with_cmd(UpdateKind::Always)
    );
    
    for process in sys.processes().values() {
        if process.name().to_string_lossy() != "LeagueClientUx.exe" {
            continue;
        }
        
        let mut port: Option<u16> = None;
        let mut token: Option<String> = None; 
        for arg in process.cmd() {
            let arg = arg.to_string_lossy();

            if let Some(v) = arg.strip_prefix("--app-port=") {
                port = v.parse::<u16>().ok();
            } else if let Some(v) = arg.strip_prefix("--remoting-auth-token=") {
                token = Some(v.to_string())
            }
        }
        if let (Some(p), Some(t)) = (port, token) {
            return Some((p, t));
        }
    }

    None
}