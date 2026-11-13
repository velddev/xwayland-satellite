//! Overhead benchmarks for satellite's X11 <-> Wayland event paths.
//!
//! Every benchmark is `#[ignore]`d so `cargo test` skips them. Run them with
//!
//!     cargo test --release --test bench -- --ignored --test-threads 1 --nocapture
//!
//! or compare git refs with `bench/run.sh` (see BENCH.md).
//!
//! The file only uses harness APIs that exist on upstream main (b424e5e): `xwls::main`,
//! `xwls::RunData` and testwl. It keeps its own copy of the fixture so it compiles unchanged on any
//! branch, whatever that branch does to tests/integration.rs.
//!
//! Timing is end to end through a real Xwayland and the testwl compositor running in this process.
//! No benchmark sleeps: each one either waits for an effect it can observe after every operation
//! (latency mode) or sends all operations and then waits on a barrier (throughput mode):
//!
//! * X -> Wayland barrier: set WM_NAME on a barrier window to a fresh marker and wait for testwl to
//!   receive that title. Satellite handles X events in order, so every X event queued before the
//!   PropertyNotify has been handled and every Wayland request it caused has reached testwl.
//! * Wayland -> X barrier: configure a barrier toplevel to a new width and wait for the X
//!   ConfigureNotify satellite's resize produces. Satellite handles Wayland events in order, so
//!   every event sent before it has been handled and its X requests have been processed.
//!
//! A full barrier is X -> Wayland, Wayland -> X, X -> Wayland.
//!
//! Output, one line per measured run and one line of per-column medians per benchmark, all per
//! operation:
//!
//!     BENCH\t<name>\t<ops>\t<run>\t<wall ns>\t<satellite CPU ns>\t<Xwayland CPU ns>\t<bench thread CPU ns>\t<satellite timeslices>
//!     BENCHMED\t<name>\t<ops>\t<same five columns>
//!
//! The CPU times come from /proc/.../schedstat, so they barely move when other processes compete
//! for the CPU, unlike the wall time.

use rustix::event::{PollFd, PollFlags, poll};
use rustix::process::{Pid, Signal, WaitOptions};
use std::collections::HashMap;
use std::io::Write;
use std::mem::ManuallyDrop;
use std::os::fd::{AsRawFd, BorrowedFd, OwnedFd};
use std::os::unix::net::UnixStream;
use std::sync::{
    Arc, Mutex, Once,
    atomic::{AtomicBool, Ordering},
};
use std::thread::JoinHandle;
use std::time::{Duration, Instant};
use wayland_protocols::xdg::shell::server::xdg_toplevel;
use wayland_server::Resource;
use wayland_server::protocol::wl_output::WlOutput;
use xcb::{Xid, x};
use xwayland_satellite as xwls;
use xwls::timespec_from_millis;

/// Measured runs per benchmark (after one warm-up run). `BENCH_RUNS` overrides it.
fn runs() -> usize {
    std::env::var("BENCH_RUNS")
        .ok()
        .and_then(|r| r.parse().ok())
        .unwrap_or(5)
}

/// Scales every operation count (`BENCH_SCALE=0.1` for a quick smoke test).
fn ops(n: usize) -> usize {
    let scale: f64 = std::env::var("BENCH_SCALE")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(1.0);
    ((n as f64 * scale) as usize).max(4)
}

// ---------------------------------------------------------------------------------------------
// Fixture (copied from tests/integration.rs on main, minus the asserts and with quiet logging)
// ---------------------------------------------------------------------------------------------

#[derive(Default)]
struct TestDataInner {
    server_created: AtomicBool,
    server_connected: AtomicBool,
    display: Mutex<Option<String>>,
    server: Mutex<Option<UnixStream>>,
    pid: Mutex<Option<u32>>,
    quit_rx: Mutex<Option<UnixStream>>,
}

#[derive(Default, Clone)]
struct TestData(Arc<TestDataInner>);

impl TestData {
    fn new(server: UnixStream, quit_rx: UnixStream) -> Self {
        Self(Arc::new(TestDataInner {
            server: Mutex::new(server.into()),
            quit_rx: Mutex::new(Some(quit_rx)),
            ..Default::default()
        }))
    }
}

