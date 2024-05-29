/*
 * Copyright (c) 2024 Maximilian Barmetler <https://barmetler.com>
 *
 * Use of this source code is governed by an MIT-style
 * license that can be found in the LICENSE file or at
 * https://opensource.org/licenses/MIT.
 */

use crate::node_info::NodeInfo;

#[derive(Clone, Eq, PartialEq, Debug, Default)]
pub struct FingerTable {
    /// The predecessors of the node.
    /// Usually, one is enough, but for stability, multiple can be used.
    predecessors: Vec<FingerTableEntry>,

    /// The successors of the node.
    /// Usually, one is enough, but for stability, multiple can be used.
    successors: Vec<FingerTableEntry>,

    /// The entries of the finger table.
    ///
    /// The size is ensured to be m (128), but entries can be empty.
    entries: Vec<Option<FingerTableEntry>>,
}

#[derive(Clone, Eq, PartialEq, Debug)]
pub struct FingerTableEntry {
    pub node_info: NodeInfo,
}

impl FingerTable {
    pub fn new(size: usize) -> Self {
        Self {
            predecessors: Vec::new(),
            successors: Vec::new(),
            entries: vec![None; size],
        }
    }

    pub fn get_predecessors(&self) -> &Vec<FingerTableEntry> {
        &self.predecessors
    }

    pub fn get_successors(&self) -> &Vec<FingerTableEntry> {
        &self.successors
    }

    pub fn get_entries(&self) -> &Vec<Option<FingerTableEntry>> {
        &self.entries
    }

    pub fn get_predecessors_mut(&mut self) -> &mut Vec<FingerTableEntry> {
        &mut self.predecessors
    }

    pub fn get_successors_mut(&mut self) -> &mut Vec<FingerTableEntry> {
        &mut self.successors
    }

    pub fn get_entries_mut(&mut self) -> &mut [Option<FingerTableEntry>] {
        &mut self.entries
    }
}
