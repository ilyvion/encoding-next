// This is a part of encoding-next.
//
// Any copyright is dedicated to the Public Domain.
// https://creativecommons.org/publicdomain/zero/1.0/

//! Korean index tables for [encoding-next](https://github.com/alexschrod/encoding-next).

#![cfg_attr(test, feature(test))]

/// KS X 1001 plus Unified Hangul Code.
///
/// From the Encoding Standard:
///
/// > This matches the KS X 1001 standard and the Unified Hangul Code,
/// > more commonly known together as Windows Codepage 949.
#[rustfmt::skip]
pub mod euc_kr;
