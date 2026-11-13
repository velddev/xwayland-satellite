# Overhead benchmarks

`tests/bench.rs` measures what satellite's X11 <-> Wayland paths cost, end to end, with a real
Xwayland and the testwl compositor. It was written to check whether three PR branches
(stable-primary, configure-notify, fullscreen-state) make satellite slower than upstream main
(b424e5e). It only uses harness APIs that exist on main, so the same file compiles on any branch.

## Running

Compare refs (builds a detached worktree and target dir per ref under `target/bench/`, copies
`tests/bench.rs` in, runs every tree's bench binary REPS times interleaved, then prints the
summary and writes it to `<outdir>/summary.md`):

    bench/run.sh                     # main twice (A/A) + the three upstream/* PR branches, 20 rounds
    bench/run.sh -n 20 -s 5 main=b424e5e main-again=b424e5e mine=HEAD
    bench/run.sh -f focus -n 5 main=b424e5e x=upstream/stable-primary   # only benchmarks matching "focus"

`-s N` also times the whole `cargo test --release` suite N times per tree. `LABEL=REF` names a
tree; give the baseline first. A second tree at the baseline's commit is an A/A control: its
spread is the noise floor the summary compares every change against. `bench/stats.py <outdir>`
re-renders a summary.

One tree by hand:

    cargo test --release --test bench -- --ignored --test-threads 1 --nocapture

`BENCH_RUNS` (default 5 measured runs after a warm-up, per process) and `BENCH_SCALE` (multiplies
every operation count; `0.05` for a smoke test) tune it. DISPLAY and WAYLAND_DISPLAY are not
needed; run.sh unsets them and RUST_LOG.

## What each benchmark does

Every run starts and ends on a full barrier, so the time covers everything satellite did for the
operations. Barriers are deterministic, never sleeps:

- X -> Wayland: set WM_NAME on a barrier window to a fresh marker, wait for testwl to get that
  title. Satellite handles X events in order.
- Wayland -> X: configure a second barrier toplevel to a new width, wait for the ConfigureNotify
  of satellite's resize. Satellite handles Wayland events in order.
- Full barrier: X -> Wayland, Wayland -> X, X -> Wayland.

Per run the bench prints, per operation, the wall time, the CPU time of the satellite thread,
the CPU time of Xwayland (all threads; from `/proc/*/schedstat`) and of the bench thread. CPU
times move much less than wall time when the rest of the machine is busy.

| benchmark | ops | what one operation is | mode |
|---|---|---|---|
| `configure_position` | 5000 | position-only ConfigureRequest on a mapped toplevel (satellite ignores the position) | all sent at once |
| `configure_position_sync` | 1000 | same, each followed by an X -> Wayland barrier | one at a time |
| `configure_position_size` | 1000 | position + size request, wait for the ConfigureNotify with the new size (like Wine) | one at a time |
| `fullscreen_toggle` | 2000 | `_NET_WM_STATE` add/remove fullscreen, alternating | all sent at once (barrier every 64) |
| `fullscreen_toggle_wait` | 500 | same, wait for `_NET_WM_STATE` to be rewritten (like Wine) | one at a time |
| `fullscreen_redundant` | 500 | add, add, remove, remove: half the requests don't change the state | each followed by X -> Wayland -> X barrier |
| `barrier_only` | 500 | that X -> Wayland -> X barrier alone (subtract it from `fullscreen_redundant`) | one at a time |
| `focus_m2/m50/m200` | 2000 | keyboard focus to the next of 2/50/200 mapped toplevels on one output, wait for satellite's WM_STATE write on it | one at a time |
| `focus_m3_u1000` | 2000 | same with 3 mapped toplevels and 1000 created-but-unmapped windows | one at a time |
| `output_enter_m3/m200` | 2000 | `wl_surface.enter` of the focused window, alternating between two outputs, with 3/200 mapped toplevels | all sent at once (barrier every 64) |
| `output_enter_m3_u1000` | 2000 | same with 1000 unmapped windows | all sent at once (barrier every 64) |
| `map_churn` | 200 | create + map a window, wait for its xdg_toplevel, configure it, destroy it, wait for the toplevel to go | one at a time |

