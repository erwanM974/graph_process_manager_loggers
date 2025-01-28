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



use std::fmt;
use graph_process_manager_core::handler::filter::AbstractFilter;

use crate::tests::tree_of_trees_proc::filter::elim::TreeOfTreesFilterEliminationKind;


pub struct TreeOfTreesFilterCriterion {}

impl fmt::Display for TreeOfTreesFilterCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}


pub enum TreeOfTreesFilter {
    MaxProcessDepth(u32),
    MaxNodeNumber(u32)
}

impl fmt::Display for TreeOfTreesFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TreeOfTreesFilter::MaxProcessDepth(num) => {
                write!(f,"MaxDepth={}",num)
            },
            TreeOfTreesFilter::MaxNodeNumber(num) => {
                write!(f,"MaxNum={}",num)
            }
        }
    }
}


impl AbstractFilter<TreeOfTreesFilterCriterion,TreeOfTreesFilterEliminationKind>  for TreeOfTreesFilter {

    fn apply_filter(&self,
                    depth: u32,
                    node_counter: u32,
                    _criterion: &TreeOfTreesFilterCriterion) -> Option<TreeOfTreesFilterEliminationKind> {
        match self {
            TreeOfTreesFilter::MaxProcessDepth( max_depth ) => {
                if depth > *max_depth {
                    return Some( TreeOfTreesFilterEliminationKind::MaxProcessDepth );
                }
            },
            TreeOfTreesFilter::MaxNodeNumber( max_node_number ) => {
                if node_counter >= *max_node_number {
                    return Some( TreeOfTreesFilterEliminationKind::MaxNodeNumber );
                }
            }
        }
        return None;
    }

}

