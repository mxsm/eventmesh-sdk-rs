/*
 * Licensed to the Apache Software Foundation (ASF) under one or more
 * contributor license agreements.  See the NOTICE file distributed with
 * this work for additional information regarding copyright ownership.
 * The ASF licenses this file to You under the Apache License, Version 2.0
 * (the "License"); you may not use this file except in compliance with
 * the License.  You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

use std::fmt::{Display, Formatter};

pub(crate) struct ProtocolKey;

impl ProtocolKey {
    pub(crate) const REQUEST_CODE: &'static str = "code";
    pub(crate) const REQUEST_URI: &'static str = "uri";
    pub(crate) const LANGUAGE: &'static str = "language";
    pub(crate) const VERSION: &'static str = "version";
    pub(crate) const PROTOCOL_TYPE: &'static str = "protocoltype";
    pub(crate) const PROTOCOL_VERSION: &'static str = "protocolversion";
    pub(crate) const PROTOCOL_DESC: &'static str = "protocoldesc";
    pub(crate) const CONTENT_TYPE: &'static str = "contenttype";
}

pub(crate) enum ClientInstanceKey {
    ENV,
    IDC,
    SYS,
    PID,
    IP,
    USERNAME,
    PASSWD,
    BIZSEQNO,
    UNIQUEID,
    PRODUCERGROUP,
    CONSUMERGROUP,
    TOKEN,
}

impl ClientInstanceKey {
    pub(crate) fn key(&self) -> &'static str {
        match self {
            ClientInstanceKey::ENV => "env",
            ClientInstanceKey::IDC => "idc",
            ClientInstanceKey::SYS => "sys",
            ClientInstanceKey::PID => "pid",
            ClientInstanceKey::IP => "ip",
            ClientInstanceKey::USERNAME => "username",
            ClientInstanceKey::PASSWD => "passwd",
            ClientInstanceKey::BIZSEQNO => "bizseqno",
            ClientInstanceKey::UNIQUEID => "uniqueid",
            ClientInstanceKey::PRODUCERGROUP => "producergroup",
            ClientInstanceKey::CONSUMERGROUP => "consumergroup",
            ClientInstanceKey::TOKEN => "token",
        }
    }
}

pub(crate) struct EventMeshInstanceKey;

impl EventMeshInstanceKey {
    const EVENTMESH_CLUSTER: &'static str = "eventmeshcluster";
    const EVENTMESH_IP: &'static str = "eventmeship";
    const EVENTMESH_ENV: &'static str = "eventmeshenv";
    const EVENTMESH_IDC: &'static str = "eventmeshidc";
}

pub(crate) struct CloudEventsKey;

impl CloudEventsKey {
    const ID: &'static str = "id";
    const SOURCE: &'static str = "source";
    const SUBJECT: &'static str = "subject";
    const TYPE: &'static str = "type";
}

pub(crate) struct ProtocolConstants;

impl ProtocolConstants {
    const RET_CODE: &'static str = "retCode";
    const RET_MSG: &'static str = "retMsg";
    const RES_TIME: &'static str = "resTime";
}

#[derive(Debug, Clone, Copy)]
pub(crate) enum ProtocolVersion {
    V1,
    V2,
}

impl ProtocolVersion {
    pub(crate) fn value(&self) -> &'static str {
        match self {
            ProtocolVersion::V1 => "1.0",
            ProtocolVersion::V2 => "2.0",
            _ => "Unknown protocol version "
        }
    }
}

impl Display for ProtocolVersion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ProtocolVersion::V1 => write!(f, "1.0"),
            ProtocolVersion::V2 => write!(f, "2.0"),
            _ => write!(f, "Unknown protocol version")
        }
    }
}

pub(crate) struct ProtocolConstant;

impl ProtocolConstant {
    pub(crate) const CE_PROTOCOL: &'static str = "cloudevents";
    pub(crate) const EM_MESSAGE_PROTOCOL: &'static str = "eventmeshmessage";
    pub(crate) const OP_MESSAGE_PROTOCOL: &'static str = "openmessage";
    pub(crate) const PROTOCOL_DESC: &'static str = "http";
}

#[derive(Debug, PartialEq)]
pub(crate) enum RequestCode {
    Unknown,
    MsgBatchSend,
    MsgBatchSendV2,
    MsgSendSync,
    MsgSendAsync,
    HttpPushClientAsync,
    HttpPushClientSync,
    Register,
    Unregister,
    Heartbeat,
    Subscribe,
    Unsubscribe,
    ReplyMessage,
    AdminMetrics,
    AdminShutdown,
}

impl RequestCode {
    pub(crate) fn new_with_code(code: i32) -> Self {
        match code {
            0 => RequestCode::Unknown,
            102 => RequestCode::MsgBatchSend,
            107 => RequestCode::MsgBatchSendV2,
            101 => RequestCode::MsgSendSync,
            104 => RequestCode::MsgSendAsync,
            105 => RequestCode::HttpPushClientAsync,
            106 => RequestCode::HttpPushClientSync,
            201 => RequestCode::Register,
            202 => RequestCode::Unsubscribe,
            203 => RequestCode::Heartbeat,
            206 => RequestCode::Subscribe,
            207 => RequestCode::Unsubscribe,
            301 => RequestCode::ReplyMessage,
            603 => RequestCode::AdminMetrics,
            601 => RequestCode::AdminShutdown,
            _ => RequestCode::Unknown,
        }
    }

    pub(crate) fn get_code(&self) -> i32 {
        match self {
            RequestCode::Unknown => 0,
            RequestCode::MsgBatchSend => 102,
            RequestCode::MsgBatchSendV2 => 107,
            RequestCode::MsgSendSync => 101,
            RequestCode::MsgSendAsync => 104,
            RequestCode::HttpPushClientAsync => 105,
            RequestCode::HttpPushClientSync => 106,
            RequestCode::Register => 201,
            RequestCode::Unregister => 202,
            RequestCode::Heartbeat => 203,
            RequestCode::Subscribe => 206,
            RequestCode::Unsubscribe => 207,
            RequestCode::ReplyMessage => 301,
            RequestCode::AdminMetrics => 603,
            RequestCode::AdminShutdown => 601,
        }
    }

    pub(crate) fn get_desc(&self) -> &'static str {
        match self {
            RequestCode::Unknown => "UNKNOWN",
            RequestCode::MsgBatchSend => "SEND BATCH MSG",
            RequestCode::MsgBatchSendV2 => "SEND BATCH MSG V2",
            RequestCode::MsgSendSync => "SEND SINGLE MSG SYNC",
            RequestCode::MsgSendAsync => "SEND SINGLE MSG ASYNC",
            RequestCode::HttpPushClientAsync => "PUSH CLIENT BY HTTP POST",
            RequestCode::HttpPushClientSync => "PUSH CLIENT BY HTTP POST",
            RequestCode::Register => "REGISTER",
            RequestCode::Unregister => "UNREGISTER",
            RequestCode::Heartbeat => "HEARTBEAT",
            RequestCode::Subscribe => "SUBSCRIBE",
            RequestCode::Unsubscribe => "UNSUBSCRIBE",
            RequestCode::ReplyMessage => "REPLY MESSAGE",
            RequestCode::AdminMetrics => "ADMIN METRICS",
            RequestCode::AdminShutdown => "ADMIN SHUTDOWN",
        }
    }
}
