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

use graph_process_manager_core::{process::{config::AbstractProcessConfiguration, filter::{AbstractNodePostFilter, AbstractNodePreFilter, AbstractStepFilter, GenericFiltersManager}}, queue::{priorities::GenericProcessPriorities, strategy::QueueSearchStrategy}};
use graphviz_dot_builder::item::node::{node::GraphVizNode, style::{GraphvizNodeStyle, GraphvizNodeStyleItem, GvNodeShape, GvNodeStyleKind}};



pub trait ProcessLegendWriter<Conf : AbstractProcessConfiguration + 'static> {

    /** 
     * Returns a title describing the whole process.
     * **/
     fn get_process_description(&self) -> String;

     /** 
      * Returns the description of individual relevant parameters.
      * Each individual parameter may be written on several lines (inner vec)
      * **/
     fn get_parameters_description(&self, context_and_param : &Conf::ContextAndParameterization) -> Vec<Vec<String>>;


     /** 
      * Returns the description of priorities configuration.
      * **/
     fn get_priorities_description(&self, priorities : &Conf::Priorities) -> Vec<Vec<String>>;


     /** 
      * Returns the description of a given step filter.
      * **/
     fn get_step_filter_description(&self, filter : &dyn AbstractStepFilter<Conf>) -> Option<Vec<String>>;


     /** 
      * Returns the description of a given node pre filter.
      * **/
     fn get_node_pre_filter_description(&self, filter : &dyn AbstractNodePreFilter<Conf>) -> Option<Vec<String>>;


     /** 
      * Returns the description of a given node post filter.
      * **/
     fn get_node_post_filter_description(&self, filter : &dyn AbstractNodePostFilter<Conf>) -> Option<Vec<String>>;

     fn get_legend_node(
        &self, 
        context_and_param: &Conf::ContextAndParameterization,
        strategy: &QueueSearchStrategy,
        priorities: &GenericProcessPriorities<Conf::Priorities>,
        filters_manager : &GenericFiltersManager<Conf>,
        use_memoization : bool
    ) -> GraphVizNode {
    
        let mut label_lines : Vec<String> = Vec::new();
        // ***
        label_lines.push(self.get_process_description().to_owned());
        // ***
        {
            label_lines.push( "  parameters=[".to_string() );
            let parameters_descs = self.get_parameters_description(context_and_param);
            let parameters_number = parameters_descs.len();
            for (param_id, param_dsc) in parameters_descs.into_iter().enumerate() {
                let that_one_param_lines_num = param_dsc.len();
                for (pr_line_id,pr_line) in param_dsc.into_iter().enumerate() {
                    if (pr_line_id == that_one_param_lines_num - 1) && (param_id < parameters_number - 1) {
                        label_lines.push( format!("   {},", pr_line) );
                    } else {
                        label_lines.push( format!("   {}", pr_line) );
                    }
                }
            }
            label_lines.push( "  ];".to_string() );
        }
        // ***
        label_lines.push( format!("  strategy={};", strategy) );
        // ***
        {
            label_lines.push( "  priorities=[".to_string() );
            label_lines.push( format!("    randomize={},", priorities.randomize) );
            let priorities_descs = self.get_priorities_description(&priorities.domain_specific);
            let priorities_number = priorities_descs.len();
            for (p_id, p_dsc) in priorities_descs.into_iter().enumerate() {
                let that_one_p_lines_num = p_dsc.len();
                for (pr_line_id,pr_line) in p_dsc.into_iter().enumerate() {
                    if (pr_line_id == that_one_p_lines_num - 1) && (p_id < priorities_number - 1) {
                        label_lines.push( format!("    {},", pr_line) );
                    } else {
                        label_lines.push( format!("    {}", pr_line) );
                    }
                }
            }
            label_lines.push( "  ];".to_string() );
        }
        // ***
        label_lines.push( format!("  memoize={}:", use_memoization) );
        // ***
        {
            label_lines.push( "  step_filters=[".to_string() );
            let filters = filters_manager.get_step_filters();
            let filters_num = filters.len();
            for (filter_id, filter) in filters.iter().enumerate() {
                match self.get_step_filter_description(&**filter) {
                    None => {
                        if filter_id < filters_num - 1 {
                            label_lines.push( "    unknown filter,".to_owned() );
                        } else {
                            label_lines.push( "    unknown filter".to_owned() );
                        }
                    },
                    Some(that_one_filter_desc) => {
                        let that_one_f_lines_num = that_one_filter_desc.len();
                        for (f_line_id,f_line) in that_one_filter_desc.into_iter().enumerate() {
                            if (f_line_id == that_one_f_lines_num - 1) && (filter_id < filters_num - 1) {
                                label_lines.push( format!("    {},", f_line) );
                            } else {
                                label_lines.push( format!("    {}", f_line) );
                            }
                        }
                    }
                }
            }
            label_lines.push( "  ];".to_string() );
        }
        // ***
        {
            label_lines.push( "  node_pre_filters=[".to_string() );
            let filters = filters_manager.get_node_pre_filters();
            let filters_num = filters.len();
            for (filter_id, filter) in filters.iter().enumerate() {
                match self.get_node_pre_filter_description(&**filter) {
                    None => {
                        if filter_id < filters_num - 1 {
                            label_lines.push( "    unknown filter,".to_owned() );
                        } else {
                            label_lines.push( "    unknown filter".to_owned() );
                        }
                    },
                    Some(that_one_filter_desc) => {
                        let that_one_f_lines_num = that_one_filter_desc.len();
                        for (f_line_id,f_line) in that_one_filter_desc.into_iter().enumerate() {
                            if (f_line_id == that_one_f_lines_num - 1) && (filter_id < filters_num - 1) {
                                label_lines.push( format!("    {},", f_line) );
                            } else {
                                label_lines.push( format!("    {}", f_line) );
                            }
                        }
                    }
                }
            }
            label_lines.push( "  ];".to_string() );
        }
        // ***
        {
            label_lines.push( "  node_post_filters=[".to_string() );
            let filters = filters_manager.get_node_post_filters();
            let filters_num = filters.len();
            for (filter_id, filter) in filters.iter().enumerate() {
                match self.get_node_post_filter_description(&**filter) {
                    None => {
                        if filter_id < filters_num - 1 {
                            label_lines.push( "    unknown filter,".to_owned() );
                        } else {
                            label_lines.push( "    unknown filter".to_owned() );
                        }
                    },
                    Some(that_one_filter_desc) => {
                        let that_one_f_lines_num = that_one_filter_desc.len();
                        for (f_line_id,f_line) in that_one_filter_desc.into_iter().enumerate() {
                            if (f_line_id == that_one_f_lines_num - 1) && (filter_id < filters_num - 1) {
                                label_lines.push( format!("    {},", f_line) );
                            } else {
                                label_lines.push( format!("    {}", f_line) );
                            }
                        }
                    }
                }
            }
            label_lines.push( "  ];".to_string() );
        }
        // ***
        let legend_style : GraphvizNodeStyle = vec![
            GraphvizNodeStyleItem::Label( label_lines.join(r"\l") + r"\l" ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
            GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Bold,GvNodeStyleKind::Rounded]),
            GraphvizNodeStyleItem::FontSize( 18 )];
            
        GraphVizNode::new(
            "legend".to_string(),
            legend_style
        )
    }
    


    
    fn get_final_global_state_description_for_legend(
        &self, 
        context_and_param: &Conf::ContextAndParameterization,
        final_state : &Conf::MutablePersistentState
    ) -> Vec<String>;

    fn get_verdict_node(
        &self,
        context_and_param: &Conf::ContextAndParameterization,
        final_global_state : &Conf::MutablePersistentState
    ) -> GraphVizNode {
        let legs = self.get_final_global_state_description_for_legend(context_and_param,final_global_state);
        let final_legend_style : GraphvizNodeStyle = vec![
            GraphvizNodeStyleItem::Label( legs.join(r"\l") + r"\l" ),
            GraphvizNodeStyleItem::Shape(GvNodeShape::Rectangle),
            GraphvizNodeStyleItem::Style(vec![GvNodeStyleKind::Bold,GvNodeStyleKind::Rounded]),
            GraphvizNodeStyleItem::FontSize( 18 )];
        // ***
        GraphVizNode::new(
            "verdict".to_string(),
            final_legend_style
        )
    }
    

}


