use std::fs;
pub struct CpuInfo {
    pub model_name: String,
    pub cpu_mhz: f64,
}
pub fn cpu_info() -> Option<Vec<CpuInfo>> {
    let data = fs::read_to_string("/proc/cpuinfo").ok()?;

    let blocks = data
        .split('\n')
        .filter(|elm| elm.starts_with("model name") || elm.starts_with("cpu MHz"))
        .map(|elm| elm.split(": ").nth(1))
        .collect::<Vec<Option<&str>>>();

    blocks
        .chunks(2)
        .map(|ck| -> Option<CpuInfo> {
            Some(CpuInfo {
                model_name: String::from(ck[0]?),
                cpu_mhz: ck[1]?.parse::<f64>().ok()?,
            })
        })
        .collect()
}