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

#[derive(Debug, Clone)]
pub struct EventMeshHttpClientConfig {
    pub(crate) lite_event_mesh_addr: String,
    pub(crate) load_balance_type: LoadBalanceType,
    pub(crate) consume_thread_core: i32,
    pub(crate) consume_thread_max: i32,
    pub(crate) env: String,
    pub(crate) consumer_group: String,
    pub(crate) producer_group: String,
    pub(crate) idc: String,
    pub(crate) ip: String,
    pub(crate) pid: String,
    pub(crate) sys: String,
    pub(crate) user_name: String,
    pub(crate) password: String,
    pub(crate) use_tls: bool,
    pub(crate) ssl_client_protocol: String,
    pub(crate) max_connection_pool_size: i32,
    pub(crate) connection_idle_time_seconds: i32,
}

impl Default for EventMeshHttpClientConfig {
    fn default() -> Self {
        Self {
            lite_event_mesh_addr: String::from("http://localhost:10105"),
            load_balance_type: LoadBalanceType::RANDOM,
            consume_thread_core: 2,
            consume_thread_max: 5,
            env: String::default(),
            consumer_group: String::from("DefaultConsumerGroup"),
            producer_group: String::from("DefaultProducerGroup"),
            idc: String::default(),
            ip: String::from("localhost"),
            pid: String::default(),
            sys: String::default(),
            user_name: String::default(),
            password: String::default(),
            use_tls: false,
            ssl_client_protocol: String::from("TLSv1.2"),
            max_connection_pool_size: 30,
            connection_idle_time_seconds: 10,
        }
    }
}

impl EventMeshHttpClientConfig {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lite_event_mesh_addr(&mut self, event_mesh_addr: impl Into<String>) -> &mut Self {
        self.lite_event_mesh_addr = event_mesh_addr.into();
        self
    }

    pub fn env(&mut self, env: impl Into<String>) -> &mut Self {
        self.env = env.into();
        self
    }

    pub fn consumer_group(&mut self, consumer_group: impl Into<String>) -> &mut Self {
        self.consumer_group = consumer_group.into();
        self
    }

    pub fn producer_group(&mut self, producer_group: impl Into<String>) -> &mut Self {
        self.producer_group = producer_group.into();
        self
    }

    pub fn idc(&mut self, idc: impl Into<String>) -> &mut Self {
        self.producer_group = idc.into();
        self
    }

    pub fn ip(&mut self, ip: impl Into<String>) -> &mut Self {
        self.ip = ip.into();
        self
    }

    pub fn sys(&mut self, sys: impl Into<String>) -> &mut Self {
        self.sys = sys.into();
        self
    }

    pub fn pid(&mut self, pid: impl Into<String>) -> &mut Self {
        self.pid = pid.into();
        self
    }
}

#[derive(Debug, Clone)]
pub enum LoadBalanceType {
    RANDOM,
}
