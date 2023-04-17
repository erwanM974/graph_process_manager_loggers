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
use crate::nodesprint::printer::NodesPrintProcessPrinter;
use crate::tests::fibo_proc::conf::FiboConfig;
use crate::tests::fibo_proc::context::{FiboContext, FiboParameterization};
use crate::tests::fibo_proc::node::FiboNodeKind;

pub struct FiboProcessNodePrinter {}

impl NodesPrintProcessPrinter<FiboConfig> for FiboProcessNodePrinter {

    fn should_print_node(&self,
                         _context: &FiboContext,
                         _param: &FiboParameterization,
                         _node: &FiboNodeKind) -> bool {
        true
    }

    fn print_node(&self,
                  _context: &FiboContext,
                  _param: &FiboParameterization,
                  node: &FiboNodeKind,
                  path: &Path) {
        let mut file = File::create(path).unwrap();
        let as_string = node.current.to_string();
        file.write(as_string.as_bytes() );
    }
}