impl std::ops::Deref for TestData {
    type Target = Arc<TestDataInner>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl xwls::RunData for TestData {
    fn created_server(&self) {
        self.server_created.store(true, Ordering::Relaxed);
    }

    fn connected_server(&self) {
        self.server_connected.store(true, Ordering::Relaxed);
    }

    fn quit_rx(&self) -> Option<UnixStream> {
        self.quit_rx.lock().unwrap().take()
    }

    fn xwayland_ready(&self, display: String, pid: u32) {
        *self.display.lock().unwrap() = Some(display);
        *self.pid.lock().unwrap() = Some(pid);
    }

    fn display(&self) -> Option<&str> {
        None
    }

    fn listenfds(&mut self) -> Vec<OwnedFd> {
        Vec::new()
    }

    fn server(&self) -> Option<UnixStream> {
        self.server.lock().unwrap().take()
    }
}

const SATELLITE_THREAD: &str = "xwls-satellite";

/// CPU time (ns) and number of timeslices from a schedstat file; zeros if the task is gone.
fn schedstat(path: &str) -> (u64, u64) {
    let Ok(stat) = std::fs::read_to_string(path) else {
        return (0, 0);
    };
    let fields: Vec<u64> = stat
        .split_whitespace()
        .map(|f| f.parse().unwrap())
        .collect();
    (fields[0], fields[2])
}

/// `schedstat` summed over every thread of process `pid`.
fn process_schedstat(pid: i32) -> (u64, u64) {
    let mut total = (0, 0);
    for task in std::fs::read_dir(format!("/proc/{pid}/task")).unwrap() {
        let task = task.unwrap().path().join("schedstat");
        let (cpu, slices) = schedstat(task.to_str().unwrap());
        total.0 += cpu;
        total.1 += slices;
    }
    total
}

/// The satellite thread's schedstat file.
fn satellite_schedstat_path() -> String {
    for task in std::fs::read_dir("/proc/self/task").unwrap() {
        let task = task.unwrap().path();
        let comm = std::fs::read_to_string(task.join("comm")).unwrap_or_default();
        if comm.trim() == SATELLITE_THREAD {
            return task.join("schedstat").to_str().unwrap().to_owned();
        }
    }
    panic!("satellite thread not found");
}

struct Fixture {
    testwl: testwl::Server,
    thread: ManuallyDrop<JoinHandle<Option<()>>>,
    pollfd: PollFd<'static>,
    display: String,
    pid: Pid,
    quit_tx: UnixStream,
}

impl Drop for Fixture {
    fn drop(&mut self) {
        if std::thread::panicking() {
            // Satellite may be stuck waiting on Xwayland; kill it first so the join below returns.
            let _ = rustix::process::kill_process(self.pid, Signal::KILL);
        }
        let thread = unsafe { ManuallyDrop::take(&mut self.thread) };
        if self
            .quit_tx
            .write_all(&0_i32.to_ne_bytes())
            .is_err_and(|e| e.kind() != std::io::ErrorKind::BrokenPipe)
        {
            panic!("could not message the main thread to terminate");
        }
        if thread.join().is_err() {
            log::error!("main thread panicked");
        }
        if rustix::process::test_kill_process(self.pid).is_ok() {
            rustix::process::kill_process(self.pid, Signal::TERM).unwrap();
            rustix::process::waitpid(Some(self.pid), WaitOptions::NOHANG).unwrap();
        }
    }
}

impl Fixture {
    fn new() -> Self {
        static INIT: Once = Once::new();
        INIT.call_once(|| {
            // Debug logging would dominate every timing (and the branches add debug! lines), so
            // only errors are printed unless RUST_LOG asks for more.
            pretty_env_logger::formatted_timed_builder()
                .is_test(true)
                .filter_level(log::LevelFilter::Error)
                .parse_default_env()
                .init();
        });

        let (quit_tx, quit_rx) = UnixStream::pair().unwrap();
        let (a, b) = UnixStream::pair().unwrap();
        let mut testwl = testwl::Server::new(false);
        testwl.connect(a);
        let our_data = TestData::new(b, quit_rx);
        let data = our_data.clone();
        let thread = std::thread::Builder::new()
            .name(SATELLITE_THREAD.into())
            .spawn(move || xwls::main(data))
            .unwrap();

        let fd = unsafe { BorrowedFd::borrow_raw(testwl.poll_fd().as_raw_fd()) };
        let pollfd = PollFd::from_borrowed_fd(fd, PollFlags::IN);
        let timeout = timespec_from_millis(5000);
        assert!(poll(&mut [pollfd.clone()], Some(&timeout)).unwrap() > 0);
        testwl.dispatch();

        let wait_bool = |b: &AtomicBool, what: &str| {
            let start = Instant::now();
            while !b.load(Ordering::Relaxed) {
                assert!(start.elapsed() < Duration::from_secs(5), "{what}");
                std::thread::yield_now();
            }
        };
        wait_bool(&our_data.server_created, "creating server");
        wait_bool(&our_data.server_connected, "connecting to server");

        let mut f = [pollfd.clone()];
        let start = Instant::now();
        let mut ready = our_data.display.lock().unwrap().is_some();
        while !ready && start.elapsed() < Duration::from_secs(20) {
            let timeout = timespec_from_millis(100);
            if poll(&mut f, Some(&timeout)).unwrap() > 0 {
                testwl.dispatch();
            }
            ready = our_data.display.lock().unwrap().is_some();
        }
        assert!(ready, "connecting to xwayland failed");

        let display = our_data.display.lock().unwrap().take().unwrap();
        let pid = our_data.pid.lock().unwrap().take().unwrap();
        Self {
            testwl,
            thread: ManuallyDrop::new(thread),
            pollfd,
            display,
            pid: Pid::from_raw(pid as _).expect("Xwayland PID was invalid?"),
            quit_tx,
        }
    }
}

xcb::atoms_struct! {
    struct Atoms {
        net_wm_state => b"_NET_WM_STATE" only_if_exists = false,
        fullscreen => b"_NET_WM_STATE_FULLSCREEN" only_if_exists = false,
        wm_state => b"WM_STATE" only_if_exists = false,
    }
}

#[derive(Clone, Copy, Debug)]
struct Win {
    window: x::Window,
    surface: testwl::SurfaceId,
}

// ---------------------------------------------------------------------------------------------
// Bench harness
// ---------------------------------------------------------------------------------------------

struct Bench {
    f: Fixture,
    conn: xcb::Connection,
    xfd: PollFd<'static>,
    atoms: Atoms,
    root: x::Window,
    visual: u32,
    /// PropertyNotify events seen per (window, property) and not consumed yet.
    props: HashMap<(x::Window, x::Atom), u32>,
    /// Last ConfigureNotify width seen per window.
    widths: HashMap<x::Window, u16>,
    marker: u32,
    /// [0] carries the X -> Wayland title barrier, [1] the Wayland -> X configure barrier.
    /// Empty only while `new` creates them.
    barrier: Vec<Win>,
    barrier_width: u16,
    outputs: Vec<WlOutput>,
}

const WIN_SIZE: u16 = 100;

impl Bench {
    /// A satellite with `outputs` (x, y positions) and the two mapped barrier toplevels, which
    /// enter the first output if there is one.
    fn new(outputs: &[(i32, i32)]) -> Self {
        let f = Fixture::new();
        let (conn, _) = xcb::Connection::connect(Some(&f.display)).unwrap();
        let xfd = unsafe { BorrowedFd::borrow_raw(conn.as_raw_fd()) };
        let xfd = PollFd::from_borrowed_fd(xfd, PollFlags::IN);
        let atoms = Atoms::intern_all(&conn).unwrap();
        let screen = conn.get_setup().roots().next().unwrap();
        let root = screen.root();
        let visual = screen.root_visual();
        let mut b = Self {
            f,
            conn,
            xfd,
            atoms,
            root,
            visual,
            props: HashMap::new(),
            widths: HashMap::new(),
            marker: 0,
            barrier: Vec::new(),
            barrier_width: WIN_SIZE,
            outputs: Vec::new(),
        };
        for &(x, y) in outputs {
            b.f.testwl.new_output();
            b.settle();
            let output = b.f.testwl.finalize_output(x, y);
            b.settle();
            b.outputs.push(output);
        }
        let output = b.outputs.first().cloned();
        let b0 = b.map_toplevel(output.as_ref());
        let b1 = b.map_toplevel(output.as_ref());
        b.barrier = vec![b0, b1];
        b.full_barrier();
        b
    }

