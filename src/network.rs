use pnet::packet::Packet;

use crate::ActionResult;

pub fn config () -> ActionResult<Vec<pnet::datalink::NetworkInterface>> {
    Ok(pnet::datalink::interfaces())
}

pub fn traceroute () -> ActionResult<String> {
    let interfaces = pnet::datalink::interfaces();
    let interface = interfaces.iter().find(|interface| interface.is_up() && !interface.is_loopback());

    if let Some(interface) = interface {
        println!("{interface}");

        let protocol = pnet::transport::TransportChannelType::Layer3(pnet::packet::ip::IpNextHeaderProtocols::Icmp);
        // let protocol = pnet::transport::TransportChannelType::Layer4(pnet::transport::TransportProtocol::Ipv4(pnet::packet::ip::IpNextHeaderProtocols::Icmp));
        let (mut tx, mut rx) = pnet::transport::transport_channel(u16::MAX as usize, protocol).map_err(|err| err.to_string())?;

        // rx.
        let destination = std::net::Ipv4Addr::new(8, 8, 8, 8);

        // Network
        // let mut buf = [0u8; 50];
        // let mut ip = pnet::packet::ipv4::MutableIpv4Packet::new(&mut buf).unwrap();
        // ip.set_version(4);
        // ip.set_header_length(5);
        // ip.set_total_length(20 + 5);
        // ip.set_ttl(64); // TODO
        // ip.set_destination(destination);
        // ip.set_next_level_protocol(pnet::packet::ip::IpNextHeaderProtocols::Udp);

        // // // Transport
        // // let mut buf = [0u8; 24];
        // // let mut udp = pnet::packet::udp::MutableUdpPacket::new(&mut buf).unwrap();
        // // udp.set_length(8);
        // // udp.set_checksum(pnet::util::checksum(&ip.packet(), 1));
        // // // udp.set_payload(&buf);

        // // ip.set_payload(&udp.packet());
        // ip.set_payload(&[1, 2, 3, 4, 5]);

        // Transport
        let mut buf = [0u8; 8];
        let mut icmp = pnet::packet::icmp::echo_request::MutableEchoRequestPacket::new(&mut buf).unwrap();
        icmp.set_identifier(0xabcd);
        icmp.set_sequence_number(0);
        icmp.set_icmp_type(pnet::packet::icmp::IcmpTypes::EchoRequest);
        icmp.set_checksum(pnet::util::checksum(&icmp.packet(), 1));

        println!("{icmp:?} {:?}", icmp.packet());
        tx.send_to(icmp, std::net::IpAddr::V4(destination)).unwrap();

        let mut rx = pnet::transport::icmp_packet_iter(&mut rx);
        let (packet, address) = rx.next().map_err(|err| err.to_string())?;
        println!("{address} {packet:?} {:?}", packet.payload());

        Ok("a".to_string())
    } else {
        Err("No available interface found".to_string())
    }
}
