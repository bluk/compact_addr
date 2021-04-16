// Copyright 2021 Bryant Luk
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Standard IP addresses with ports represented as compact byte arrays.

use std::net::{Ipv4Addr, Ipv6Addr, SocketAddrV4, SocketAddrV6};

/// An IPv4 socket address representable by a compact format.
///
/// The trait is intended to help convert an IPv4 socket address to a compact form.
///
/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait CompactAddrV4Info: private::Sealed {
    /// Returns the address encoded as a compact address.
    fn to_compact_address(&self) -> [u8; 6];

    /// Converts from the compact address to the self type.
    fn from_compact_address(bytes: &[u8; 6]) -> Self;
}

impl CompactAddrV4Info for SocketAddrV4 {
    fn to_compact_address(&self) -> [u8; 6] {
        let mut a: [u8; 6] = [0; 6];
        a[0..4].copy_from_slice(&self.ip().octets());
        a[4..6].copy_from_slice(&self.port().to_be_bytes());
        a
    }

    fn from_compact_address(bytes: &[u8; 6]) -> Self {
        let mut ip: [u8; 4] = [0; 4];
        ip[0..4].copy_from_slice(&bytes[0..4]);
        let ip = Ipv4Addr::from(ip);

        let mut port: [u8; 2] = [0; 2];
        port[0..2].copy_from_slice(&bytes[4..6]);
        let port = u16::from_be_bytes(port);

        SocketAddrV4::new(ip, port)
    }
}

/// An IPv6 socket address representable by a compact format.
///
/// The trait is intended to help convert an IPv6 socket address to a compact form.
///
/// This trait is sealed and cannot be implemented for types outside this crate.
pub trait CompactAddrV6Info: private::Sealed {
    /// Returns the address encoded as a compact address.
    fn to_compact_address(&self) -> [u8; 18];

    /// Converts from the compact address to the self type.
    fn from_compact_address(bytes: &[u8; 18]) -> Self;
}

impl CompactAddrV6Info for SocketAddrV6 {
    fn to_compact_address(&self) -> [u8; 18] {
        let mut a: [u8; 18] = [0; 18];
        a[0..16].copy_from_slice(&self.ip().octets());
        a[16..18].copy_from_slice(&self.port().to_be_bytes());
        a
    }

    fn from_compact_address(bytes: &[u8; 18]) -> Self {
        let mut ip: [u8; 16] = [0; 16];
        ip[0..16].copy_from_slice(&bytes[0..16]);
        let ip = Ipv6Addr::from(ip);

        let mut port: [u8; 2] = [0; 2];
        port[0..2].copy_from_slice(&bytes[16..18]);
        let port = u16::from_be_bytes(port);

        SocketAddrV6::new(ip, port, 0, 0)
    }
}

mod private {
    use std::net::{SocketAddrV4, SocketAddrV6};

    pub trait Sealed {}

    impl Sealed for SocketAddrV6 {}
    impl Sealed for SocketAddrV4 {}
}