    /// Dispatch until neither side has had anything to say for 50 ms. Setup only.
    fn settle(&mut self) {
        let timeout = timespec_from_millis(50);
        loop {
            self.f.testwl.dispatch();
            self.drain_x();
            self.conn.flush().unwrap();
            let mut fds = [self.f.pollfd.clone(), self.xfd.clone()];
            if poll(&mut fds, Some(&timeout)).unwrap() == 0 {
                break;
            }
        }
    }

    fn drain_x(&mut self) {
        loop {
            match self.conn.poll_for_event() {
                Ok(Some(xcb::Event::X(x::Event::PropertyNotify(e)))) => {
                    *self.props.entry((e.window(), e.atom())).or_default() += 1;
                }
                Ok(Some(xcb::Event::X(x::Event::ConfigureNotify(e)))) => {
                    self.widths.insert(e.window(), e.width());
                }
                Ok(Some(_)) => {}
                Ok(None) => break,
                Err(e) => panic!("X error: {e:?}"),
            }
        }
    }

    /// Flush our X requests and handle whatever is ready on either side, without waiting.
    /// Throughput benchmarks call this every few operations so testwl keeps reading satellite's
    /// requests (satellite panics if its Wayland socket fills up).
    fn pump(&mut self) {
        self.conn.flush().unwrap();
        self.f.testwl.dispatch();
        self.drain_x();
    }

