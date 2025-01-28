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
use std::path::{Path, PathBuf};
use std::process::Command;
use graphviz_dot_builder::edge::edge::GraphVizEdge;
use graphviz_dot_builder::edge::style::{GraphvizEdgeStyleItem, GvArrowHeadStyle};
use graphviz_dot_builder::graph::graph::GraphVizDiGraph;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::GraphvizNodeStyleItem;
use graphviz_dot_builder::traits::{DotBuildable, DotTranslatable};
use crate::graphviz::builtin::node_drawer::CustomNodeDrawerForGraphvizLogger;
use crate::tests::tree_of_trees_proc::conf::TreeOfTreesConfig;
use crate::tests::tree_of_trees_proc::context::{TreeOfTreesContext, TreeOfTreesParameterization};
use crate::tests::tree_of_trees_proc::node::TreeOfTreesNodeKind;


pub struct TreeOfTreesNodeDrawer {}

impl CustomNodeDrawerForGraphvizLogger<TreeOfTreesConfig> for TreeOfTreesNodeDrawer {

    fn draw(&self,
            node : &TreeOfTreesNodeKind,
            _context: &TreeOfTreesContext,
            _parameterization: &TreeOfTreesParameterization,
            full_path : &Path) {
        // ***
        let temp_file_name = "temp.dot";
        let temp_path : PathBuf = [&temp_file_name].iter().collect();
        let mut file = File::create(temp_path.as_path()).unwrap();
        let _ = file.write( term_gv_repr(node).to_dot_string().as_bytes() );
        // ***
        let _ = Command::new("dot")
            .arg("-Tpng")
            .arg(temp_path.as_path())
            .arg("-o")
            .arg(full_path)
            .output();
    }

}




fn term_gv_repr(tree : &TreeOfTreesNodeKind) -> GraphVizDiGraph
{
    let mut digraph = GraphVizDiGraph::new(vec![]);
    term_gv_repr_rec(tree,&mut 0, &mut digraph);
    return digraph;
}


fn term_gv_repr_rec(
    tree : &TreeOfTreesNodeKind,
    current_id : &mut u32,
    gv_graph : &mut GraphVizDiGraph) -> String
{
    let node_name = format!("p{:}",current_id);
    *current_id += 1;
    // the parent node
    {
        let node_style = vec![
            GraphvizNodeStyleItem::Label(format!("{}",tree.letter))
        ];
        gv_graph.add_node( GraphVizNode::new(node_name.clone(), node_style) );
    }
    // the child nodes
    for (x,child_opt) in vec![("left",&tree.left), ("right",&tree.right)] {
        if let Some(child) = child_opt {
            let child_node_name = term_gv_repr_rec(
                child,
                current_id,
                gv_graph
            );
            let gv_edge = GraphVizEdge::new(node_name.clone(),
                                            None,
                                            child_node_name,
                                            None,
                                            vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]);
            gv_graph.add_edge(gv_edge);
        } else {
            let sub_node_name = format!("{}_{}",node_name, x);
            let sub_node_style = vec![
                GraphvizNodeStyleItem::Label(format!("."))
            ];
            gv_graph.add_node( GraphVizNode::new(sub_node_name.clone(), sub_node_style) );
            let gv_edge = GraphVizEdge::new(node_name.clone(),
                                            None,
                                            sub_node_name,
                                            None,
                                            vec![ GraphvizEdgeStyleItem::Head( GvArrowHeadStyle::NoArrow )]);
            gv_graph.add_edge(gv_edge);
        }
    }
    return node_name;
}