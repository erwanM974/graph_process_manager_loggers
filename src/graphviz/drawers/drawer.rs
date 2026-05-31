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

use std::path::Path;

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graphviz_dot_builder::colors::GraphvizColor;

use crate::graphviz::item::BuiltinGraphvizLoggerItemStyle;


pub trait GraphVizProcessDrawer<Conf: AbstractProcessConfiguration> {

    /// Number of visual sub-panels to render per node.
    /// 1 (the default) produces a simple node; more than 1 produces an anchored cluster
    /// with one sub-node per view, laid out side by side.
    fn node_view_count(&self) -> usize { 1 }

    /// Draw one visual sub-panel for a node.
    /// `view_index` ranges from `0` to `node_view_count() - 1`.
    /// If the implementation writes an image to `image_file_path` it should return
    /// `BuiltinGraphvizLoggerItemStyle::CustomImage`; otherwise return a `Default` style.
    fn draw_node_view(
        &self,
        ctx: &Conf::ContextAndParameterization,
        node: &Conf::DomainSpecificNode,
        view_index: usize,
        image_file_path: &Path,
    ) -> BuiltinGraphvizLoggerItemStyle;

    fn draw_step(
        &self,
        ctx: &Conf::ContextAndParameterization,
        step: &Conf::DomainSpecificStep,
        image_file_path: &Path,
    ) -> BuiltinGraphvizLoggerItemStyle;

    fn step_edge_color(
        &self,
        ctx: &Conf::ContextAndParameterization,
        step: &Conf::DomainSpecificStep,
    ) -> GraphvizColor;

    fn draw_filter(
        &self,
        ctx: &Conf::ContextAndParameterization,
        filtration_result: &Conf::FiltrationResult,
        image_file_path: &Path,
    ) -> BuiltinGraphvizLoggerItemStyle;

    fn filter_edge_color(
        &self,
        ctx: &Conf::ContextAndParameterization,
        filtration_result: &Conf::FiltrationResult,
    ) -> GraphvizColor;

    /// Assign a phase id to a node so it is grouped into a colored background cluster.
    /// Return `None` (the default) to leave the node ungrouped.
    fn node_phase(
        &self,
        _ctx: &Conf::ContextAndParameterization,
        _node: &Conf::DomainSpecificNode,
    ) -> Option<usize> { None }

    /// Background fill color for the cluster that groups all nodes of `phase_id`.
    fn phase_color(&self, _phase_id: usize) -> GraphvizColor { GraphvizColor::white }
}
