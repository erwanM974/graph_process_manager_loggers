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



use std::fs::File;
use std::io::Write;
use std::path::Path;
use crate::stepstrace::printer::StepsTraceProcessPrinter;
use crate::tests::fibo_proc::conf::FiboConfig;
use crate::tests::fibo_proc::context::{FiboContext, FiboParameterization};
use crate::tests::fibo_proc::loggers::slog::object::FiboStepsTrace;
use crate::tests::fibo_proc::node::FiboNodeKind;
use crate::tests::fibo_proc::step::FiboStepKind;

pub struct FiboProcessStepPrinter {}

impl StepsTraceProcessPrinter<FiboConfig,FiboStepsTrace> for FiboProcessStepPrinter {

    fn get_initial_object(&self,
                          _context: &FiboContext,
                          _param: &FiboParameterization,
                          _node: &FiboNodeKind) -> FiboStepsTrace {
        FiboStepsTrace::new()
    }

    fn add_step_to_object(&self,
                          _context: &FiboContext,
                          _param: &FiboParameterization,
                          object: &FiboStepsTrace,
                          step: &FiboStepKind) -> FiboStepsTrace {
        let mut obj = object.clone();
        obj.trace.push(step.clone());
        obj
    }

    fn should_print_on_node_reached(&self,
                                    _context: &FiboContext,
                                    _param: &FiboParameterization,
                                    _node: &FiboNodeKind,
                                    _node_depth : u32) -> bool {
        true
    }

    fn print_object(&self,
                    _context: &FiboContext,
                    _param: &FiboParameterization,
                    object: &FiboStepsTrace,
                    path: &Path) {
        let mut file = File::create(path).unwrap();
        let as_string = object.to_string();
        let _ = file.write(as_string.as_bytes() );
    }
}