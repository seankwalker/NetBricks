use netbricks::headers::*;
use netbricks::operators::*;
use netbricks::scheduler::*;

pub fn mme<T>(input: T) -> CompositionBatch
where
    T: Batch<Header = NullHeader> + 'static,
{
    let ipv4_packet_batch =
        input
            .parse::<MacHeader>()
            .parse::<Ipv4Header>()
            .transform(box |packet| {
                println!(
                    "packet header: {}\n packet payload: {:?}\n",
                    packet.get_header(),
                    packet.get_payload()
                );

                // array of bytes
                let payload = packet.get_payload();


            });

    ipv4_packet_batch.compose()
}
