use tom_arch::riscv;
use tom_timer::Ticker;
const TIME_BASE: u64 = 1_500_000_000;
static TICKER: Vf2Ticker = Vf2Ticker {
    time_base: TIME_BASE,
};

pub static TICKER_REF: &'static dyn Ticker = &TICKER;

pub struct Vf2Ticker {
    time_base: u64,
}

impl Ticker for Vf2Ticker {
    #[inline]
    fn get_tick(&self) -> u64 {
        riscv::mcycle::read() as u64
    }
    #[inline]
    fn tick_to_nanos(&self, tick: u64) -> core::time::Duration {
        riscv::tick_to_nanos(tick, self.time_base)
    }
    #[inline]
    fn tick_to_micros(&self, tick: u64) -> core::time::Duration {
        riscv::tick_to_micros(tick, self.time_base)
    }
    #[inline]
    fn tick_to_millis(&self, tick: u64) -> core::time::Duration {
        riscv::tick_to_millis(tick, self.time_base)
    }
    #[inline]
    fn tick_to_secs(&self, tick: u64) -> core::time::Duration {
        riscv::tick_to_secs(tick, self.time_base)
    }
    #[inline]
    fn nanos_to_tick(&self, nanos: u64) -> u64 {
        riscv::nanos_to_tick(nanos, self.time_base)
    }
    #[inline]
    fn micros_to_tick(&self, micros: u64) -> u64 {
        riscv::micros_to_tick(micros, self.time_base)
    }
    #[inline]
    fn millis_to_tick(&self, millis: u64) -> u64 {
        riscv::millis_to_tick(millis, self.time_base)
    }
    #[inline]
    fn secs_to_tick(&self, secs: u64) -> u64 {
        riscv::secs_to_tick(secs, self.time_base)
    }
}