Pipelining thousands of resizes (or Wayland events) at once is not possible: Xwayland's buffer
reallocations fill satellite's socket to testwl and satellite panics (`flush().unwrap()` on
WouldBlock in `dispatch.rs`) or deadlocks, on main too. That is why the size variant and the
Wayland-driven throughput benchmarks are paced.

## Results

### Final branch tips (second run, 2026-11-13)

`bench/run.sh -n 20 -s 5 main=b424e5e main-again=b424e5e stable-primary=17746c1
stable-primary-17c0827=17c0827 configure-notify=0ecb0fb fullscreen-state=fb0ed3b`. configure-notify
0ecb0fb is the pipelined version of the fix (one wait per ignored position request, SendEvent
cookies checked once per event batch); 17746c1 is a behaviour-neutral restyle of 17c0827. Full
tables: `bench/results/2026-11-13-final-tips/all/summary.md`.

Change of the median vs main, wall / satellite CPU (bold: whole 95% CI beyond the A/A floor and 2%):

| benchmark | ops | main: wall, sat CPU (us/op) | stable-primary 17746c1 | stable-primary 17c0827 | configure-notify 0ecb0fb | fullscreen-state fb0ed3b | A/A (main-again) |
|---|---|---|---|---|---|---|---|
| configure_position | 5000 | 12.1, 7.0 | -1.2% / +0.6% | +0.3% / +0.3% | **+53% / +33% slower** | -0.9% / +0.0% | +2.3% / +1.2% |
| configure_position_sync | 1000 | 44.1, 22.5 | -4.0% / -4.9% | +2.0% / -1.2% | +8.8% / +2.5% | -3.1% / -3.5% | -1.1% / -2.7% |
| configure_position_size | 1000 | 67.2, 23.1 | -5.7% / -3.9% | -1.2% / +2.3% | +13.4% / +14.8% (no panics) | -6.5% / -4.4% | -3.5% / +0.1% |
| fullscreen_toggle | 2000 | 26.0, 15.1 | +3.0% / +5.6% | +5.8% / +7.8% | +6.0% / +6.1% | **+39% / +37% slower** | +5.4% / +6.9% |
| fullscreen_toggle_wait | 500 | 39.8, 27.3 | -2.0% / -0.9% | -0.3% / +0.1% | -2.2% / -1.1% | **+29% / +20% slower** | -0.4% / +0.5% |
| fullscreen_redundant | 500 | 102.1, 57.0 | -1.3% / -1.0% | +5.1% / +3.7% | +2.3% / +2.2% | +10.7% / +8.7% | -3.5% / -3.3% |
| barrier_only | 500 | 78.4, 42.7 | -5.0% / -6.0% | +2.8% / +1.4% | +4.6% / +5.3% | +6.0% / +5.5% | -0.7% / -0.5% |
| focus_m2 | 2000 | 48.3, 24.3 | -10.8% / -7.7% | -14.1% / -8.1% | -11.3% / -8.9% | -18.6% / -14.3% | -0.6% / +1.2% |
| focus_m50 | 2000 | 43.1, 22.5 | -8.7% / -6.0% | +14.9% / +12.7% | +8.7% / +5.1% | -5.9% / -5.0% | +5.7% / +0.3% |
| focus_m200 | 2000 | 39.0, 21.1 | +12.2% / +9.7% | +11.4% / +10.3% | +2.1% / -0.9% | +5.6% / +2.2% | +6.8% / +3.8% |
| focus_m3_u1000 | 2000 | 44.9, 23.1 | -13.7% / -7.0% | -10.6% / -4.3% | -11.7% / -8.2% | -2.0% / +1.1% | -2.3% / -4.1% |
| output_enter_m3 | 2000 | 76.7, 37.1 | **-74% / -71% faster** | **-72% / -68% faster** | -4.3% / -5.0% | -15.2% / -12.5% | -14.9% / -13.6% |
| output_enter_m200 | 2000 | 71.9, 32.8 | **-72% / -67% faster** | **-74% / -67% faster** | -4.4% / +1.0% | -14.4% / -7.9% | -10.5% / -4.9% |
| output_enter_m3_u1000 | 2000 | 100.7, 41.6 | **-69% / -64% faster** | **-69% / -64% faster** | -7.0% / -9.6% | -6.5% / +0.5% | -5.9% / -12.8% |
| map_churn | 200 | 302.5, 164.7 | -1.9% / +0.6% | +2.1% / +2.2% | +1.2% / +1.1% | +2.4% / +0.4% | +2.0% / +0.7% |

