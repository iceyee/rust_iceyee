// **************************************************
// *  Author: Iceyee                                *
// *  Mail: iceyee.studio@qq.com                    *
// *  Git: https://github.com/iceyee                *
// **************************************************
//
#![allow(clippy::needless_return)]

/* Use. */

use std::cell::Cell;
use std::time::SystemTime;
use std::time::UNIX_EPOCH;

thread_local! {
    /* 已经是const初始化; clippy 1.98 的missing_const_for_thread_local对const块会误报, 故忽略. */
    #[allow(clippy::missing_const_for_thread_local)]
    static SEED: Cell<u64> = const { Cell::new(0) };
}

/* 计数器步长, 黄金分割常数. */
const GOLDEN_RATIO: u64 = 0x9E37_79B9_7F4A_7C15;

/* 兜底种子, 极端情况下才会用到. */
const DEFAULT_SEED: u64 = 0x2545_F491_4F6C_DD1D;

/* Enum. */

/* Trait. */

/* Struct. */

/// 随机数.
///
/// 种子是线程变量.
///
/// 内部状态是一个64位计数器, 每取一次递增一个黄金分割常数, 因此周期是2^64;
/// 对外输出再把状态过一遍splitmix64混淆函数, 保证雪崩效应与低位质量.
pub struct Random;

impl Random {
    /// 设置种子, 种子是线程变量.
    ///
    /// 参数为0时, 用当前线程id作为种子.
    pub fn set_seed(s: u64) {
        let s: u64 = if s == 0 { get_thread_id() } else { s };
        SEED.with(|seed| seed.set(s));
        return;
    }

    /// 下一个随机数.
    pub fn next() -> u64 {
        let state: u64 = SEED.with(|seed| {
            let mut state: u64 = seed.get();
            if state == 0 {
                /* 没有设置过种子, 用线程id与时间戳初始化. */
                state = get_thread_id() ^ get_time_nanos();
                if state == 0 {
                    state = DEFAULT_SEED;
                }
            }
            state = state.wrapping_add(GOLDEN_RATIO);
            seed.set(state);
            return state;
        });
        /* splitmix64混淆函数. */
        let mut z: u64 = state;
        z = (z ^ (z >> 30)).wrapping_mul(0xBF58476D1CE4E5B9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94D049BB133111EB);
        return z ^ (z >> 31);
    }

    /// 下一个小于max的随机数, 相当于next() % max.
    ///
    /// max为0时返回0.
    pub fn next_less_than(max: u64) -> u64 {
        if max == 0 {
            return 0;
        }
        return Self::next() % max;
    }
}

/* Function. */

/* 取当前时间戳, 单位纳秒. */
fn get_time_nanos() -> u64 {
    return SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_or(0, |duration| duration.as_nanos() as u64);
}

/* 取线程id. */
fn get_thread_id() -> u64 {
    #[cfg(target_os = "linux")]
    {
        /* type pthread_t = long unsigned int. */
        /* pthread_t pthread_self(void); */
        use std::ffi::c_ulong;
        unsafe extern "C" {
            fn pthread_self() -> c_ulong;
        }
        return unsafe { pthread_self() } as u64;
    }
    #[cfg(target_os = "windows")]
    {
        /* DWORD GetCurrentThreadId(); */
        unsafe extern "C" {
            fn GetCurrentThreadId() -> u32;
        }
        return unsafe { GetCurrentThreadId() } as u64;
    }
    #[cfg(not(any(target_os = "linux", target_os = "windows")))]
    {
        /* 只考虑linux与windows. */
        compile_error!("iceyee_random 只支持 linux 和 windows.")
    }
}
