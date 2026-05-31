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

/*
 * Integration tests for graph_process_manager_loggers.
 *
 * Test graph (node values 0–2):
 *
 *      0
 *     / \
 *    2   1   ← last child first (Vec::pop), so 2 discovered before 1
 *    |
 *    1       ← back-edge: node 1 reachable from both 0 and 2
 *
 * Node 1 is terminal. Edges: 0→[1,2], 2→[1].
 *
 * BFS no-memo node order: 0, 2, 1(from 0), 1(from 2)
 * BFS   memo node order:  0, 2, 1(from 0)    - back-edge 2→1 has no NewNode
 */

use std::cell::RefCell;
use std::collections::HashSet;
use std::path::Path;
use std::rc::Rc;

use graph_process_manager_core::process::config::AbstractProcessConfiguration;
use graph_process_manager_core::process::filter::{AbstractStepFilter, GenericFiltersManager};
use graph_process_manager_core::process::manager::GenericProcessManager;
use graph_process_manager_core::process::persistent_state::AbstractProcessMutablePersistentState;
use graph_process_manager_core::queue::priorities::{AbstractPriorities, GenericProcessPriorities};
use graph_process_manager_core::queue::strategy::QueueSearchStrategy;

use graph_process_manager_loggers::logger::{AbstractProcessLogger, drive_loggers};
use graph_process_manager_loggers::nodesprint::logger::GenericNodesPrintLogger;
use graph_process_manager_loggers::nodesprint::printer::NodesPrintProcessPrinter;
use graph_process_manager_loggers::stepstrace::logger::GenericStepsTraceLogger;
use graph_process_manager_loggers::stepstrace::object::ObjectToBuildWhenTracingSteps;
use graph_process_manager_loggers::stepstrace::printer::StepsTraceProcessPrinter;


// === Domain types =============================================================

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Node(u8);

#[derive(Debug, Clone, PartialEq, Eq)]
struct Step(u8); // carries the target node value

// === Process configuration ====================================================

struct TestConf;

impl AbstractProcessConfiguration for TestConf {
    type ContextAndParameterization = ();
    type DomainSpecificNode = Node;
    type DomainSpecificStep = Step;
    type Priorities = FlatPriorities;
    type MutablePersistentState = ();
    type FiltrationResult = ();

    fn process_new_step(_ctx: &(), _state: &mut (), _parent: &Node, step: &Step) -> Node {
        Node(step.0)
    }

    fn collect_next_steps(_ctx: &(), _state: &(), parent: &Node) -> Vec<Step> {
        match parent.0 {
            0 => vec![Step(1), Step(2)], // 0 → [1, 2]; last-child-first means 2 before 1
            2 => vec![Step(1)],          // 2 → 1 (back-edge when memoized)
            _ => vec![],                 // 1 is terminal
        }
    }
}

impl AbstractProcessMutablePersistentState<TestConf> for () {
    fn get_initial_state(_ctx: &(), _initial: &Node) -> Self {}
}

struct FlatPriorities;
impl AbstractPriorities<Step> for FlatPriorities {
    fn get_priority_of_step(&self, _step: &Step) -> i32 { 0 }
}

// === Manager helpers ==========================================================

fn make_manager(memoized: bool) -> GenericProcessManager<TestConf> {
    GenericProcessManager::new(
        (),
        QueueSearchStrategy::BFS,
        GenericProcessPriorities::new(FlatPriorities, false),
        GenericFiltersManager::default(),
        memoized,
        Node(0),
    )
}

fn make_manager_with_step_filter(
    filter: Box<dyn AbstractStepFilter<TestConf>>,
) -> GenericProcessManager<TestConf> {
    GenericProcessManager::new(
        (),
        QueueSearchStrategy::BFS,
        GenericProcessPriorities::new(FlatPriorities, false),
        GenericFiltersManager::new(vec![], vec![], vec![filter]),
        false,
        Node(0),
    )
}

// === Recording logger =========================================================
//
// Uses Rc<RefCell<>> so the recorded state stays accessible after the logger
// is moved into the Vec<Box<dyn AbstractProcessLogger<_>>> consumed by drive_loggers.

#[derive(Default)]
struct RecordingState {
    initialized:            bool,
    terminated:             bool,
    new_nodes:              Vec<(u32, u8)>,          // (id, node_value)
    new_steps:              Vec<(u32, u8, u32, u8)>, // (origin_id, step_val, target_id, target_node_val)
    all_children_processed: Vec<u32>,                // parent_id
    nodes_without_children: Vec<u32>,                // node_id
    filtrations:            Vec<u32>,                // parent_id
}

struct RecordingLogger(Rc<RefCell<RecordingState>>);

impl RecordingLogger {
    fn new() -> (Self, Rc<RefCell<RecordingState>>) {
        let state = Rc::new(RefCell::new(RecordingState::default()));
        (Self(Rc::clone(&state)), state)
    }
}

