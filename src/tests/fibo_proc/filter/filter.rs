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

use crate::tests::fibo_proc::filter::elim::FiboFilterEliminationKind;


pub struct FiboFilterCriterion {}

impl fmt::Display for FiboFilterCriterion {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}


pub enum FiboFilter {
    MaxProcessDepth(u32),
    MaxNodeNumber(u32)
}

impl fmt::Display for FiboFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FiboFilter::MaxProcessDepth(num) => {
                write!(f,"MaxDepth={}",num)
            },
            FiboFilter::MaxNodeNumber(num) => {
                write!(f,"MaxNum={}",num)
            }
        }
    }
}


impl AbstractFilter<FiboFilterCriterion,FiboFilterEliminationKind>  for FiboFilter {

    fn apply_filter(&self,
                    depth: u32,
                    node_counter: u32,
                    _criterion: &FiboFilterCriterion) -> Option<FiboFilterEliminationKind> {
        match self {
            FiboFilter::MaxProcessDepth( max_depth ) => {
                if depth > *max_depth {
                    return Some( FiboFilterEliminationKind::MaxProcessDepth );
                }
            },
            FiboFilter::MaxNodeNumber( max_node_number ) => {
                if node_counter >= *max_node_number {
                    return Some( FiboFilterEliminationKind::MaxNodeNumber );
                }
            }
        }
        return None;
    }

}

