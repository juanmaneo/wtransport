#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod bytes;
pub mod datagram;
pub mod error;
pub mod frame;
pub mod headers;
pub mod ids;
pub mod settings;
pub mod stream;
pub mod varint;

/// Application Layer Protocol Negotiation for WebTransport connections.
pub const WEBTRANSPORT_ALPN: &[u8; 2] = b"h3";