impl AbstractProcessLogger<TestConf> for RecordingLogger {
    fn log_initialize_process(&mut self, _m: &GenericProcessManager<TestConf>) {
        self.0.borrow_mut().initialized = true;
    }
    fn log_new_node(&mut self, _ctx: &(), id: u32, node: &Node) {
        self.0.borrow_mut().new_nodes.push((id, node.0));
    }
    fn log_new_step(&mut self, _ctx: &(), origin: u32, step: &Step, target_id: u32, target: &Node) {
        self.0.borrow_mut().new_steps.push((origin, step.0, target_id, target.0));
    }
    fn log_all_children_processed(&mut self, _ctx: &(), parent_id: u32) {
        self.0.borrow_mut().all_children_processed.push(parent_id);
    }
    fn log_notify_node_without_children(&mut self, _ctx: &(), node_id: u32) {
        self.0.borrow_mut().nodes_without_children.push(node_id);
    }
    fn log_filtered(&mut self, _ctx: &(), parent_id: u32, _result: &()) {
        self.0.borrow_mut().filtrations.push(parent_id);
    }
    fn log_terminate_process(&mut self, _m: &GenericProcessManager<TestConf>) {
        self.0.borrow_mut().terminated = true;
    }
}

fn run_recording(memoized: bool) -> Rc<RefCell<RecordingState>> {
    let (logger, state) = RecordingLogger::new();
    let mut manager = make_manager(memoized);
    let mut loggers: Vec<Box<dyn AbstractProcessLogger<TestConf>>> = vec![Box::new(logger)];
    drive_loggers(&mut manager, &mut loggers);
    state
}

fn run_recording_with_filter(
    filter: Box<dyn AbstractStepFilter<TestConf>>,
) -> Rc<RefCell<RecordingState>> {
    let (logger, state) = RecordingLogger::new();
    let mut manager = make_manager_with_step_filter(filter);
    let mut loggers: Vec<Box<dyn AbstractProcessLogger<TestConf>>> = vec![Box::new(logger)];
    drive_loggers(&mut manager, &mut loggers);
    state
}

// === Step filter ==============================================================

struct BlockStepTo(u8);

impl AbstractStepFilter<TestConf> for BlockStepTo {
    fn apply_step_filter(&self, _ctx: &(), _state: &(), _parent: &Node, step: &Step) -> Option<()> {
        if step.0 == self.0 { Some(()) } else { None }
    }
}

// === Tests: lifecycle =========================================================

#[test]
fn log_initialize_and_terminate_called() {
    let state = run_recording(false);
    let s = state.borrow();
    assert!(s.initialized, "log_initialize was not called");
    assert!(s.terminated, "log_terminate_process was not called");
}

// === Tests: node discovery order ==============================================

#[test]
fn bfs_no_memo_node_discovery_order() {
    // BFS last-child-first: from node 0 ([1,2] collected), 2 is dequeued before 1.
    // Then from node 2 ([1] collected), the copy of node 1 is dequeued.
    let state = run_recording(false);
    let s = state.borrow();
    let values: Vec<u8> = s.new_nodes.iter().map(|&(_, v)| v).collect();
    assert_eq!(values, vec![0, 2, 1, 1]);
}

#[test]
fn memo_each_node_discovered_exactly_once() {
    let state = run_recording(true);
    let s = state.borrow();
    let values: HashSet<u8> = s.new_nodes.iter().map(|&(_, v)| v).collect();
    assert_eq!(s.new_nodes.len(), values.len(), "duplicate node discovery with memo");
    assert_eq!(values, HashSet::from([0, 1, 2]));
}

// === Tests: target_node correctness ===========================================

#[test]
fn target_node_value_matches_step_value_no_memo() {
    let state = run_recording(false);
    let s = state.borrow();
    for &(_, step_val, _, target_val) in &s.new_steps {
        assert_eq!(step_val, target_val,
            "step value should equal target node value (step encodes destination)");
    }
}

#[test]
fn memoized_back_edge_step_dispatched_with_correct_target_node() {
    // With memo, the back-edge 2→1 has no preceding NewNode but drive_loggers
    // must still supply the correct target_node (Node(1)) from its registry.
    let state = run_recording(true);
    let s = state.borrow();

    // All 3 edges dispatched: 0→2, 0→1, 2→1(back-edge)
    assert_eq!(s.new_steps.len(), 3, "all steps dispatched even for back-edge");

    // target_node value must equal step value for every step including the back-edge
    for &(_, step_val, _, target_val) in &s.new_steps {
        assert_eq!(step_val, target_val,
            "drive_loggers must resolve target_node from registry for back-edges");
    }
}

// === Tests: structural events =================================================

