// SPDX-FileCopyrightText: Copyright (c) 2017-2025 slowtec GmbH <post@slowtec.de>
// SPDX-License-Identifier: MIT OR Apache-2.0

use super::*;

use crate::slave::SlaveId;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header {
    pub slave_id: SlaveId,
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
            slave: from.hdr.slave_id,
            request: from.pdu.into(),
        }
    }
}
