use std::io;

pub struct GpuInfo {
    pub name: String,
}

pub fn gpu_info() -> io::Result<Option<GpuInfo>> {
    let output = std::process::Command::new("bash")
        .arg("-c")
        .arg("lspci -nn | grep -i VGA | awk -F ']' '{print $2}' | tr -d ':' | tr -d '['")
        .output()?;
    let data = String::from_utf8(output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
    if !data.trim().is_empty() {
        Ok(Some(GpuInfo { name: data.trim().to_string() }))
    } else {
        Ok(None)
    }
}