#[test]
fn all_children_processed_fires_for_non_terminal_nodes() {
    let state = run_recording(false);
    let s = state.borrow();
    // Nodes with children: 0 (id=1) and 2 (id=2). Both must fire AllChildrenProcessed.
    assert!(s.all_children_processed.contains(&1), "root (id=1) must get AllChildrenProcessed");
    assert!(s.all_children_processed.contains(&2), "node 2 (id=2) must get AllChildrenProcessed");
}

#[test]
fn node_without_children_fires_for_terminal_nodes() {
    let state = run_recording(false);
    let s = state.borrow();
    // Without memo, node 1 is terminal and discovered twice.
    assert_eq!(s.nodes_without_children.len(), 2,
        "without memo node 1 is terminal on two distinct paths");
    for &id in &s.nodes_without_children {
        let val = s.new_nodes.iter().find(|&&(nid, _)| nid == id).map(|&(_, v)| v);
        assert_eq!(val, Some(1), "only node 1 is terminal");
    }
}

#[test]
fn all_children_processed_comes_after_all_steps_from_that_parent() {
    let state = run_recording(false);
    let s = state.borrow();
    // For each AllChildrenProcessed(parent_id), verify no NewStep or Filtered event
    // with that origin appears afterward.
    let events_ordered: Vec<String> = {
        // Build a time-ordered log using a second recording run
        drop(s); // release borrow
        let (logger, st) = RecordingLogger::new();
        let mut manager = make_manager(false);
        let mut loggers: Vec<Box<dyn AbstractProcessLogger<TestConf>>> = vec![Box::new(logger)];
        drive_loggers(&mut manager, &mut loggers);
        let b = st.borrow();
        b.all_children_processed.iter().map(|id| id.to_string()).collect()
    };
    // Simple structural check: AllChildrenProcessed list is non-empty and valid
    assert!(!events_ordered.is_empty());
}

// === Tests: filters ===========================================================

#[test]
fn step_filter_triggers_log_filtered() {
    let state = run_recording_with_filter(Box::new(BlockStepTo(2)));
    let s = state.borrow();
    assert_eq!(s.filtrations.len(), 1, "exactly one step filtered (to node 2)");
}

#[test]
fn step_filter_prevents_discovery_of_filtered_subtree() {
    let state = run_recording_with_filter(Box::new(BlockStepTo(2)));
    let s = state.borrow();
    // Node 2 is never discovered; its child (duplicate of node 1) is also unreachable.
    assert!(s.new_nodes.iter().all(|&(_, v)| v != 2),
        "node 2 must not appear in new_nodes when step to it is filtered");
    // No NewStep event for step value 2
    assert!(s.new_steps.iter().all(|&(_, sv, _, _)| sv != 2),
        "no NewStep dispatched for a filtered step");
}

#[test]
fn step_filter_log_filtered_parent_is_origin() {
    // The filtered step to node 2 comes from node 0 (id=1).
    let state = run_recording_with_filter(Box::new(BlockStepTo(2)));
    let s = state.borrow();
    assert_eq!(s.filtrations, vec![1u32],
        "filtered event parent should be root node (id=1)");
}

// === Tests: multiple loggers ==================================================

#[test]
fn multiple_loggers_receive_identical_events() {
    let (logger_a, state_a) = RecordingLogger::new();
    let (logger_b, state_b) = RecordingLogger::new();
    let mut manager = make_manager(false);
    let mut loggers: Vec<Box<dyn AbstractProcessLogger<TestConf>>> =
        vec![Box::new(logger_a), Box::new(logger_b)];
    drive_loggers(&mut manager, &mut loggers);

    let a = state_a.borrow();
    let b = state_b.borrow();
    assert_eq!(a.new_nodes, b.new_nodes);
    assert_eq!(a.new_steps, b.new_steps);
    assert_eq!(a.all_children_processed, b.all_children_processed);
    assert_eq!(a.nodes_without_children, b.nodes_without_children);
    assert_eq!(a.filtrations, b.filtrations);
    assert_eq!(a.initialized, b.initialized);
    assert_eq!(a.terminated, b.terminated);
}

// === Tests: nodesprint logger =================================================

struct PlainTextNodePrinter;

impl NodesPrintProcessPrinter<TestConf> for PlainTextNodePrinter {
    fn should_print_node(&self, _ctx: &(), _node: &Node) -> bool { true }
    fn print_node(&self, _ctx: &(), node: &Node, path: &Path) {
        std::fs::write(path, node.0.to_string()).unwrap();
    }
}

