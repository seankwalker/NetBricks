use colored::*;
use netbricks::headers::*;
use netbricks::operators::*;
use std::str::FromStr;

struct Meta {
    payload_len: u16,
}

pub fn ndp_nf<T: 'static + Batch<Header = NullHeader>>(parent: T) -> CompositionBatch {
    let pipeline = parent
        .parse::<MacHeader>()
        .filter(box |pkt| match pkt.get_header().etype() {
            Some(EtherType::IPv6) => true,
            _ => false,
        });

    ndp_router_advertisement_nf(pipeline)
}

#[inline]
fn ndp_router_advertisement_nf<T: 'static + Batch<Header = MacHeader>>(
    parent: T,
) -> CompositionBatch {
    println!(
        "{}",
        format!("Tests ICMPv6 messages for msg_type, code and checksum").white()
    );
    parent
        .parse::<Ipv6Header>()
        .metadata(box |pkt| Meta {
            payload_len: pkt.get_header().payload_len(),
        })
        .parse::<Icmpv6RouterAdvertisement<Ipv6Header>>()
        .transform(box |pkt| {
            let payload_len = pkt.read_metadata().payload_len;
            let dl = pkt.data_len();
            let router_advertisement = pkt.get_mut_header();

            println!("payload_len {}; data_len {}", payload_len, dl);

            println!(
                "{}",
                format!(
                    "   Msg Type: {:X?} | Code: {} | Checksum: {:X?}",
                    router_advertisement.msg_type().unwrap(),
                    router_advertisement.code(),
                    router_advertisement.checksum()
                )
                .purple()
            );

            assert_eq!(
                format!("{:X?}", router_advertisement.msg_type().unwrap()),
                format!("{:X?}", IcmpMessageType::RouterAdvertisement)
            );
            assert_eq!(
                format!("{:X?}", router_advertisement.code()),
                format!("{:X?}", 0)
            );
            assert_eq!(
                format!("{:X?}", router_advertisement.checksum()),
                format!("{:X?}", 0xbff2)
            );
            assert_eq!(
                format!("{:X?}", router_advertisement.current_hop_limit()),
                format!("{:X?}", 64)
            );
            assert_eq!(
                format!("{:X?}", router_advertisement.managed_addr_cfg()),
                format!("{:X?}", true)
            );
            assert_eq!(
                format!("{:X?}", router_advertisement.other_cfg()),
                format!("{:X?}", true)
            );
            assert_eq!(
                format!("{:X?}", router_advertisement.router_lifetime()),
                format!("{:X?}", 1800)
            );
            assert_eq!(
                format!("{:X?}", router_advertisement.reachable_time()),
                format!("{:X?}", 600)
            );
            assert_eq!(
                format!("{:X?}", router_advertisement.retrans_timer()),
                format!("{:X?}", 500)
            );

            assert_eq!(format!("{:X?}", payload_len), format!("{:X?}", 64));

            assert_eq!(
                format!(
                    "{:X?}",
                    router_advertisement
                        .get_source_link_layer_address_option(payload_len)
                        .unwrap()
                ),
                format!("{:X?}", MacAddress::from_str("c2:00:54:f5:00:00").unwrap())
            );

            assert_eq!(
                format!(
                    "{:X?}",
                    router_advertisement.get_mtu_option(payload_len).unwrap()
                ),
                format!("{:X?}", 1500)
            );
        })
        .compose()
}
