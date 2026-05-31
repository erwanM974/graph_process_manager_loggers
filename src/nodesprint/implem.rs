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

use std::fs;
use std::path::PathBuf;

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graph_process_manager_core::process::manager::GenericProcessManager;

use crate::logger::AbstractProcessLogger;
use crate::nodesprint::logger::GenericNodesPrintLogger;


impl<Conf: AbstractProcessConfiguration + 'static> AbstractProcessLogger<Conf> for GenericNodesPrintLogger<Conf> {

    fn log_initialize_process(&mut self, _manager: &GenericProcessManager<Conf>) {
        let _ = fs::remove_dir_all(&self.parent_folder);
        let _ = fs::create_dir_all(&self.parent_folder);
    }

    fn log_new_node(
        &mut self,
        context_and_param: &Conf::ContextAndParameterization,
        new_node_id: u32,
        new_node: &Conf::DomainSpecificNode,
    ) {
        if self.printer.should_print_node(context_and_param, new_node) {
            let file_name = format!("{}_node{}.{}", self.prefix, new_node_id, self.file_extension);
            let path_buf: PathBuf = [&self.parent_folder, &file_name].iter().collect();
            self.printer.print_node(context_and_param, new_node, path_buf.as_path());
        }
    }
}
