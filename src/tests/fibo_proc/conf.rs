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
use crate::tests::fibo_proc::handler::FiboProcessHandler;
use crate::tests::fibo_proc::node::FiboNodeKind;
use crate::tests::fibo_proc::priorities::FiboPriorities;
use crate::tests::fibo_proc::step::FiboStepKind;

use super::context::{FiboContextAndParameterization, FiboFiltrationResult, FiboPersistentState};

pub struct FiboConfig {}


impl AbstractProcessConfiguration for FiboConfig {
    // ***
    type ContextAndParameterization = FiboContextAndParameterization;
    type AlgorithmOperationHandler = FiboProcessHandler;
    // ***
    type DomainSpecificNode = FiboNodeKind;
    type DomainSpecificStep = FiboStepKind;
    type Priorities = FiboPriorities;
    // ***
    type MutablePersistentState = FiboPersistentState;
    // ***
    type FiltrationResult = FiboFiltrationResult;
}

