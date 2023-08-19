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

#[derive(Debug,Clone)]
pub struct EventMeshHttpClientConfig {
    pub lite_event_mesh_addr: String,
    pub load_balance_type: LoadBalanceType,
    pub consume_thread_core: i32,
    pub consume_thread_max: i32,
    pub env: String,
    pub consumer_group: String,
    pub producer_group: String,
    pub idc: String,
    pub ip: String,
    pub pid: String,
    pub sys: String,
    pub user_name: String,
    pub password: String,
    pub use_tls: bool,
    pub ssl_client_protocol: String,
    pub max_connection_pool_size: i32,
    pub connection_idle_time_seconds: i32,
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
    pub fn new()-> Self{
        Self::default()
    }
}


#[derive(Debug,Clone)]
pub enum LoadBalanceType {
    RANDOM
}