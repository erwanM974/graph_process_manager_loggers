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


use std::path::PathBuf;

use graph_process_manager_core::process::filter::GenericFiltersManager;
use graph_process_manager_core::process::manager::GenericProcessManager;
use graph_process_manager_core::queue::priorities::GenericProcessPriorities;
use graph_process_manager_core::queue::strategy::QueueSearchStrategy;
use graphviz_dot_builder::traits::GraphVizOutputFormat;

use crate::graphviz::format::GraphVizProcessLoggerLayout;
use crate::graphviz::logger::GenericGraphVizLogger;

use crate::tests::tree_proc::conf::TreeConfig;
use crate::tests::tree_proc::loggers::glog::drawer::TreeProcessDrawer;
use crate::tests::tree_proc::node::TreeNodeKind;
use crate::tests::tree_proc::priorities::TreePriorities;

use super::tree_proc::context::TreeContextAndParameterization;
use super::tree_proc::filter::{TreeNodePreFilter, TreeStepFilter};

#[test]
fn process_tree() {
    let tree_buf : PathBuf = [".", "tree"].iter().collect();
    let temp_buf : PathBuf = [".", "tree_temp"].iter().collect();

    explo_tree(tree_buf.clone(),
               temp_buf.clone(),
               "DFS".to_string(),
               QueueSearchStrategy::DFS,
               false);
    explo_tree(tree_buf.clone(),
               temp_buf.clone(),
               "BFS".to_string(),
               QueueSearchStrategy::BFS,
               false);
    explo_tree(tree_buf.clone(),
               temp_buf.clone(),
               "HCS".to_string(),
               QueueSearchStrategy::HCS,
               false);
}



fn explo_tree(tree_buf : PathBuf, temp_buf : PathBuf, name : String, queue_strategy : QueueSearchStrategy, memoize : bool) {
    let drawer = TreeProcessDrawer::new(temp_buf.into_os_string().into_string().unwrap());
    let graphic_logger : GenericGraphVizLogger<TreeConfig> = GenericGraphVizLogger::new(
        Box::new(drawer),
        GraphVizOutputFormat::svg,
        GraphVizProcessLoggerLayout::Vertical,
        true,
        tree_buf.clone().into_os_string().into_string().unwrap(),
        format!("proc_{}",name));

    let init_node = TreeNodeKind::new("O".to_string());

    let process_ctx = TreeContextAndParameterization{};
    let priorities : GenericProcessPriorities<TreePriorities> =
        GenericProcessPriorities::new(
            TreePriorities{},
            false
        );
    let filters_manager = GenericFiltersManager::new(
        vec![
            Box::new(TreeNodePreFilter::MaxProcessDepth(4)),
            ], 
        vec![], 
        vec![
            Box::new(TreeStepFilter::MaxNodeNumber(8))
        ]
    );
    let mut manager : GenericProcessManager<TreeConfig> =
        GenericProcessManager::new(
            process_ctx,
            queue_strategy,
            priorities,
            filters_manager,
            vec![Box::new(graphic_logger)],
            memoize
        );

    manager.start_process(init_node);
}