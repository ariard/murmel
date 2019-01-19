//
// Copyright 2018 Tamas Blummer
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//
//!
//! # Download Blocks
//!

use configdb::SharedConfigDB;
use chaindb::SharedChainDB;
use p2p::{PeerMessageReceiver, P2PControlSender};

use bitcoin::{
    blockdata::block::Block
};

#[allow(unused)]
pub struct BlockDownloader {
    configdb: SharedConfigDB,
    chaindb: SharedChainDB,
    p2p: P2PControlSender,
    messages: PeerMessageReceiver
}

impl BlockDownloader {
    pub fn new (configdb: SharedConfigDB, chaindb: SharedChainDB, p2p: P2PControlSender, messages: PeerMessageReceiver) -> BlockDownloader {
        BlockDownloader{ configdb, chaindb, p2p, messages }
    }

    pub fn run(&mut self) {

    }
}