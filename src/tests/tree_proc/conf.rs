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




use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use crate::tests::tree_proc::context::{TreeContext, TreeParameterization};
use crate::tests::tree_proc::filter::elim::TreeFilterEliminationKind;
use crate::tests::tree_proc::filter::filter::TreeFilterCriterion;
use crate::tests::tree_proc::handler::TreeProcessHandler;
use crate::tests::tree_proc::node::TreeNodeKind;
use crate::tests::tree_proc::priorities::TreePriorities;
use crate::tests::tree_proc::step::TreeStepKind;
use crate::tests::tree_proc::verdict::global::TreeGlobalVerdict;
use crate::tests::tree_proc::verdict::local::TreeLocalVerdict;

pub struct TreeConfig {}

pub struct TreeStaticProof {}

impl AbstractProcessConfiguration for TreeConfig {
    type Context = TreeContext;
    type Parameterization = TreeParameterization;
    type NodeKind = TreeNodeKind;
    type StepKind = TreeStepKind;
    type Priorities = TreePriorities;
    type FilterCriterion = TreeFilterCriterion;
    type FilterEliminationKind = TreeFilterEliminationKind;
    type LocalVerdict = TreeLocalVerdict;
    type StaticLocalVerdictAnalysisProof = TreeStaticProof;
    type GlobalVerdict = TreeGlobalVerdict;
    type ProcessHandler = TreeProcessHandler;
}

