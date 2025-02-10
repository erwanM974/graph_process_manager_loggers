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



use graph_process_manager_core::process::filter::{AbstractNodePreFilter, AbstractStepFilter};

use super::{conf::TreeConfig, context::TreeContextAndParameterization, filtration::TreeFiltrationResult, node::TreeNodeKind, state::TreePersistentState, step::TreeStepKind};






pub enum TreeStepFilter {
    MaxNodeNumber(u32)
}

impl AbstractStepFilter<TreeConfig> for TreeStepFilter {
    fn apply_filter(
        &self,
        _context_and_param : &TreeContextAndParameterization,
        global_state : &TreePersistentState,
        _parent_node : &TreeNodeKind,
        _step : &TreeStepKind
    ) -> Option<TreeFiltrationResult> {
        match self {
            TreeStepFilter::MaxNodeNumber( max_node_number ) => {
                if global_state.node_count >= *max_node_number {
                    return Some( TreeFiltrationResult::MaxNodeNumber );
                }
            }
        }
        None 
    }

    fn get_filter_description(&self) -> String {
        match self {
            TreeStepFilter::MaxNodeNumber(num) => {
                format!("MaxNum={}",num)
            }
        }
    }
}


pub enum TreeNodePreFilter {
    MaxProcessDepth(u32)
}

impl AbstractNodePreFilter<TreeConfig> for TreeNodePreFilter {

    fn apply_filter(
        &self,
        _context_and_param : &TreeContextAndParameterization,
        _global_state : &TreePersistentState,
        node : &TreeNodeKind
    ) -> Option<TreeFiltrationResult> {
        match self {
            TreeNodePreFilter::MaxProcessDepth( max_depth ) => {
                let depth = node.word.len() as u32;
                if depth > *max_depth {
                    return Some( TreeFiltrationResult::MaxProcessDepth );
                }
            }
        }
        None
    }

    fn get_filter_description(&self) -> String {
        match self {
            TreeNodePreFilter::MaxProcessDepth(num) => {
                format!("MaxDepth={}",num)
            }
        }
    }

}

