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



use graphviz_dot_builder::colors::GraphvizColor;
use crate::graphviz::builtin::builtin_process_drawer_trait::BuiltinProcessDrawer;
use crate::graphviz::builtin::node_drawer::CustomNodeDrawerForGraphvizLogger;
use crate::graphviz::builtin::proof_drawer::CustomProofDrawerForGraphvizLogger;
use crate::graphviz::builtin::step_drawer::CustomStepDrawerForGraphvizLogger;
use crate::tests::tree_of_trees_proc::conf::TreeOfTreesConfig;
use crate::tests::tree_of_trees_proc::context::{TreeOfTreesContext, TreeOfTreesParameterization};
use crate::tests::tree_of_trees_proc::loggers::glog::node_drawer::TreeOfTreesNodeDrawer;
use crate::tests::tree_of_trees_proc::loggers::glog::step_drawer::TreeOfTreesStepDrawer;
use crate::tests::tree_of_trees_proc::node::TreeOfTreesNodeKind;
use crate::tests::tree_of_trees_proc::verdict::local::TreeOfTreesLocalVerdict;

pub struct TreeOfTreesProcessDrawer {
    pub temp_folder : String,
    pub node_drawers : Vec<Box<
        dyn CustomNodeDrawerForGraphvizLogger<TreeOfTreesConfig>
    >>,
    pub step_drawer : Box<
        dyn CustomStepDrawerForGraphvizLogger<TreeOfTreesConfig>
    >,
    pub phases_colors : Vec<GraphvizColor>,
}

impl TreeOfTreesProcessDrawer {
    pub fn new(temp_folder: String) -> Self {
        let drawer : Box<dyn CustomNodeDrawerForGraphvizLogger<TreeOfTreesConfig>>
         = Box::new(TreeOfTreesNodeDrawer{});
        let node_drawers = vec![drawer];
        let step_drawer : Box<dyn CustomStepDrawerForGraphvizLogger<TreeOfTreesConfig>> = Box::new(
            TreeOfTreesStepDrawer::new()
        ) ;
        let phases_colors = vec![
            GraphvizColor::lightskyblue,
            GraphvizColor::lightgoldenrod1,
            GraphvizColor::seagreen1,
            GraphvizColor::lightsalmon
        ];
        TreeOfTreesProcessDrawer { temp_folder, node_drawers, step_drawer, phases_colors }
    }
}


impl BuiltinProcessDrawer<TreeOfTreesConfig> for TreeOfTreesProcessDrawer {
    fn get_node_drawers(&self) -> &Vec<Box<dyn CustomNodeDrawerForGraphvizLogger<TreeOfTreesConfig>>> {
        &self.node_drawers
    }

    fn get_step_drawer(&self) -> &Box<dyn CustomStepDrawerForGraphvizLogger<TreeOfTreesConfig>> {
        &self.step_drawer
    }

    fn get_proof_drawer(&self) -> Option<&Box<dyn CustomProofDrawerForGraphvizLogger<TreeOfTreesConfig>>> {
        None
    }

    fn get_temp_folder(&self) -> &str {
        &self.temp_folder
    }

    fn get_verdict_color(&self, _local_verdict: &TreeOfTreesLocalVerdict) -> GraphvizColor {
        GraphvizColor::black
    }
    
    fn get_node_phase_id(&self,
        _context: &TreeOfTreesContext,
        _param: &TreeOfTreesParameterization,
        new_node: &TreeOfTreesNodeKind) -> Option<u32> {
        let count = new_node.count_letters();
        Some(count/2)
    }
    
    fn get_phase_color(&self, phase_id : u32) -> GraphvizColor {
        self.phases_colors.get((phase_id as usize) % self.phases_colors.len()).unwrap().clone()
    }
}