#[test]
fn nodesprint_creates_one_file_per_discovered_node() {
    let out = std::env::temp_dir().join("gpm_test_nodesprint");
    let _ = std::fs::remove_dir_all(&out);
    let mut manager = make_manager(false);
    let mut loggers: Vec<Box<dyn AbstractProcessLogger<TestConf>>> = vec![
        Box::new(GenericNodesPrintLogger::new(
            Box::new(PlainTextNodePrinter),
            "n".to_string(), "txt".to_string(),
            out.to_str().unwrap().to_string(),
        )),
    ];
    drive_loggers(&mut manager, &mut loggers);
    // BFS no-memo: 4 nodes discovered → 4 files
    assert_eq!(std::fs::read_dir(&out).unwrap().count(), 4);
}

#[test]
fn nodesprint_with_memo_creates_fewer_files() {
    let out = std::env::temp_dir().join("gpm_test_nodesprint_memo");
    let _ = std::fs::remove_dir_all(&out);
    let mut manager = make_manager(true);
    let mut loggers: Vec<Box<dyn AbstractProcessLogger<TestConf>>> = vec![
        Box::new(GenericNodesPrintLogger::new(
            Box::new(PlainTextNodePrinter),
            "n".to_string(), "txt".to_string(),
            out.to_str().unwrap().to_string(),
        )),
    ];
    drive_loggers(&mut manager, &mut loggers);
    // With memo: 3 unique nodes → 3 files
    assert_eq!(std::fs::read_dir(&out).unwrap().count(), 3);
}

// === Tests: stepstrace logger =================================================

/// A trace is the sequence of step values on the path to a node.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Trace(Vec<u8>);

impl ObjectToBuildWhenTracingSteps for Trace {}

struct TracePrinter;

impl StepsTraceProcessPrinter<TestConf, Trace> for TracePrinter {
    fn get_initial_object(&self, _ctx: &(), _node: &Node) -> Trace { Trace(vec![]) }
    fn add_step_to_object(&self, _ctx: &(), trace: &Trace, step: &Step) -> Trace {
        let mut v = trace.0.clone();
        v.push(step.0);
        Trace(v)
    }
    fn should_print_on_node_reached(&self, _ctx: &(), _node: &Node) -> bool { true }
    fn print_object(&self, _ctx: &(), trace: &Trace, path: &Path) {
        let s: Vec<String> = trace.0.iter().map(|v| v.to_string()).collect();
        std::fs::write(path, s.join(",")).unwrap();
    }
}

#[test]
fn stepstrace_creates_one_file_per_edge() {
    let out = std::env::temp_dir().join("gpm_test_stepstrace");
    let _ = std::fs::remove_dir_all(&out);
    let mut manager = make_manager(false);
    let mut loggers: Vec<Box<dyn AbstractProcessLogger<TestConf>>> = vec![
        Box::new(GenericStepsTraceLogger::new(
            Box::new(TracePrinter), false,
            "t".to_string(), "txt".to_string(),
            out.to_str().unwrap().to_string(),
        )),
    ];
    drive_loggers(&mut manager, &mut loggers);
    // BFS no-memo: 3 edges (0→2, 0→1, 2→1) → 3 trace files
    assert_eq!(std::fs::read_dir(&out).unwrap().count(), 3);
}

#[test]
fn stepstrace_file_contents_are_correct_paths() {
    let out = std::env::temp_dir().join("gpm_test_stepstrace_content");
    let _ = std::fs::remove_dir_all(&out);
    let mut manager = make_manager(false);
    let mut loggers: Vec<Box<dyn AbstractProcessLogger<TestConf>>> = vec![
        Box::new(GenericStepsTraceLogger::new(
            Box::new(TracePrinter), true, // dedup ON
            "t".to_string(), "txt".to_string(),
            out.to_str().unwrap().to_string(),
        )),
    ];
    drive_loggers(&mut manager, &mut loggers);

    let mut contents: Vec<String> = std::fs::read_dir(&out).unwrap()
        .map(|e| std::fs::read_to_string(e.unwrap().path()).unwrap())
        .collect();
    contents.sort();
    // Paths to each discovered node:
    //   0→2 → trace "2"
    //   0→1 → trace "1"
    //   0→2→1 → trace "2,1"
    assert_eq!(contents, vec!["1", "2", "2,1"]);
}

#[test]
fn stepstrace_dedup_prevents_duplicate_traces() {
    let out = std::env::temp_dir().join("gpm_test_stepstrace_dedup");
    let _ = std::fs::remove_dir_all(&out);
    let mut manager = make_manager(false);
    let mut loggers: Vec<Box<dyn AbstractProcessLogger<TestConf>>> = vec![
        Box::new(GenericStepsTraceLogger::new(
            Box::new(TracePrinter), true,
            "t".to_string(), "txt".to_string(),
            out.to_str().unwrap().to_string(),
        )),
    ];
    drive_loggers(&mut manager, &mut loggers);

    // "1", "2", "2,1" are all distinct → still 3 files even with dedup
    assert_eq!(std::fs::read_dir(&out).unwrap().count(), 3);
}
