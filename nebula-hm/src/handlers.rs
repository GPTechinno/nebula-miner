use core::sync::atomic::{compiler_fence, Ordering};

use embassy_time::{Instant, Timer};
use nebula_hicd::{Asic, Chain, Info, Job, LedState, SleepEndpoint, SleepMillis, SleptMillis};
use postcard_rpc::{header::VarHeader, server::Sender};

use crate::app::{AppTx, Context, TaskContext};

/// This is an example of a BLOCKING handler.
pub fn unique_id(ctx: &mut Context, _header: VarHeader, _arg: ()) -> u64 {
    ctx.unique_id
}

/// Also a BLOCKING handler
pub fn picoboot_reset(_ctx: &mut Context, _header: VarHeader, _arg: ()) {
    embassy_rp::rom_data::reset_to_usb_boot(0, 0);
    loop {
        // Wait for reset...
        compiler_fence(Ordering::SeqCst);
    }
}

/// Also a BLOCKING handler
pub fn set_led(ctx: &mut Context, _header: VarHeader, arg: LedState) {
    match arg {
        LedState::Off => ctx.led.set_low(),
        LedState::On => ctx.led.set_high(),
    }
}

pub fn info<'a>(_ctx: &mut Context, _header: VarHeader, _arg: ()) -> Info<'a> {
    Info {
        version: env!("CARGO_PKG_VERSION"),
        chain: Chain {
            asic: Asic::Bm1370,
            cnt: 1,
        },
    }
}

/// This is a SPAWN handler
///
/// The pool size of three means we can have up to three of these requests "in flight"
/// at the same time. We will return an error if a fourth is requested at the same time
#[embassy_executor::task(pool_size = 3)]
pub async fn sleep_handler(
    _ctx: TaskContext,
    header: VarHeader,
    arg: SleepMillis,
    sender: Sender<AppTx>,
) {
    // We can send string logs, using the sender
    let _ = sender.log_str("Starting sleep...").await;
    let start = Instant::now();
    Timer::after_millis(arg.millis.into()).await;
    let _ = sender.log_str("Finished sleep").await;
    // Async handlers have to manually reply, as embassy doesn't support returning by value
    let _ = sender
        .reply::<SleepEndpoint>(
            header.seq_no,
            &SleptMillis {
                millis: start.elapsed().as_millis() as u16,
            },
        )
        .await;
}

pub async fn hash_job(_ctx: &mut Context, _header: VarHeader, _arg: Job, _sender: &Sender<AppTx>) {
    todo!("hash job")
}

pub async fn stop_hashing(
    _ctx: &mut Context,
    _header: VarHeader,
    _arg: (),
    _sender: &Sender<AppTx>,
) {
    todo!("stop hashing")
}
