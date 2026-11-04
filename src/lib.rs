//! A pure-Rust writer for MPEG audio elementary streams (MP1, MP2 and MP3):
//! streaming, constant memory, with the Xing/Info + LAME frame and ID3v2/ID3v1
//! tags.
//!
//! The frames are concatenated as an encoder produces them. The Xing/Info +
//! LAME frame is reserved up front and filled in after the stream, so VBR
//! durations, seeking and gapless playback are carried.
//!
//! This crate is in design and has no API yet.
#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![cfg_attr(docsrs, allow(unused_attributes))]
#![deny(missing_docs)]
#![allow(unused_extern_crates)]

#[cfg(all(not(feature = "std"), feature = "alloc"))]
extern crate alloc as std;

#[cfg(feature = "std")]
extern crate std;
