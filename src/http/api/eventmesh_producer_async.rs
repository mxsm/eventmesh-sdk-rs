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
use async_trait::async_trait;

#[async_trait]
pub trait EventMeshProtocolProducer<T: ?Sized> {
    async fn publish(&self, message: T) -> anyhow::Result<()>;

    async fn request(&self, message: T, timeout: u32) -> anyhow::Result<T>;

    async fn request_with_callback(
        &self,
        message: T,
        timeout: u32,
        rr_callback: impl Fn(Option<T>, Option<Box<anyhow::Error>>),
    ) -> anyhow::Result<()>;
}