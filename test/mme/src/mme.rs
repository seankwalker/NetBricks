use netbricks::headers::*;
use netbricks::operators::*;
use netbricks::scheduler::*;

pub fn mme<T, S>(parent: T, sched: &mut S) -> CompositionBatch where
    T: Batch<Header = NullHeader> + 'static,
    S: Scheduler + Sized
{
    parent.parse::<MacHeader>().transform(box move |packet| {
        println!("packet: {:?}", packet);
    });
}