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




use std::hash::Hash;

use graph_process_manager_core::process::config::AbstractNodeKind;


#[derive(Clone, PartialEq, Debug, Eq, Hash)]
pub struct TreeOfTreesNodeKind {
    pub letter : char,
    pub left : Option<Box<TreeOfTreesNodeKind>>,
    pub right : Option<Box<TreeOfTreesNodeKind>>
}

impl TreeOfTreesNodeKind {

    pub fn count_letters(&self) -> u32 {
        let left_count = match &self.left {
            None => {0},
            Some(on_left) => {
                on_left.count_letters()
            }
        };
        let right_count = match &self.right {
            None => {0},
            Some(on_right) => {
                on_right.count_letters()
            }
        };
        1 + left_count + right_count
    }

    pub fn new(
        letter : char,
        left : Option<Box<TreeOfTreesNodeKind>>,
        right : Option<Box<TreeOfTreesNodeKind>>) -> Self {
        Self { letter, left, right }
    }

    pub fn add_to_the_left(&self, letter : char) -> Self {
        let new_left = match &self.left {
            None => {
                Some(Box::new(TreeOfTreesNodeKind::new(letter,None,None)))
            },
            Some(on_the_left) => {
                Some(Box::new(on_the_left.add_to_the_left(letter)))
            }
        };
        TreeOfTreesNodeKind::new(
            self.letter,
            new_left,
            self.right.clone()
        )
    }

    pub fn add_to_the_right(&self, letter : char) -> Self {
        let new_right = match &self.right {
            None => {
                Some(Box::new(TreeOfTreesNodeKind::new(letter,None,None)))
            },
            Some(on_the_right) => {
                Some(Box::new(on_the_right.add_to_the_right(letter)))
            }
        };
        TreeOfTreesNodeKind::new(
            self.letter,
            self.left.clone(),
            new_right,
        )
    }
}

impl AbstractNodeKind for TreeOfTreesNodeKind {
    fn is_included_for_memoization(&self, memoized_node: &Self) -> bool {
        memoized_node == self
    }
}

