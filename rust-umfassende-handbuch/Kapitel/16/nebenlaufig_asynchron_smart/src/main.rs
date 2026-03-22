#[cfg(feature = "thread_send_sync")]
mod send_sync;
mod threads;
mod atomar;
mod smart_pointer;
mod asynchron;

use threads::{builder, channel, panics, thread_starten};

#[cfg(feature = "thread_scope")]
use threads::scope;

#[cfg(feature = "thread_barrier")]
use threads::barrier;

#[cfg(feature = "thread_mutex")]
use threads::mutex;

#[cfg(feature = "thread_arc_beispiel")]
use threads::lebenszeiten_und_move;

#[cfg(feature = "thread_rwlock")]
use threads::rwlock;

#[cfg(feature = "thread_condvar")]
use threads::cond_var;

#[cfg(feature = "thread_park_unpark")]
use crate::threads::park_unpark;

#[cfg(feature = "thread_once")]
use crate::threads::once;

fn main() {
    #[cfg(feature = "thread_starten")]
    thread_starten::ohne_join::main();

    #[cfg(feature = "thread_starten_join")]
    thread_starten::mit_join::main();

    #[cfg(feature = "thread_arc_beispiel")]
    lebenszeiten_und_move::main();

    builder::main();

    #[cfg(feature = "thread_scope")]
    scope::main();

    #[cfg(feature = "thread_barrier")]
    barrier::main();

    #[cfg(feature = "thread_park_unpark")]
    park_unpark::main();

    #[cfg(feature = "thread_once")]
    once::main();

    panics::main();

    #[cfg(feature = "thread_send_sync")]
    send_sync::main();

    #[cfg(feature = "thread_channel_async")]
    channel::async_channel::main();

    #[cfg(feature = "thread_channel_sync")]
    channel::sync_channel::main();

    #[cfg(feature = "thread_mutex")]
    mutex::main();

    #[cfg(feature = "thread_rwlock")]
    rwlock::main();

    #[cfg(feature = "thread_condvar")]
    cond_var::main();


    #[cfg(feature = "thread_atomar")]
    atomar::operationen::main();

    #[cfg(feature = "smart_pointer")]
    smart_pointer::main();

    asynchron::main();
}
