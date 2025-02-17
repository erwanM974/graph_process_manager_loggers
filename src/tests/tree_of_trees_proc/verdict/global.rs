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
use graph_process_manager_core::process::verdict::AbstractGlobalVerdict;
use crate::tests::tree_of_trees_proc::verdict::local::TreeOfTreesLocalVerdict;


#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct TreeOfTreesGlobalVerdict{}

impl fmt::Display for TreeOfTreesGlobalVerdict {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f,"")
    }
}

impl AbstractGlobalVerdict<TreeOfTreesLocalVerdict> for TreeOfTreesGlobalVerdict {

    fn is_verdict_pertinent_for_process() -> bool {
        false
    }

    fn get_baseline_verdict() -> Self {
        TreeOfTreesGlobalVerdict{}
    }

    fn update_with_local_verdict(self,
                                 _local_verdict: &TreeOfTreesLocalVerdict) -> Self {
        self
    }

    fn is_goal_reached(&self,
                       _goal: &Option<Self>) -> bool {
        false
    }

    fn update_knowing_nodes_were_filtered_out(self,
                                              _has_filtered_nodes: bool) -> Self {
        self
    }
}


