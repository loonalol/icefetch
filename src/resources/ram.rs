use bytesize::ByteSize;
use std::collections::HashMap;
use std::fs;
pub struct MemInfo<T> {
    pub total: T,
    pub avail: T,
    pub cached: T,
    pub buffers: T,
    pub used: T,
}

pub fn mem_info() -> Option<MemInfo<ByteSize>> {
    let data = fs::read_to_string("/proc/meminfo").ok()?;
    let mem = data
        .split('\n')
        .map(|kv| kv.split_whitespace().take(2).collect::<Vec<&str>>())
        .filter(|elm| !elm.is_empty())
        .map(|elm| -> (String, Option<u64>) {
            let mut key = elm[0].to_string();
            key.pop();
            let val = elm[1].parse::<u64>().ok();
            (key, val)
        })
        .collect::<HashMap<String, Option<u64>>>();

    let total = mem["MemTotal"];
    let avail = mem["MemAvailable"];
    let cached = mem["Cached"];
    let buffers = mem["Buffers"];
    let used = total? - avail?;

    Some(MemInfo {
        total: ByteSize::kb(total?),
        avail: ByteSize::kb(avail?),
        cached: ByteSize::kb(cached?),
        buffers: ByteSize::kb(buffers?),
        used: ByteSize::kb(used),
    })
}