    /// Pump both connections until `done` holds.
    #[track_caller]
    fn pump_until(&mut self, what: &str, mut done: impl FnMut(&mut Self) -> bool) {
        let start = Instant::now();
        let timeout = timespec_from_millis(100);
        loop {
            self.f.testwl.dispatch();
            self.drain_x();
            if done(self) {
                return;
            }
            assert!(
                start.elapsed() < Duration::from_secs(20),
                "timed out waiting for {what}"
            );
            self.conn.flush().unwrap();
            let mut fds = [self.f.pollfd.clone(), self.xfd.clone()];
            poll(&mut fds, Some(&timeout)).unwrap();
        }
    }

    /// Wait for one PropertyNotify of `atom` on `window`.
    #[track_caller]
    fn wait_property(&mut self, window: x::Window, atom: x::Atom) {
        self.pump_until("PropertyNotify", |b| {
            match b.props.get_mut(&(window, atom)) {
                Some(n) if *n > 0 => {
                    *n -= 1;
                    true
                }
                _ => false,
            }
        });
    }

    fn create_window(&self) -> x::Window {
        let wid = self.conn.generate_id();
        self.conn
            .send_and_check_request(&x::CreateWindow {
                depth: 0,
                wid,
                parent: self.root,
                x: 0,
                y: 0,
                width: WIN_SIZE,
                height: WIN_SIZE,
                border_width: 0,
                class: x::WindowClass::InputOutput,
                visual: self.visual,
                value_list: &[x::Cw::EventMask(
                    x::EventMask::STRUCTURE_NOTIFY | x::EventMask::PROPERTY_CHANGE,
                )],
            })
            .expect("creating window failed");
        wid
    }

    fn alive_toplevel(&self, surface: testwl::SurfaceId) -> bool {
        self.f.testwl.get_surface_data(surface).is_some_and(
            |d| matches!(&d.role, Some(testwl::SurfaceRole::Toplevel(t)) if t.toplevel.is_alive()),
        )
    }

    /// Map a new window and wait until satellite has given it an xdg_toplevel, then configure it
    /// (and send it into `output`).
    fn map_toplevel(&mut self, output: Option<&WlOutput>) -> Win {
        let window = self.create_window();
        let window = self.map_existing(window);
        if let Some(output) = output {
            self.f.testwl.move_surface_to_output(window.surface, output);
        }
        window
    }

    fn map_existing(&mut self, window: x::Window) -> Win {
        let before = self.f.testwl.created_surfaces().len();
        self.conn
            .send_and_check_request(&x::MapWindow { window })
            .unwrap();
        let mut surface = None;
        self.pump_until("toplevel role", |b| {
            surface = b.f.testwl.created_surfaces()[before..]
                .iter()
                .copied()
                .find(|s| b.alive_toplevel(*s));
            surface.is_some()
        });
        let surface = surface.unwrap();
        self.f.testwl.configure_toplevel(
            surface,
            WIN_SIZE as _,
            WIN_SIZE as _,
            vec![xdg_toplevel::State::Activated],
        );
        Win { window, surface }
    }

