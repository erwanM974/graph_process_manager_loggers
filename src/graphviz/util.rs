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



pub fn get_anchor_id(prefix : &str, id: u32) -> String {
    format!("{:}_a{:}", prefix, id)
}

pub fn get_node_id(prefix : &str, id: u32) -> String {
    format!("{:}_n{:}", prefix, id)
}

pub fn get_filtration_id(prefix : &str, id: u32) -> String {
    format!("{:}_f{:}", prefix, id)
}

pub fn get_step_id(prefix : &str,
                   origin_id: u32,
                   target_id: u32) -> String {
    format!("{:}_s_{:}_{:}", prefix, origin_id, target_id)
}

