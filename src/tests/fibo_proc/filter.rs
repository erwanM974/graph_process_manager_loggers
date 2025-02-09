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



use graph_process_manager_core::process::filter::AbstractNodePreFilter;

use crate::tests::fibo_proc::{conf::FiboConfig, context::{FiboContextAndParameterization, FiboPersistentState}, node::FiboNodeKind};

use super::context::FiboFiltrationResult;





pub enum FiboFilter {
    MaxNum(u32)
}



impl AbstractNodePreFilter<FiboConfig> for FiboFilter {

    fn apply_filter(
        &self,
        _context_and_param : &FiboContextAndParameterization,
        _global_state : &FiboPersistentState,
        node : &FiboNodeKind
    ) -> Option<FiboFiltrationResult> {
        match self {
            FiboFilter::MaxNum( max_num ) => {
                if node.current > *max_num {
                    return Some( FiboFiltrationResult::MaxNumberExceeded );
                }
            }
        }
        return None;
    }

    fn get_filter_description(&self) -> String {
        match self {
            FiboFilter::MaxNum(num) => {
                format!("max number = {}", num)
            },
        }
    }

}