    /// X -> Wayland barrier.
    fn x_to_wl(&mut self) {
        self.marker += 1;
        let marker = format!("m{}", self.marker);
        let b = self.barrier[0];
        self.conn.send_request(&x::ChangeProperty {
            mode: x::PropMode::Replace,
            window: b.window,
            property: x::ATOM_WM_NAME,
            r#type: x::ATOM_STRING,
            data: marker.as_bytes(),
        });
        self.conn.flush().unwrap();
        self.pump_until("title marker", |bench| {
            bench.f.testwl.get_surface_data(b.surface).is_some_and(|d| {
                matches!(&d.role, Some(testwl::SurfaceRole::Toplevel(t))
                         if t.title.as_deref() == Some(marker.as_str()))
            })
        });
    }

    /// Wayland -> X barrier.
    fn wl_to_x(&mut self) {
        self.barrier_width = if self.barrier_width == WIN_SIZE {
            WIN_SIZE + 1
        } else {
            WIN_SIZE
        };
        let width = self.barrier_width;
        let b = self.barrier[1];
        self.f
            .testwl
            .configure_toplevel(b.surface, width as _, WIN_SIZE as _, vec![]);
        self.pump_until("barrier ConfigureNotify", |bench| {
            bench.widths.get(&b.window) == Some(&width)
        });
    }

    fn full_barrier(&mut self) {
        self.x_to_wl();
        self.wl_to_x();
        self.x_to_wl();
    }

    /// Runs `op` (which does `n` operations) once to warm up and then `runs()` times. Every run
    /// starts and ends on a full barrier. Per run it prints, per operation: wall time, CPU time
    /// of the satellite thread, CPU time of Xwayland (all threads), CPU time of this thread
    /// (testwl and the X client), all in ns, and satellite timeslices (wakeups).
    fn measure(&mut self, name: &str, n: usize, mut op: impl FnMut(&mut Self, usize)) {
        let sat_path = satellite_schedstat_path();
        let xwl_pid = self.f.pid.as_raw_nonzero().get();
        let mut results: Vec<[f64; 5]> = Vec::new();
        for run in 0..=runs() {
            self.full_barrier();
            self.props.clear();
            let sat0 = schedstat(&sat_path);
            let xwl0 = process_schedstat(xwl_pid);
            let me0 = schedstat("/proc/thread-self/schedstat");
            let start = Instant::now();
            op(self, n);
            self.full_barrier();
            let wall = start.elapsed().as_nanos() as f64;
            let sat1 = schedstat(&sat_path);
            let xwl1 = process_schedstat(xwl_pid);
            let me1 = schedstat("/proc/thread-self/schedstat");
            let n_f = n as f64;
            let r = [
                wall / n_f,
                (sat1.0 - sat0.0) as f64 / n_f,
                (xwl1.0 - xwl0.0) as f64 / n_f,
                (me1.0 - me0.0) as f64 / n_f,
                (sat1.1 - sat0.1) as f64 / n_f,
            ];
            if run > 0 {
                println!(
                    "BENCH\t{name}\t{n}\t{run}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.2}",
                    r[0], r[1], r[2], r[3], r[4]
                );
                results.push(r);
            }
        }
        let median = |i: usize| {
            let mut v: Vec<f64> = results.iter().map(|r| r[i]).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v[v.len() / 2]
        };
        println!(
            "BENCHMED\t{name}\t{n}\t{:.0}\t{:.0}\t{:.0}\t{:.0}\t{:.2}",
            median(0),
            median(1),
            median(2),
            median(3),
            median(4)
        );
    }

    fn set_fullscreen(&self, window: x::Window, add: bool) {
        let event = x::ClientMessageEvent::new(
            window,
            self.atoms.net_wm_state,
            x::ClientMessageData::Data32([
                add as u32,
                self.atoms.fullscreen.resource_id(),
                0,
                1,
                0,
            ]),
        );
        self.conn.send_request(&x::SendEvent {
            propagate: false,
            destination: x::SendEventDest::Window(self.root),
            event_mask: x::EventMask::SUBSTRUCTURE_NOTIFY | x::EventMask::SUBSTRUCTURE_REDIRECT,
            event: &event,
        });
    }

