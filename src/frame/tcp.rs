// SPDX-FileCopyrightText: Copyright (c) 2017-2025 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::*;

pub type TransactionId = u16;
pub type UnitId = u8;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Header {
    pub transaction_id: TransactionId,
    pub unit_id: UnitId,
}

#[derive(Debug, Clone)]
pub struct RequestAdu<'a> {
    pub hdr: Header,
    pub pdu: RequestPdu<'a>,
}

#[derive(Debug, Clone)]
pub struct ResponseAdu {
    pub hdr: Header,
    pub pdu: ResponsePdu,
}

impl<'a> From<RequestAdu<'a>> for Request<'a> {
    fn from(from: RequestAdu<'a>) -> Self {
        from.pdu.into()
    }
}

#[cfg(feature = "server")]
impl<'a> From<RequestAdu<'a>> for SlaveRequest<'a> {
    fn from(from: RequestAdu<'a>) -> Self {
        Self {
            slave: from.hdr.unit_id,
            request: from.pdu.into(),
        }
    }
}
