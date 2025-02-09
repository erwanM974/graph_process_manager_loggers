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




use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use crate::tests::tree_of_trees_proc::context::{TreeOfTreesContext, TreeOfTreesParameterization};
use crate::tests::tree_of_trees_proc::filter::elim::TreeOfTreesFilterEliminationKind;
use crate::tests::tree_of_trees_proc::filter::filter::TreeOfTreesFilterCriterion;
use crate::tests::tree_of_trees_proc::handler::TreeOfTreesProcessHandler;
use crate::tests::tree_of_trees_proc::node::TreeOfTreesNodeKind;
use crate::tests::tree_of_trees_proc::priorities::TreeOfTreesPriorities;
use crate::tests::tree_of_trees_proc::step::TreeOfTreesStepKind;
use crate::tests::tree_of_trees_proc::verdict::global::TreeOfTreesGlobalVerdict;
use crate::tests::tree_of_trees_proc::verdict::local::TreeOfTreesLocalVerdict;

pub struct TreeOfTreesConfig {}

pub struct TreeOfTreesStaticProof {}

impl AbstractProcessConfiguration for TreeOfTreesConfig {
    type Context = TreeOfTreesContext;
    type Parameterization = TreeOfTreesParameterization;
    type NodeKind = TreeOfTreesNodeKind;
    type StepKind = TreeOfTreesStepKind;
    type Priorities = TreeOfTreesPriorities;
    type FilterCriterion = TreeOfTreesFilterCriterion;
    type FilterEliminationKind = TreeOfTreesFilterEliminationKind;
    type LocalVerdict = TreeOfTreesLocalVerdict;
    type StaticLocalVerdictAnalysisProof = TreeOfTreesStaticProof;
    type GlobalVerdict = TreeOfTreesGlobalVerdict;
    type ProcessHandler = TreeOfTreesProcessHandler;
}

