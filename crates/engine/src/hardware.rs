//! Machine-aware defaults for thin, thermally limited laptops (the "Air edition").
//!
//! Two things are tuned, both overridable and both a no-op on machines that don't need them:
//!
//! - **Worker threads.** Hyper-threads add throughput but also heat; on a thin Intel laptop that
//!   throttles under sustained all-core load, one rayon worker per *physical* core keeps clocks
//!   steadier. Honours `RAYON_NUM_THREADS` / `LIGHTCRAFT_THREADS` when set. Only macOS reports the
//!   physical core count without native calls, so elsewhere nothing changes.
//! - **Memory budget.** See [`crate::memory::default_budget`], which uses [`budget_for_ram`].

/// Smallest worker pool we will ever ask for.
const MIN_THREADS: usize = 2;

/// Rayon worker count for a machine with `logical` hardware threads and `physical` cores:
/// one per physical core (at least [`MIN_THREADS`], never more than `logical`). `None` when the
/// default pool is already right (no hyper-threading, or the topology is unknown).
pub fn threads_for(logical: usize, physical: Option<usize>) -> Option<usize> {
    let physical = physical.filter(|p| *p > 0)?;
    if logical <= physical {
        return None;
    }
    Some(physical.max(MIN_THREADS).min(logical))
}

/// Default memory budget (bytes) for a machine with `ram` bytes of RAM: a quarter, at most
/// 1.5 GiB, but on machines with 8 GiB or less an eighth (1 GiB at 8 GiB), because the integrated
/// GPU, macOS and other apps share that memory. Never below 256 MiB.
pub fn budget_for_ram(ram: usize) -> usize {
    const GIB: usize = 1 << 30;
    let share = if ram <= 8 * GIB { ram / 8 } else { ram / 4 };
    share.clamp(256 << 20, 3 * GIB / 2)
}

/// Physical core count, where the platform reports it without native calls.
fn physical_cores() -> Option<usize> {
    #[cfg(target_os = "macos")]
    {
        let out = std::process::Command::new("/usr/sbin/sysctl").args(["-n", "hw.physicalcpu"]).output().ok()?;
        String::from_utf8_lossy(&out.stdout).trim().parse().ok()
    }
    #[cfg(not(target_os = "macos"))]
    {
        None
    }
}

fn env_threads() -> Option<usize> {
    ["LIGHTCRAFT_THREADS", "RAYON_NUM_THREADS"]
        .iter()
        .find_map(|k| std::env::var(k).ok().and_then(|v| v.trim().parse::<usize>().ok()).filter(|n| *n > 0))
}

/// Size the global rayon pool for this machine. Call once, first thing in `main`. Returns the
/// worker count when it changed the pool; an already-built pool or an unknown topology leaves it
/// alone (never an error: the default pool is always a valid fallback).
pub fn init_thread_pool() -> Option<usize> {
    let n = match env_threads() {
        Some(n) => n,
        None => {
            let logical = std::thread::available_parallelism().map_or(1, |n| n.get());
            threads_for(logical, physical_cores())?
        }
    };
    rayon::ThreadPoolBuilder::new().num_threads(n).build_global().ok().map(|()| n)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_worker_per_physical_core_when_hyperthreaded() {
        assert_eq!(threads_for(8, Some(4)), Some(4), "i5 Air: 4 cores / 8 threads");
        assert_eq!(threads_for(4, Some(2)), Some(2), "i3 Air: 2 cores / 4 threads");
    }

    #[test]
    fn default_pool_kept_when_nothing_to_gain_or_unknown() {
        assert_eq!(threads_for(4, Some(4)), None, "no hyper-threading");
        assert_eq!(threads_for(8, None), None, "unknown topology");
        assert_eq!(threads_for(8, Some(0)), None, "nonsense topology");
    }

    #[test]
    fn pool_never_below_two_nor_above_logical() {
        assert_eq!(threads_for(2, Some(1)), Some(2));
        assert_eq!(threads_for(1, Some(1)), None);
    }

    #[test]
    fn budget_tiers_by_ram() {
        const GIB: usize = 1 << 30;
        assert_eq!(budget_for_ram(8 * GIB), GIB, "8 GB Air");
        assert_eq!(budget_for_ram(4 * GIB), 512 << 20);
        assert_eq!(budget_for_ram(16 * GIB), 3 * GIB / 2, "16 GB keeps the 1.5 GiB cap");
        assert_eq!(budget_for_ram(32 * GIB), 3 * GIB / 2);
        assert_eq!(budget_for_ram(GIB), 256 << 20, "floor");
        assert_eq!(budget_for_ram(0), 256 << 20, "floor, no panic");
    }
}