Suite (`cargo test --release`, median of 5, s): main 10.46, main-again 10.54, 17746c1 11.39,
17c0827 11.27, 0ecb0fb 10.63, fb0ed3b 10.48 (differences follow the tests each branch adds).

- **configure-notify 0ecb0fb**: no panics in 20 rounds (c45fab6 panicked in 20 of 20).
  `configure_position_sync` (+8.8% wall, CI -3..+17%) and `configure_position_size` (+13% / +15%,
  CIs +5..+25% / +8..+24% against floors of 10% / 8%) are within the A/A floor. A cost remains only
  in the 5000-request burst: +6.4 us wall, +2.3 us satellite CPU, +4.0 us Xwayland CPU per ignored
  request (c45fab6: +31 / +14 / +18 us). That is the GetGeometry reply plus the SendEvent and its
  delivery, with no extra round trip; removing it would need the geometry from satellite's own
  state instead of GetGeometry. `configure_position_size` still pays for a synthetic event that X
  makes redundant (it already sends a real ConfigureNotify when the size changes).
- **stable-primary 17746c1 matches 17c0827**: compared directly
  (`bench/results/2026-11-13-final-tips/stable-primary-vs-17c0827/`), no benchmark differs beyond
  noise (largest: focus_m50 -21% / -17%, where main and main-again themselves differ by 8-13%;
  output_enter -6..+7%).
- **fullscreen-state fb0ed3b**: unchanged, same cost as in the first run (+10-11 us per
  `_NET_WM_STATE` change).
- **Panics outside configure-notify**: the pre-existing `flush().unwrap()` at
  `src/server/dispatch.rs:313` fired twice in `output_enter_m3`, once on 17746c1 and once on
  17c0827 (1 of 20 processes each; main 0 of 40 in this run). A dedicated 40-round rerun of the
  output_enter benchmarks (`bench/results/2026-11-13-final-tips/output-enter-panics/`) panicked 0
  of 40 times on both main and 17746c1, so the difference isn't significant. Its trigger is the
  bench's pacing (64 enters between testwl dispatches). The fix is the same: handle WouldBlock there.

### First run (2026-11-13, before the configure-notify fix)

Run on 2026-11-13: `bench/run.sh -n 20 -s 5 main=b424e5e main-again=b424e5e
stable-primary=17c0827 stable-primary-1st=d9cd02b configure-notify=c45fab6 fullscreen-state=fb0ed3b`
(20 interleaved rounds, rotating order; each round's sample is the median of 5 runs after a
warm-up). Full tables with p10/p90 and confidence intervals: `bench/results/2026-11-13/all/summary.md`.

Trees: `stable-primary` = 17c0827 (both commits, with `primary_pinned_by_others` and the
`_WINE_HWND_STYLE` read; velddev/stable-primary), `stable-primary-1st` = d9cd02b (its first commit
only, the local upstream/stable-primary at the time of the run), `configure-notify` = c45fab6 and
`fullscreen-state` = fb0ed3b (the branch tips at the time; their source differs from 1eefa1c and
dd4d424 only in comments).

