// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
// Use.

use std::cell::Cell;

thread_local! {
    static SEED: Cell<u64> = Cell::new(0);
}

// Enum.

// Trait.

// Struct.

/// 随机数.

#[derive(Debug)]
pub struct Random;

impl Random {
    /// 设置种子, 种子是线程局部变量.
    pub fn set_seed(s: u64) {
        SEED.with(|seed| seed.set(s));
        return;
    }

    /// 取一个随机数.
    pub fn next() -> usize {
        let mut seed: u64 = SEED.with(|seed_2| {
            if seed_2.get() == 0 {
                // 如果是0(未初始化), 则执行初始化.
                init();
            }
            seed_2.get()
        });
        const TABLE: [u64; 32] = [
            0x59763CEA1457DFC5,
            0x0E701C6631DCCC01,
            0x67793534A8C778D7,
            0x64770DE4EB0B80CE,
            0x747A9AF32BB93809,
            0xC71D4F13A1E22A09,
            0xE74813B3E6C1ECAB,
            0x3B88A9B5A97C3862,
            0x24057CC3128D9579,
            0xD41D3C71A9426A59,
            0xE5E61B92D1F676F4,
            0xA01EC8F62398DE1C,
            0x3025BB78C7E3DD78,
            0xD869150399B67D2A,
            0xD4F8F70CEEFBB738,
            0xF910F138AFE1C1C9,
            0xE63F12FA3125DB84,
            0x84DC6ADA95196F95,
            0x6C47E3124371EF67,
            0x3339D137640D3929,
            0x6604916E4DF3C5AF,
            0x8D86EBD5FD2374CD,
            0xAD13E8135162601F,
            0xDA5C5215F3867431,
            0x1540D632794D96C6,
            0x012533A3629D7DE8,
            0x238F9B1046DDCA4C,
            0xB61FAC20EBE58CEA,
            0xF6982B86164D089B,
            0xEAA86C9587A038B3,
            0xF4B1F470ABDA3652,
            0x10A7C3C4A75EF01E,
        ];
        for x in 0..16 {
            seed = (seed << 63) | (seed >> 1);
            seed ^= TABLE[x];
            seed = ((seed as u128) + (TABLE[x + 16] as u128)) as u64;
        }
        SEED.with(|seed_2| seed_2.set(seed));
        return seed as usize;
    }
}

// Function.

fn init() {
    use std::time::SystemTime;
    let id: u64 = get_thread_id() as u64;
    let time_2: u64 = SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;
    let seed: u64 = id & 0xFFFF;
    let seed: u64 = (seed << 48) | (seed << 32) | (seed << 16) | (seed << 0);
    let seed: u64 = seed ^ time_2;
    SEED.with(|seed_2| seed_2.set(seed));
    return;
}

// 线程id.
fn get_thread_id() -> i64 {
    #[cfg(target_os = "linux")]
    unsafe {
        // pthread_t pthread_self(void);
        // type pthread_t = long unsigned int.
        use std::ffi::c_ulong;
        extern "C" {
            fn pthread_self() -> c_ulong;
        }
        return pthread_self() as i64;
    }
    #[cfg(target_os = "windows")]
    unsafe {
        // DWORD GetCurrentThreadId();
        use std::ffi::c_ulong;
        extern "C" {
            fn GetCurrentThreadId() -> c_ulong;
        }
        return GetCurrentThreadId() as i64;
    }
}
