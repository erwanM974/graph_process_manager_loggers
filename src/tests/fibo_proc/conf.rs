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
use crate::tests::fibo_proc::context::{FiboContext, FiboParameterization};
use crate::tests::fibo_proc::filter::elim::FiboFilterEliminationKind;
use crate::tests::fibo_proc::filter::filter::FiboFilterCriterion;
use crate::tests::fibo_proc::handler::FiboProcessHandler;
use crate::tests::fibo_proc::node::FiboNodeKind;
use crate::tests::fibo_proc::priorities::FiboPriorities;
use crate::tests::fibo_proc::step::FiboStepKind;
use crate::tests::fibo_proc::verdict::global::FiboGlobalVerdict;
use crate::tests::fibo_proc::verdict::local::FiboLocalVerdict;

pub struct FiboConfig {}

pub struct FiboStaticProof {}

impl AbstractProcessConfiguration for FiboConfig {
    type Context = FiboContext;
    type Parameterization = FiboParameterization;
    type NodeKind = FiboNodeKind;
    type StepKind = FiboStepKind;
    type Priorities = FiboPriorities;
    type FilterCriterion = FiboFilterCriterion;
    type FilterEliminationKind = FiboFilterEliminationKind;
    type LocalVerdict = FiboLocalVerdict;
    type StaticLocalVerdictAnalysisProof = FiboStaticProof;
    type GlobalVerdict = FiboGlobalVerdict;
    type ProcessHandler = FiboProcessHandler;
}

