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


use std::collections::{HashMap, HashSet};
use graph_process_manager_core::manager::config::AbstractProcessConfiguration;
use crate::stepstrace::object::ObjectToBuildWhenTracingSteps;

use crate::stepstrace::printer::StepsTraceProcessPrinter;


pub struct GenericStepsTraceLogger<Conf : AbstractProcessConfiguration,
            ObjectToBuild : ObjectToBuildWhenTracingSteps> {
    // ***
    pub(crate) printer : Box<dyn StepsTraceProcessPrinter<Conf, ObjectToBuild>>,
    // ***
    // maps node id u32 to objects to build
    // the initial node of id 1 has an initial object
    // subsequents objects are build progressively once nodes are reached from the initial node of id 1
    // there may be several objects associated to a single node id
    // because there may be several distinct paths to the same node if memoization is used in the process
    pub(crate) trace_map : HashMap<u32,HashSet<ObjectToBuild>>,
    // ***
    pub(crate) prefix : String,
    pub(crate) file_extension : String,
    pub(crate) parent_folder : String,
    // ***
    pub(crate) trace_counter : u32
}

impl<Conf: AbstractProcessConfiguration,
        ObjectToBuild: ObjectToBuildWhenTracingSteps>
            GenericStepsTraceLogger<Conf, ObjectToBuild> {
    pub fn new(printer: Box<dyn StepsTraceProcessPrinter<Conf, ObjectToBuild>>,
               prefix: String, file_extension: String,
               parent_folder: String) -> Self {
        Self { printer, trace_map : hashmap!{}, prefix, file_extension, parent_folder, trace_counter:0 }
    }
}







