/*
Copyright 2020 Erwan Mahe (github.com/erwanM974)

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/




use std::hash::Hash;

use graph_process_manager_core::process::config::AbstractNodeKind;


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct FiboNodeKind {
    pub current : u32,
    pub next : u32
}

impl FiboNodeKind {
    pub fn new(current: u32, next: u32) -> Self {
        Self { current, next }
    }
}

impl AbstractNodeKind for FiboNodeKind {
    fn is_included_for_memoization(&self, memoized_node: &Self) -> bool {
        memoized_node.current == self.current && memoized_node.next == self.next
    }
}

