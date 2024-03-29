const BEFORE_COM    : Duration = Duration::from_secs(if std::option_env!("TESTFAST").is_some() { 1 } else { 20 });
const WITH_COM      : Duration = Duration::from_secs(if std::option_env!("TESTFAST").is_some() { 1 } else { 20 });



use xinput::*;

use mmrbi::*;

use winapi::shared::winerror::*;
use winapi::um::combaseapi::*;
use winapi::um::objbase::*;

use std::ptr::null_mut;
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering::Relaxed;
use std::thread::sleep;
use std::time::Duration;



macro_rules! fuzz {
    ( $expr:expr $(, $error:expr)* $(,)? ) => {
        fuzz(stringify!($expr), file!(), line!(), &[$(($error).into()),*], $expr)
    }
}

#[test] fn torment() {
    let should_init_com     = Arc::new(AtomicBool::new(false));
    let continue_running    = Arc::new(AtomicBool::new(true));

    let threads = (0 .. 8).map(|thread_idx|{
        let should_init_com     = should_init_com.clone();
        let continue_running    = continue_running.clone();
        std::thread::spawn(move || {
            let mut initialized_com = false;

            while !initialized_com || continue_running.load(Relaxed) {
                if !initialized_com && should_init_com.load(Relaxed) {
                    initialized_com = true;
                    initialize_com_for_the_first_time_on_this_thread(thread_idx & 1 != 0);
                }

                for i in 0 ..= 100 {
                    let _ = xinput::enable(i>>2 & 1 != 0);
                }

                fuzz!(
                    #[allow(deprecated)] |u| xinput::get_audio_device_ids(u),
                    error::BAD_ARGUMENTS,
                    error::DEVICE_NOT_CONNECTED,    // only on some systems
                    error::INVALID_FUNCTION,
                );

                fuzz!(
                    #[allow(deprecated)] |u| xinput::get_dsound_audio_device_guids(u),
                    error::BAD_ARGUMENTS,
                    error::DEVICE_NOT_CONNECTED,
                    error::INVALID_FUNCTION,
                );

                for bt in [
                    xinput::BatteryDevType::Gamepad,
                    xinput::BatteryDevType::Headset,
                    xinput::BatteryDevType::from_unchecked(2),
                    xinput::BatteryDevType::from_unchecked(42),
                    xinput::BatteryDevType::from_unchecked(255),
                ].iter().copied() {
                    fuzz!(
                        |u| xinput::get_battery_information(u, bt),
                        error::BAD_ARGUMENTS,
                        error::DEVICE_NOT_CONNECTED,
                        error::INVALID_FUNCTION,
                    );
                }

                for flag in [
                    xinput::Flag::None,
                    xinput::Flag::Gamepad,
                    xinput::Flag::from_unchecked(2),
                    xinput::Flag::from_unchecked(4),
                    xinput::Flag::from_unchecked(8),
                    xinput::Flag::from_unchecked(16),
                    xinput::Flag::from_unchecked(32),
                    xinput::Flag::from_unchecked(64),
                    xinput::Flag::from_unchecked(128),
                    xinput::Flag::from_unchecked(!0),
                ].iter().copied() {
                    fuzz!(
                        |u| xinput::get_capabilities(u, flag),
                        error::BAD_ARGUMENTS,
                        error::DEVICE_NOT_CONNECTED,
                    );
                }

                fuzz!(
                    |u| xinput::get_keystroke(u, ()),
                    error::BAD_ARGUMENTS,
                    error::DEVICE_NOT_CONNECTED,
                    error::INVALID_FUNCTION,
                );

                fuzz!(
                    |u| xinput::get_state(u),
                    error::BAD_ARGUMENTS,
                    error::DEVICE_NOT_CONNECTED,
                );

                #[cfg(feature = "undocumented")] fuzz!(
                    #[allow(deprecated)] |u| xinput::get_state_ex(u),
                    error::BAD_ARGUMENTS,
                    error::DEVICE_NOT_CONNECTED,
                );

                for enable in [false, true].iter().copied() {
                    let _ = xinput::enable(enable);
                    for vibration in [
                        xinput::Vibration::default(),
                        xinput::Vibration { left_motor_speed: 1, right_motor_speed: 1 },
                    ].iter().copied() {
                        fuzz!(
                            |u| xinput::set_state(u, vibration),
                            error::BAD_ARGUMENTS,
                            error::DEVICE_NOT_CONNECTED,
                        );
                    }
                }
            }
        })
    });

    // SAFETY: ⚠️ test coverage of no COM init has several caveats
    //  * It's possible that some XInput functions init COM for us, but not others
    //  * It's possible that rust's testing framework inits COM for us
    // Fully fixing this would probably involve multi process shenannigans... I'm fairly satisfied with this being "good enough"
    status!("Testing", "without COM initialized ({:?})", BEFORE_COM);
    sleep(BEFORE_COM);

    status!("Testing", "with COM initialized ({:?})", WITH_COM);
    should_init_com.store(true, Relaxed);
    sleep(WITH_COM);

    status!("Testing", "teardown");
    continue_running.store(false, Relaxed);
    for thread in threads { thread.join().unwrap() }
}

fn initialize_com_for_the_first_time_on_this_thread(mta: bool) {
    // SAFETY: ✔️ reserved is null, flags are always documented
    match unsafe { CoInitializeEx(null_mut(), if mta { COINIT_MULTITHREADED } else { COINIT_APARTMENTTHREADED }) } {
        S_OK    => {},
        S_FALSE => fatal!("CoInitializeEx(nullptr, ...) returned S_FALSE: COM was already initialized on this thread!  Tests were trying to ensure xinput is sound to use pre-COM..."),
        err     => fatal!("CoInitializeEx(nullptr, ...) returned 0x{:08X} (unexpected {})", err, if SUCCEEDED(err) { "success code" } else { "error" }),
    }
    // We don't ever try to pair this with uninitializing COM
}

fn fuzz<T>(title: &str, file: &str, line: u32, codes: &[xinput::error::Kind], f: impl Fn(u32) -> Result<T, Error>) {
    for u in [0, 1, 2, 3, xuser::INDEX_ANY, 4, 42, 67, 254] {
        for _ in 0 ..= 100 {
            if let Err(err) = f(u) {
                let kind = err.kind();
                if !codes.contains(&kind) { fatal!(at: file, line: line as _, "when fuzzing {}: unexpected error {:?}", title, kind); }
            }
        }
    }

    for u in 0 ..= 255 {
        if let Err(err) = f(u) {
            let kind = err.kind();
            if !codes.contains(&kind) { fatal!(at: file, line: line as _, "when fuzzing {}: unexpected error {:?}", title, kind); }
        }
    }
}