Change of the median vs main, wall time / satellite CPU. **slower**/**faster** only when the
whole bootstrap 95% CI is beyond both the A/A noise floor (main vs main-again) and 2%.

| benchmark | ops | main: wall, sat CPU (us/op) | stable-primary | stable-primary-1st | configure-notify | fullscreen-state | A/A (main-again) |
|---|---|---|---|---|---|---|---|
| configure_position | 5000 | 12.4, 7.0 | -0.2% / -0.1% | -1.5% / -0.7% | **+251% / +203% slower** | -0.8% / -0.3% | -1.6% / +0.3% |
| configure_position_sync | 1000 | 45.3, 22.2 | -2.0% / -1.7% | +2.3% / +0.8% | **+47% / +44% slower** | -1.1% / -0.3% | +1.3% / +0.2% |
| configure_position_size | 1000 | 71.0, 23.8 | -4.3% / -2.4% | -7.0% / -2.4% | satellite panics (20/20) | -5.0% / -1.9% | -4.8% / -1.0% |
| fullscreen_toggle | 2000 | 27.1, 15.8 | +1.5% / +1.7% | +1.9% / +2.0% | +4.9% / +1.5% | **+41% / +36% slower** | +2.7% / +2.4% |
| fullscreen_toggle_wait | 500 | 40.3, 27.0 | +2.5% / +1.6% | +1.3% / +2.0% | +1.4% / +1.2% | **+28% / +22% slower** | +1.9% / +1.0% |
| fullscreen_redundant | 500 | 102.8, 56.8 | -1.2% / -1.0% | +1.8% / +3.0% | +1.7% / -0.6% | +13.6% / +11.5% (CI +8..+27%, floor 13%) | -2.6% / -5.9% |
| barrier_only | 500 | 83.2, 45.3 | -8.2% / -10.9% | -4.8% / -4.7% | +0.5% / +0.3% | -0.6% / -4.7% | -3.6% / -4.4% |
| focus_m2 | 2000 | 41.8, 21.2 | +8.5% / +8.1% | +5.1% / +6.3% | +19.8% / +16.0% | +8.8% / +11.4% | -6.9% / -2.7% |
| focus_m50 | 2000 | 42.1, 21.5 | +5.6% / +8.1% | +3.6% / +2.2% | +8.3% / +4.6% | -2.7% / -1.2% | +5.7% / +5.7% |
| focus_m200 | 2000 | 41.1, 21.6 | +2.4% / +4.5% | -4.5% / -3.4% | +18.0% / +11.5% | +5.9% / +4.4% | +1.5% / +0.5% |
| focus_m3_u1000 | 2000 | 38.1, 20.6 | +13.5% / +9.7% | +4.5% / +1.7% | +14.0% / +3.4% | +22.6% / +12.1% | +19.0% / +12.2% |
| output_enter_m3 | 2000 | 67.9, 32.1 | **-70% / -67% faster** | **-71% / -68% faster** | -5.0% / -2.7% | +17.0% / +19.7% | +3.0% / +1.7% |
| output_enter_m200 | 2000 | 69.4, 32.4 | **-74% / -69% faster** | **-72% / -69% faster** | -5.2% / -4.6% | -4.9% / -4.2% | -2.0% / -3.2% |
| output_enter_m3_u1000 | 2000 | 98.6, 39.7 | **-70% / -65% faster** | **-72% / -69% faster** | -9.6% / -8.9% | -17.1% / -15.1% | -5.5% / -7.4% |
| map_churn | 200 | 303.9, 165.8 | +7.1% / +3.9% | +3.7% / +0.5% | +4.0% / +1.5% | +1.4% / -0.4% | +2.8% / +0.5% |

Full `cargo test --release` suite, 5 runs per tree (median s): main 10.10, main-again 10.16,
stable-primary 11.19, stable-primary-1st 10.34, configure-notify 10.60, fullscreen-state 10.85.
The differences track the integration tests each branch adds (with their sleeps), not runtime cost.

### Noise (first run)

Wall time across rounds varies a lot on this machine (other agents were compiling Rust in
parallel; load average 4-10): p10-p90 spans are ±10-30% of the median for the latency-bound
benchmarks. Satellite CPU time is steadier. The A/A noise floor (how far the main vs
main-again 95% CI reaches) was 6-11% wall / 3-7% satellite CPU for `configure_position*` and the
fullscreen benchmarks, 8-13% for the barrier-bound ones, and 14-31% for `focus_*`. A second, focus-only run (20 rounds, main /
main-again / stable-primary, `bench/results/2026-11-13/focus/`) again found no difference in
`focus_*` (all within ±9%, floors 14-32%): the stable-primary window scan is below the ~3 us/op
these benchmarks can resolve.

### Findings (first run)

**configure-notify** is the one real cost. Every position request on a mapped toplevel now takes
three synchronous round trips to Xwayland instead of one: the (often empty) ConfigureWindow,
`GetGeometry`, and `send_and_check_request(SendEvent)`. That is +31 us wall, +14 us satellite CPU
and +18 us Xwayland CPU per request when they come in a burst, +21 us per request answered one at
a time. Because satellite now lags a client that resizes as soon as it sees the real
ConfigureNotify, `configure_position_size` makes Xwayland's buffer reallocations reach satellite in
bursts, its socket to testwl fills, and satellite panics on the pre-existing
`state.queue.flush().unwrap()` in `wl_shm_pool.destroy` (src/server/dispatch.rs:313) - 20 of 20
rounds; with an X -> Wayland barrier after every request it passes. Main never hit it.

Fix (prototype in `bench/proto-configure-notify-pipelined.patch`, local branch
`bench/proto-configure-notify`): send the GetGeometry right behind the ConfigureWindow and wait
once (the reply also completes the ConfigureWindow check), and send the SendEvent checked but check
it after the event batch instead of waiting for it. That is one round trip per request, like main.
Measured (8 rounds, `bench/results/2026-11-13/cn-pipelined/`): configure_position +20% wall /
+7% satellite CPU (not significant), configure_position_sync +0.6% / -0.9%, configure_position_size
+15% / +11% with no panics (configure-notify: 7 of 8 rounds panicked, the other ran 10x slower).
Separately, dispatch.rs:313 should handle WouldBlock like `ServerState::run` does instead of
unwrapping. Optionally skip the synthetic event when the request also changes the size, since X
then sends a real ConfigureNotify anyway.

**fullscreen-state** adds one GetProperty round trip to each `_NET_WM_STATE` write: +11 us wall,
+5.7 us satellite CPU, +6.5 us Xwayland CPU per fullscreen change, and writes for redundant
requests too (+14 us per request in `fullscreen_redundant`, at the edge of the noise floor).
Fullscreen changes happen a few times per session, so this is irrelevant in practice. If wanted:
check the ChangeProperty lazily (`send_request_checked`, checked later) so the read-modify-write
costs one round trip like the old blind write, or cache the client's `_NET_WM_STATE` from
PropertyNotify.

**stable-primary**: no measurable cost. Focus changes with 2-200 mapped windows or 1000 unmapped
ones are within noise; map/unmap churn (one extra GetProperty for `_WINE_HWND_STYLE` among the ~8
property reads at map) is +4-7% wall / +0.5-4% CPU, within noise. Output enters of the focused
window are ~70% cheaper, because the branch no longer re-focuses the window and moves the RandR
primary on every enter (a behaviour change, not an optimisation). d9cd02b alone behaves the same.

### What this does not cover

- testwl is not a real compositor: it configures instantly, renders nothing, sends no frame
  callbacks and never refuses `set_fullscreen` (main's testwl can't; `fullscreen_redundant`
  exercises the same "answer anyway" path through redundant requests instead).
- No real Wine: no window has `_WINE_HWND_STYLE`, so `primary_pinned_by_others` always scans every
  window (its worst case) and the pinned path is never taken. Wine's own reaction to the synthetic
  ConfigureNotify or the extra `_NET_WM_STATE` writes is not modelled.
- Windows are never painted; Xwayland only allocates shm buffers on resize. No GPU, no dmabuf.
- One X client. On main, every primary-output change sends RRScreenChangeNotify to every client;
  real sessions have many.
- Satellite runs as a thread of the test process, with logging at error level.
- Throughput numbers are means per run; per-operation latency distributions are not recorded.
- Memory use is not measured.

### Machine

AMD Ryzen 9 7950X (16C/32T), amd-pstate-epp, governor powersave, EPP balance_performance; kernel
7.3.1-4-pond; Xwayland 24.1.14 (xorg-xwayland-pond, with Pond's buffer-attach retry patch); rustc
1.99.1; release profile. Mike's desktop session (niri, Discord, Spotify, Steam) and other agents'
cargo builds were running; nothing was pinned.
