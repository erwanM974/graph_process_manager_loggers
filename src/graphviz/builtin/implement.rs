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

use std::path::PathBuf;
use graphviz_dot_builder::colors::GraphvizColor;
use graphviz_dot_builder::item::item::GraphVizGraphItem;
use graphviz_dot_builder::item::node::node::GraphVizNode;
use graphviz_dot_builder::item::node::style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape};
use crate::graphviz::builtin::builtin_process_drawer_trait::BuiltinProcessDrawer;
use crate::graphviz::format::GraphVizLoggerNodeFormat;
use crate::graphviz::process_drawer_trait::GraphVizProcessDrawer;



impl<ProcessDrawer,Conf> GraphVizProcessDrawer<Conf> for ProcessDrawer
 where
     Conf : AbstractProcessConfiguration,
    ProcessDrawer : BuiltinProcessDrawer<Conf>
{
 fn repr_static_analysis(&self) -> bool {
  self.get_proof_drawer().is_some()
 }

 fn get_temp_folder(&self) -> &str {
  self.get_temp_folder()
 }

 fn get_verdict_color(&self, local_verdict: &Conf::LocalVerdict) -> GraphvizColor {
  self.get_verdict_color(local_verdict)
 }

 fn get_static_analysis_as_gvnode_style(&self,
                                        context: &Conf::Context,
                                        param: &Conf::Parameterization,
                                        verdict: &Conf::LocalVerdict,
                                        data_proof: &Conf::StaticLocalVerdictAnalysisProof,
                                        static_analysis_name : &str
 ) -> GraphvizNodeStyle {
  let mut node_gv_options = vec![];
  node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
  node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
  let image_file_path : PathBuf = [
   self.get_temp_folder(), &format!("{}.png",static_analysis_name)
  ].iter().collect();
  // ***
  self.get_proof_drawer().unwrap().draw(
   context,
   param,
   verdict,
   data_proof,
   &image_file_path
  );
  // ***
  node_gv_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
  node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
  // **
  node_gv_options
 }

 fn get_step_gvnode_style(
  &self,
  context: &Conf::Context,
  param: &Conf::Parameterization,
  step: &Conf::StepKind,
  step_name : &str
 ) -> GraphvizNodeStyle {
  let mut node_gv_options = vec![];
  node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
  node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
  let image_file_path : PathBuf = [
   self.get_temp_folder(), &format!("{}.png",step_name)
  ].iter().collect();
  // ***
  self.get_step_drawer().draw(step,context,param,&image_file_path);
  // ***
  node_gv_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
  // **
  node_gv_options
 }


 fn get_node_as_gvcluster_style(
  &self,
  context: &Conf::Context,
  param: &Conf::Parameterization,
  new_node: &Conf::NodeKind,
  cluster_name : &str
 ) -> (GraphvizNodeStyle,Vec<Box<GraphVizGraphItem>>) {
  let cluster_gv_options = vec![
   GraphvizNodeStyleItem::FillColor( GraphvizColor::lightgrey ),
   GraphvizNodeStyleItem::Label( "".to_string() )
  ];
  let mut sub_nodes = vec![];
  for (drawer_id,drawer) in self.get_node_drawers().iter().enumerate() {
   let sub_node_name = format!("{}_drawn{}",cluster_name, drawer_id);
   let mut sub_node_style = vec![];
   let image_file_path : PathBuf = [
    self.get_temp_folder(), &format!("{}.png",&sub_node_name)
   ].iter().collect();
   // ***
   drawer.draw(new_node,context,param,&image_file_path);
   // ***
   sub_node_style.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
   sub_node_style.push(GraphvizNodeStyleItem::Label( "".to_string() ));
   sub_node_style.push(GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle));
   // ***
   let sub_gv_node = GraphVizNode::new(
    sub_node_name,
    sub_node_style
   );
   sub_nodes.push(
    Box::new(GraphVizGraphItem::Node(sub_gv_node))
   );
  }
  (cluster_gv_options,sub_nodes)
 }

 fn get_node_as_gvnode_style(
  &self,
  context: &Conf::Context,
  param: &Conf::Parameterization,
  new_node: &Conf::NodeKind,
  node_name : &str
 ) -> GraphvizNodeStyle {
  let mut node_gv_options = vec![];
  node_gv_options.push( GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle) );
  node_gv_options.push(GraphvizNodeStyleItem::FillColor( GraphvizColor::white ));
  node_gv_options.push(GraphvizNodeStyleItem::Label( "".to_string() ));
  if let Some(concrete_drawer) = self.get_node_drawers().first() {
   let image_file_path : PathBuf = [
    self.get_temp_folder(), &format!("{}.png",node_name)
   ].iter().collect();
   // ***
   (*concrete_drawer).draw(new_node,context,param,&image_file_path);
   // ***
   node_gv_options.push( GraphvizNodeStyleItem::Image( image_file_path.into_os_string().to_str().unwrap().to_string() ) );
  }
  node_gv_options
 }

 fn get_node_format(&self) -> GraphVizLoggerNodeFormat {
  if self.get_node_drawers().len() <= 1 {
   GraphVizLoggerNodeFormat::SimpleNode
  } else {
   GraphVizLoggerNodeFormat::AnchoredCluster
  }
 }
 
 fn get_node_phase_id(&self,
                         context: &<Conf as AbstractProcessConfiguration>::Context,
                         param: &<Conf as AbstractProcessConfiguration>::Parameterization,
                         new_node: &<Conf as AbstractProcessConfiguration>::NodeKind) -> Option<u32> {
        self.get_node_phase_id(context, param, new_node)
    }


    fn get_phase_color(&self, phase_id : u32) -> GraphvizColor {
        self.get_phase_color(phase_id)
    }
}
