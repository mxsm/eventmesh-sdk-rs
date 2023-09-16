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

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::str::FromStr;

pub struct SubscriptionItem {
    pub(crate) topic: String,
    pub(crate) sub_mode: SubscriptionMode,
    pub(crate) sub_type: SubscriptionType,
}

impl SubscriptionItem {
    pub fn new(
        topic: impl AsRef<str>,
        sub_mode: SubscriptionMode,
        sub_type: SubscriptionType,
    ) -> Self {
        Self {
            topic: topic.as_ref().to_string(),
            sub_mode,
            sub_type,
        }
    }

    #[allow(unused)]
    pub fn with_topic(mut self, topic: impl Into<String>) -> Self {
        self.topic = topic.into();
        self
    }

    #[allow(unused)]
    pub fn with_mode(mut self, sub_mode: SubscriptionMode) -> Self {
        self.sub_mode = sub_mode.into();
        self
    }

    #[allow(unused)]
    pub fn with_type(mut self, sub_type: SubscriptionType) -> Self {
        self.sub_type = sub_type.into();
        self
    }
}

#[derive(Debug)]
pub enum SubscriptionMode {
    BROADCASTING,
    CLUSTERING,
    UNRECOGNIZED,
}

impl Display for SubscriptionMode {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionMode::BROADCASTING => {
                write!(f, "BROADCASTING")
            }
            SubscriptionMode::CLUSTERING => {
                write!(f, "CLUSTERING")
            }
            SubscriptionMode::UNRECOGNIZED => {
                write!(f, "UNRECOGNIZED")
            }
        }
    }
}

impl FromStr for SubscriptionMode {
    type Err = StringParseEnumError;

    fn from_str(st: &str) -> Result<Self, Self::Err> {
        let binding = st.to_uppercase();
        let change_upper = Some(binding.as_str());
        match change_upper {
            None => Err(StringParseEnumError("Empty String".to_string())),
            Some("BROADCASTING") => Ok(SubscriptionMode::BROADCASTING),
            Some("CLUSTERING") => Ok(SubscriptionMode::CLUSTERING),
            Some("UNRECOGNIZED") => Ok(SubscriptionMode::UNRECOGNIZED),
            Some(_) => Err(StringParseEnumError(change_upper.unwrap().to_string())),
        }
    }
}

#[derive(Clone, Debug)]
pub struct StringParseEnumError(String);

impl Display for StringParseEnumError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} can not parse to enum", self.0)
    }
}

impl Error for StringParseEnumError {}

#[derive(Debug)]
pub enum SubscriptionType {
    SYNC,
    ASYNC,
    UNRECOGNIZED,
}

impl Display for SubscriptionType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SubscriptionType::SYNC => {
                write!(f, "SYNC")
            }
            SubscriptionType::ASYNC => {
                write!(f, "ASYNC")
            }
            SubscriptionType::UNRECOGNIZED => {
                write!(f, "UNRECOGNIZED")
            }
        }
    }
}

impl FromStr for SubscriptionType {
    type Err = StringParseEnumError;

    fn from_str(st: &str) -> Result<Self, Self::Err> {
        let binding = st.to_uppercase();
        let change_upper = Some(binding.as_str());
        match change_upper {
            None => Err(StringParseEnumError("Empty String".to_string())),
            Some("SYNC") => Ok(SubscriptionType::SYNC),
            Some("ASYNC") => Ok(SubscriptionType::ASYNC),
            Some("UNRECOGNIZED") => Ok(SubscriptionType::UNRECOGNIZED),
            Some(_) => Err(StringParseEnumError(change_upper.unwrap().to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::module::subscription_item::{SubscriptionItem, SubscriptionMode, SubscriptionType};

    #[test]
    fn test_subscription_item_new() {
        let item = SubscriptionItem::new(
            "mxsm",
            SubscriptionMode::UNRECOGNIZED,
            SubscriptionType::UNRECOGNIZED,
        );
        assert_eq!("mxsm", item.topic)
    }
}
