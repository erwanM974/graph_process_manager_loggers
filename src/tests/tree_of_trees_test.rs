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

use graph_process_manager_core::delegate::delegate::GenericProcessDelegate;
use graph_process_manager_core::delegate::priorities::GenericProcessPriorities;
use graph_process_manager_core::process::manager::GenericProcessManager;
use graph_process_manager_core::queued_steps::queue::strategy::QueueSearchStrategy;
use graphviz_dot_builder::traits::GraphVizOutputFormat;

use crate::graphviz::format::GraphVizProcessLoggerLayout;
use crate::graphviz::logger::GenericGraphVizLogger;

use crate::tests::tree_of_trees_proc::conf::TreeOfTreesConfig;
use crate::tests::tree_of_trees_proc::context::{TreeOfTreesContext, TreeOfTreesParameterization};
use crate::tests::tree_of_trees_proc::filter::filter::TreeOfTreesFilter;
use crate::tests::tree_of_trees_proc::loggers::glog::drawer::TreeOfTreesProcessDrawer;
use crate::tests::tree_of_trees_proc::node::TreeOfTreesNodeKind;
use crate::tests::tree_of_trees_proc::priorities::TreeOfTreesPriorities;
use crate::tests::tree_of_trees_proc::step::TreeOfTreesStepKind;

#[test]
fn process_tree_of_trees() {
    let tree_buf : PathBuf = [".", "tree_of_trees"].iter().collect();
    let temp_buf : PathBuf = [".", "tree_of_trees_temp"].iter().collect();

    explo_tree_of_trees(tree_buf.clone(),
               temp_buf.clone(),
               "DFS".to_string(),
               QueueSearchStrategy::DFS,
               false);
}



fn explo_tree_of_trees(tree_buf : PathBuf, temp_buf : PathBuf, name : String, queue_strategy : QueueSearchStrategy, memoize : bool) {
    let drawer = TreeOfTreesProcessDrawer::new(temp_buf.into_os_string().into_string().unwrap());
    let graphic_logger : GenericGraphVizLogger<TreeOfTreesConfig> = GenericGraphVizLogger::new(
        Box::new(drawer),
        GraphVizOutputFormat::svg,
        GraphVizProcessLoggerLayout::Vertical,
        true,
        tree_buf.clone().into_os_string().into_string().unwrap(),
        format!("proc_{}",name));

    let init_node = TreeOfTreesNodeKind::new('0',None,None);

    let process_ctx = TreeOfTreesContext{};
    let priorities : GenericProcessPriorities<TreeOfTreesPriorities> =
        GenericProcessPriorities::new(TreeOfTreesPriorities{},false);
    let delegate : GenericProcessDelegate<TreeOfTreesStepKind,TreeOfTreesNodeKind,TreeOfTreesPriorities> =
        GenericProcessDelegate::new(queue_strategy,priorities);
    let mut manager : GenericProcessManager<TreeOfTreesConfig> =
        GenericProcessManager::new(process_ctx,
                                   TreeOfTreesParameterization{},
                                   delegate,
                                   vec![Box::new(TreeOfTreesFilter::MaxProcessDepth(3)),Box::new(TreeOfTreesFilter::MaxNodeNumber(7))],
                                   vec![Box::new(graphic_logger)],
                                   None,
                                   memoize);

    let (_, _) = manager.start_process(init_node);
}