    /// `count` windows that are created but never mapped (Wine keeps many of these around).
    fn create_unmapped(&mut self, count: usize) {
        for _ in 0..count {
            let wid = self.conn.generate_id();
            self.conn.send_request(&x::CreateWindow {
                depth: 0,
                wid,
                parent: self.root,
                x: 0,
                y: 0,
                width: 10,
                height: 10,
                border_width: 0,
                class: x::WindowClass::InputOutput,
                visual: self.visual,
                value_list: &[],
            });
        }
        self.conn.flush().unwrap();
        self.full_barrier();
    }
}

// ---------------------------------------------------------------------------------------------
// a. ConfigureRequest storm
// ---------------------------------------------------------------------------------------------

/// Position-only ConfigureRequests on a mapped toplevel (satellite ignores the position).
#[test]
#[ignore]
fn configure_position() {
    let mut b = Bench::new(&[]);
    let w = b.map_toplevel(None);
    b.measure("configure_position", ops(5000), |b, n| {
        for i in 0..n {
            b.conn.send_request(&x::ConfigureWindow {
                window: w.window,
                value_list: &[
                    x::ConfigWindow::X((i % 300) as i32 + 1),
                    x::ConfigWindow::Y((i % 200) as i32 + 1),
                ],
            });
            if i % 32 == 31 {
                b.pump();
            }
        }
        b.conn.flush().unwrap();
    });
}

/// Like configure_position, but each request waits on an X -> Wayland barrier, so the time per
/// request includes all of satellite's handling of it (round trips included) before the next one.
#[test]
#[ignore]
fn configure_position_sync() {
    let mut b = Bench::new(&[]);
    let w = b.map_toplevel(None);
    b.measure("configure_position_sync", ops(1000), |b, n| {
        for i in 0..n {
            b.conn.send_request(&x::ConfigureWindow {
                window: w.window,
                value_list: &[
                    x::ConfigWindow::X((i % 300) as i32 + 1),
                    x::ConfigWindow::Y((i % 200) as i32 + 1),
                ],
            });
            b.x_to_wl();
        }
    });
}

/// Position and size requests, one at a time like Wine: each waits for the ConfigureNotify with
/// the new size (X sends one on every tree, since the size changes). Pipelining thousands of
/// resizes instead makes Xwayland's buffer reallocations fill satellite's Wayland socket, which
/// panics or deadlocks satellite on main as well.
#[test]
#[ignore]
fn configure_position_size() {
    let mut b = Bench::new(&[]);
    let w = b.map_toplevel(None);
    b.measure("configure_position_size", ops(1000), |b, n| {
        for i in 0..n {
            let width = 200 + (i % 2) as u16;
            b.conn.send_request(&x::ConfigureWindow {
                window: w.window,
                value_list: &[
                    x::ConfigWindow::X((i % 300) as i32 + 1),
                    x::ConfigWindow::Y((i % 200) as i32 + 1),
                    x::ConfigWindow::Width(width as u32),
                    x::ConfigWindow::Height(150 + (i % 2) as u32),
                ],
            });
            b.conn.flush().unwrap();
            b.pump_until("ConfigureNotify", |b| {
                b.widths.get(&w.window) == Some(&width)
            });
        }
    });
}

// ---------------------------------------------------------------------------------------------
// b. _NET_WM_STATE fullscreen requests
// ---------------------------------------------------------------------------------------------

/// Alternating add/remove requests, all sent at once (every one changes the state).
#[test]
#[ignore]
fn fullscreen_toggle() {
    let mut b = Bench::new(&[]);
    let w = b.map_toplevel(None);
    let mut on = false;
    b.measure("fullscreen_toggle", ops(2000), |b, n| {
        for i in 0..n {
            on = !on;
            b.set_fullscreen(w.window, on);
            if i % 64 == 63 {
                // Flow control: testwl drops satellite if its socket fills up.
                b.x_to_wl();
                b.wl_to_x();
            }
        }
        b.conn.flush().unwrap();
    });
}

/// Like a Wine client: send one request and wait until _NET_WM_STATE has been rewritten.
#[test]
#[ignore]
fn fullscreen_toggle_wait() {
    let mut b = Bench::new(&[]);
    let w = b.map_toplevel(None);
    let net_wm_state = b.atoms.net_wm_state;
    let mut on = false;
    b.measure("fullscreen_toggle_wait", ops(500), |b, n| {
        for _ in 0..n {
            on = !on;
            b.set_fullscreen(w.window, on);
            b.conn.flush().unwrap();
            b.wait_property(w.window, net_wm_state);
        }
    });
}

/// add, add, remove, remove: half of the requests don't change the state (the compositor
/// answers with the state the window already has). Upstream main doesn't answer those, so each
/// request is followed by an X -> Wayland -> X barrier instead of a wait for _NET_WM_STATE.
#[test]
#[ignore]
fn fullscreen_redundant() {
    let mut b = Bench::new(&[]);
    let w = b.map_toplevel(None);
    b.measure("fullscreen_redundant", ops(500), |b, n| {
        for i in 0..n {
            b.set_fullscreen(w.window, i % 4 < 2);
            b.x_to_wl();
            b.wl_to_x();
        }
    });
}

/// The barrier that follows every fullscreen_redundant request, on its own (subtract it).
#[test]
#[ignore]
fn barrier_only() {
    let mut b = Bench::new(&[]);
    let _w = b.map_toplevel(None);
    b.measure("barrier_only", ops(500), |b, n| {
        for _ in 0..n {
            b.x_to_wl();
            b.wl_to_x();
        }
    });
}

// ---------------------------------------------------------------------------------------------
// c. Focus changes and output enters
// ---------------------------------------------------------------------------------------------

/// `mapped` toplevels on one output (two of them are the barrier windows) plus `unmapped`
/// windows that were never mapped; focus cycles through the mapped ones, waiting for each focus
/// to reach X (satellite writes WM_STATE on the window last when it focuses it).
fn focus(name: &str, mapped: usize, unmapped: usize) {
    let mut b = Bench::new(&[(0, 0)]);
    let output = b.outputs[0].clone();
    let mut wins = b.barrier.clone();
    while wins.len() < mapped {
        wins.push(b.map_toplevel(Some(&output)));
    }
    b.create_unmapped(unmapped);
    let wm_state = b.atoms.wm_state;
    let mut idx = 0;
    b.measure(name, ops(2000), |b, n| {
        for _ in 0..n {
            idx = (idx + 1) % wins.len();
            let w = wins[idx];
            b.f.testwl.focus_toplevel(w.surface);
            b.wait_property(w.window, wm_state);
        }
    });
}

#[test]
#[ignore]
fn focus_m2() {
    focus("focus_m2", 2, 0);
}

#[test]
#[ignore]
fn focus_m50() {
    focus("focus_m50", 50, 0);
}

#[test]
#[ignore]
fn focus_m200() {
    focus("focus_m200", 200, 0);
}

#[test]
#[ignore]
fn focus_m3_u1000() {
    focus("focus_m3_u1000", 3, 1000);
}

/// The focused window entering one of two outputs over and over, all sent at once.
fn output_enter(name: &str, mapped: usize, unmapped: usize) {
    let mut b = Bench::new(&[(0, 0), (1000, 0)]);
    let outputs = b.outputs.clone();
    let mut others = 2;
    while others < mapped - 1 {
        b.map_toplevel(Some(&outputs[0]));
        others += 1;
    }
    let w = b.map_toplevel(Some(&outputs[0]));
    b.create_unmapped(unmapped);
    // Barriers don't change focus, so the window keeps it for every run.
    b.f.testwl.focus_toplevel(w.surface);
    b.wait_property(w.window, b.atoms.wm_state);
    b.measure(name, ops(2000), |b, n| {
        for i in 0..n {
            b.f.testwl
                .move_surface_to_output(w.surface, &outputs[(i + 1) % 2]);
            if i % 64 == 63 {
                // Flow control: testwl drops satellite if its socket fills up.
                b.wl_to_x();
            }
        }
    });
}

#[test]
#[ignore]
fn output_enter_m3() {
    output_enter("output_enter_m3", 3, 0);
}

#[test]
#[ignore]
fn output_enter_m200() {
    output_enter("output_enter_m200", 200, 0);
}

#[test]
#[ignore]
fn output_enter_m3_u1000() {
    output_enter("output_enter_m3_u1000", 3, 1000);
}

// ---------------------------------------------------------------------------------------------
// d. Map/unmap churn
// ---------------------------------------------------------------------------------------------

/// Create, map (wait for the xdg_toplevel), configure, destroy (wait for the toplevel to go).
#[test]
#[ignore]
fn map_churn() {
    let mut b = Bench::new(&[(0, 0)]);
    b.measure("map_churn", ops(200), |b, n| {
        for _ in 0..n {
            let window = b.create_window();
            let w = b.map_existing(window);
            b.conn.send_request(&x::DestroyWindow { window: w.window });
            b.conn.flush().unwrap();
            b.pump_until("toplevel destroyed", |b| !b.alive_toplevel(w.surface));
        }
    });
}
