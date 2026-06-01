use std::collections::{BTreeMap, BTreeSet, VecDeque};

/// A marking maps place names to their token counts.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Marking {
    pub tokens: BTreeMap<String, u32>,
}

impl Marking {
    /// Create a marking with a single token in the specified place.
    pub fn initial(place: String) -> Self {
        let mut tokens = BTreeMap::new();
        tokens.insert(place, 1);
        Marking { tokens }
    }

    /// Helper to get token count of a place (defaults to 0).
    pub fn get_tokens(&self, place: &str) -> u32 {
        *self.tokens.get(place).unwrap_or(&0)
    }
}

/// A Petri Net structure representation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PetriNet {
    pub places: BTreeSet<String>,
    pub transitions: BTreeSet<String>,
    /// pre[transition][place] = weight
    pub pre: BTreeMap<String, BTreeMap<String, u32>>,
    /// post[transition][place] = weight
    pub post: BTreeMap<String, BTreeMap<String, u32>>,
}

/// The results of structural and reachability analysis of the WF-net.
///
/// See the formal specification at [Workflow Net Verification Specification](file:///Users/sac/process-intelligence/standards/wf-net_verification_specification.md)
/// and [PETRI_AND_WFNET.md](file:///Users/sac/process-intelligence/standards/PETRI_AND_WFNET.md).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SoundnessResult {
    pub is_wf_net: bool,
    pub source_place: Option<String>,
    pub sink_place: Option<String>,
    pub is_1_bounded: bool,
    pub has_deadlock: bool,
    pub dead_transitions: BTreeSet<String>,
    pub proper_completion: bool,
    pub option_to_complete: bool,
    pub markings_visited: usize,
    pub state_limit_exceeded: bool,
}

impl PetriNet {
    /// Creates a new Petri Net.
    pub fn new(
        places: BTreeSet<String>,
        transitions: BTreeSet<String>,
        pre: BTreeMap<String, BTreeMap<String, u32>>,
        post: BTreeMap<String, BTreeMap<String, u32>>,
    ) -> Self {
        PetriNet {
            places,
            transitions,
            pre,
            post,
        }
    }

    /// Check if a transition is enabled under the given marking.
    pub fn is_enabled(&self, transition: &str, marking: &Marking) -> bool {
        if !self.transitions.contains(transition) {
            return false;
        }
        if let Some(inputs) = self.pre.get(transition) {
            for (place, &weight) in inputs {
                if marking.get_tokens(place) < weight {
                    return false;
                }
            }
        }
        true
    }

    /// Fires a transition, producing a new marking. Assumes transition is enabled.
    pub fn fire(&self, transition: &str, marking: &Marking) -> Marking {
        let mut new_tokens = marking.tokens.clone();

        // Consume inputs
        if let Some(inputs) = self.pre.get(transition) {
            for (place, &weight) in inputs {
                let val = new_tokens.entry(place.clone()).or_insert(0);
                if *val >= weight {
                    *val -= weight;
                } else {
                    *val = 0;
                }
            }
        }

        // Produce outputs
        if let Some(outputs) = self.post.get(transition) {
            for (place, &weight) in outputs {
                let val = new_tokens.entry(place.clone()).or_insert(0);
                *val += weight;
            }
        }

        // Clean up zero tokens to normalize
        let cleaned: BTreeMap<String, u32> = new_tokens
            .into_iter()
            .filter(|(_, v)| *v > 0)
            .collect();

        Marking { tokens: cleaned }
    }

    /// Checks if marking `m1` component-wise covers marking `m2` (m1 >= m2 and m1 != m2).
    pub fn covers(&self, m1: &Marking, m2: &Marking) -> bool {
        let mut strictly_greater = false;
        for place in &self.places {
            let v1 = m1.get_tokens(place);
            let v2 = m2.get_tokens(place);
            if v1 < v2 {
                return false;
            }
            if v1 > v2 {
                strictly_greater = true;
            }
        }
        strictly_greater
    }

    /// Performs the soundness and 1-boundedness reachability/coverability check on the WF-net.
    ///
    /// For the formal verification specification, see [Workflow Net Verification Specification](file:///Users/sac/process-intelligence/standards/wf-net_verification_specification.md).
    pub fn analyze_soundness(&self) -> SoundnessResult {
        // 1. Structural WF-net checks
        // Identify source places (in-degree = 0)
        // A place p has in-degree 0 if it is never produced by any transition.
        let mut source_places = Vec::new();
        for p in &self.places {
            let mut has_incoming = false;
            for outputs in self.post.values() {
                if let Some(&weight) = outputs.get(p) {
                    if weight > 0 {
                        has_incoming = true;
                        break;
                    }
                }
            }
            if !has_incoming {
                source_places.push(p.clone());
            }
        }

        // Identify sink places (out-degree = 0)
        // A place p has out-degree 0 if it is never consumed by any transition.
        let mut sink_places = Vec::new();
        for p in &self.places {
            let mut has_outgoing = false;
            for inputs in self.pre.values() {
                if let Some(&weight) = inputs.get(p) {
                    if weight > 0 {
                        has_outgoing = true;
                        break;
                    }
                }
            }
            if !has_outgoing {
                sink_places.push(p.clone());
            }
        }

        let has_unique_source = source_places.len() == 1;
        let has_unique_sink = sink_places.len() == 1;
        let mut is_wf_net = has_unique_source && has_unique_sink;

        let source_place = source_places.first().cloned();
        let sink_place = sink_places.first().cloned();

        if is_wf_net {
            let src = source_place.as_ref().unwrap();

            // Check weak path connectivity: every place/transition must lie on an undirected path.
            // We run an undirected BFS starting from the source place.
            let mut visited = BTreeSet::new();
            let mut queue = VecDeque::new();
            visited.insert(src.clone());
            queue.push_back(src.clone());

            while let Some(curr) = queue.pop_front() {
                if self.places.contains(&curr) {
                    // Current node is a place. Neighbors are transitions in its preset or postset.
                    for t in &self.transitions {
                        let is_neighbor = self.pre.get(t).map_or(false, |inputs| inputs.contains_key(&curr))
                            || self.post.get(t).map_or(false, |outputs| outputs.contains_key(&curr));
                        if is_neighbor && !visited.contains(t) {
                            visited.insert(t.clone());
                            queue.push_back(t.clone());
                        }
                    }
                } else {
                    // Current node is a transition. Neighbors are places in its preset or postset.
                    if let Some(inputs) = self.pre.get(&curr) {
                        for p in inputs.keys() {
                            if !visited.contains(p) {
                                visited.insert(p.clone());
                                queue.push_back(p.clone());
                            }
                        }
                    }
                    if let Some(outputs) = self.post.get(&curr) {
                        for p in outputs.keys() {
                            if !visited.contains(p) {
                                visited.insert(p.clone());
                                queue.push_back(p.clone());
                            }
                        }
                    }
                }
            }

            // Check if all places and transitions are weakly connected
            let mut weakly_connected = true;
            for p in &self.places {
                if !visited.contains(p) {
                    weakly_connected = false;
                    break;
                }
            }
            if weakly_connected {
                for t in &self.transitions {
                    if !visited.contains(t) {
                        weakly_connected = false;
                        break;
                    }
                }
            }
            is_wf_net = weakly_connected;
        }

        // If it's not a WF-net, or we don't have source/sink, we can't complete full reachability analysis properly.
        if !is_wf_net || source_place.is_none() || sink_place.is_none() {
            return SoundnessResult {
                is_wf_net,
                source_place,
                sink_place,
                is_1_bounded: false,
                has_deadlock: false,
                dead_transitions: self.transitions.clone(),
                proper_completion: false,
                option_to_complete: false,
                markings_visited: 0,
                state_limit_exceeded: false,
            };
        }

        let src = source_place.clone().unwrap();
        let snk = sink_place.clone().unwrap();

        let m0 = Marking::initial(src.clone());
        let final_sink_marking = Marking::initial(snk.clone());

        let mut visited = BTreeSet::new();
        let mut edges = BTreeSet::new();
        let mut path = Vec::new();
        let mut is_1_bounded = true;
        let mut state_limit_exceeded = false;

        // Run depth-first search to build reachability graph and check boundedness
        self.explore_reachability(
            m0,
            &mut path,
            &mut visited,
            &mut edges,
            &mut is_1_bounded,
            &mut state_limit_exceeded,
        );

        if state_limit_exceeded {
            return SoundnessResult {
                is_wf_net,
                source_place: Some(src.clone()),
                sink_place: Some(snk.clone()),
                is_1_bounded: false,
                has_deadlock: true,
                dead_transitions: self.transitions.clone(),
                proper_completion: false,
                option_to_complete: false,
                markings_visited: visited.len(),
                state_limit_exceeded: true,
            };
        }

        // 2. Deadlock Check
        // A deadlock is a reachable marking M with no enabled transitions and M != final_sink_marking.
        let mut has_deadlock = false;
        for m in &visited {
            let mut has_enabled = false;
            for t in &self.transitions {
                if self.is_enabled(t, m) {
                    has_enabled = true;
                    break;
                }
            }
            if !has_enabled && m != &final_sink_marking {
                has_deadlock = true;
            }
        }

        // 3. Dead Transitions Check
        // Transitions that were never fired in the reachability graph
        let mut fired_transitions = BTreeSet::new();
        for (_, t, _) in &edges {
            fired_transitions.insert(t.clone());
        }
        let dead_transitions: BTreeSet<String> = self
            .transitions
            .difference(&fired_transitions)
            .cloned()
            .collect();

        // 4. Proper Completion Check
        // If the sink place has a token, no other place can have tokens.
        let mut proper_completion = true;
        for m in &visited {
            if m.get_tokens(&snk) > 0 {
                // Check if any other place has tokens
                for p in &self.places {
                    if p != &snk && m.get_tokens(p) > 0 {
                        proper_completion = false;
                    }
                }
            }
        }

        // 5. Option to Complete Check
        // Check that final sink marking is reachable from all markings in the reachability graph
        // using backward traversal.
        let mut reach_back = BTreeSet::new();
        let mut queue = VecDeque::new();

        if visited.contains(&final_sink_marking) {
            reach_back.insert(final_sink_marking.clone());
            queue.push_back(final_sink_marking);
        }

        while let Some(curr) = queue.pop_front() {
            for (src_m, _, dest_m) in &edges {
                if dest_m == &curr {
                    if !reach_back.contains(src_m) {
                        reach_back.insert(src_m.clone());
                        queue.push_back(src_m.clone());
                    }
                }
            }
        }

        let option_to_complete = visited.iter().all(|m| reach_back.contains(m));

        SoundnessResult {
            is_wf_net,
            source_place,
            sink_place,
            is_1_bounded,
            has_deadlock,
            dead_transitions,
            proper_completion,
            option_to_complete,
            markings_visited: visited.len(),
            state_limit_exceeded: false,
        }
    }

    /// Recursive DFS helper for state space exploration.
    fn explore_reachability(
        &self,
        current: Marking,
        path: &mut Vec<Marking>,
        visited: &mut BTreeSet<Marking>,
        edges: &mut BTreeSet<(Marking, String, Marking)>,
        is_1_bounded: &mut bool,
        state_limit_exceeded: &mut bool,
    ) {
        if *state_limit_exceeded {
            return;
        }

        const MAX_STATES: usize = 10_000;
        if visited.len() >= MAX_STATES {
            *state_limit_exceeded = true;
            return;
        }

        // Check 1-boundedness condition: token count > 1
        for p in &self.places {
            if current.get_tokens(p) > 1 {
                *is_1_bounded = false;
            }
        }

        // Check if current covers any ancestor on the path
        for ancestor in path.iter() {
            if self.covers(&current, ancestor) {
                *is_1_bounded = false;
            }
        }

        // Add to visited, check if already explored
        let is_new = visited.insert(current.clone());
        if !is_new {
            return;
        }

        // Pruning logic to ensure termination:
        // If it covers an ancestor, continuing DFS on this branch will cause an infinite loop.
        let mut should_prune = false;
        for ancestor in path.iter() {
            if self.covers(&current, ancestor) {
                should_prune = true;
            }
        }
        if should_prune {
            return;
        }

        path.push(current.clone());

        // Explore enabled transitions
        for t in &self.transitions {
            if self.is_enabled(t, &current) {
                let next_marking = self.fire(t, &current);
                edges.insert((current.clone(), t.clone(), next_marking.clone()));
                self.explore_reachability(
                    next_marking,
                    path,
                    visited,
                    edges,
                    is_1_bounded,
                    state_limit_exceeded,
                );
            }
        }

        path.pop();
    }

    /// Check if a subset of places S is a siphon (non-empty).
    /// A siphon is a set S where the preset of S is a subset of the postset of S: •S ⊆ S•.
    pub fn is_siphon(&self, s: &BTreeSet<String>) -> bool {
        if s.is_empty() {
            return false;
        }
        for (t, outputs) in &self.post {
            let outputs_to_s = outputs.iter().any(|(p, &w)| w > 0 && s.contains(p));
            if outputs_to_s {
                let inputs_from_s = self.pre.get(t)
                    .map(|inputs| inputs.iter().any(|(p, &w)| w > 0 && s.contains(p)))
                    .unwrap_or(false);
                if !inputs_from_s {
                    return false;
                }
            }
        }
        true
    }

    /// Check if a subset of places T is a trap (non-empty).
    /// A trap is a set T where the postset of T is a subset of the preset of T: T• ⊆ •T.
    pub fn is_trap(&self, t_set: &BTreeSet<String>) -> bool {
        if t_set.is_empty() {
            return false;
        }
        for (t, inputs) in &self.pre {
            let inputs_from_t = inputs.iter().any(|(p, &w)| w > 0 && t_set.contains(p));
            if inputs_from_t {
                let outputs_to_t = self.post.get(t)
                    .map(|outputs| outputs.iter().any(|(p, &w)| w > 0 && t_set.contains(p)))
                    .unwrap_or(false);
                if !outputs_to_t {
                    return false;
                }
            }
        }
        true
    }

    /// Recursively find all siphons.
    pub fn find_siphons(&self) -> Vec<BTreeSet<String>> {
        let places: Vec<String> = self.places.iter().cloned().collect();
        let mut result = Vec::new();
        let mut current = BTreeSet::new();
        self.siphons_recurse(&places, 0, &mut current, &mut result);
        result
    }

    fn siphons_recurse(
        &self,
        places: &[String],
        index: usize,
        current: &mut BTreeSet<String>,
        result: &mut Vec<BTreeSet<String>>,
    ) {
        if result.len() >= 1000 {
            return;
        }
        if index == places.len() {
            if self.is_siphon(current) {
                result.push(current.clone());
            }
            return;
        }
        self.siphons_recurse(places, index + 1, current, result);
        current.insert(places[index].clone());
        self.siphons_recurse(places, index + 1, current, result);
        current.remove(&places[index]);
    }

    /// Recursively find all traps.
    pub fn find_traps(&self) -> Vec<BTreeSet<String>> {
        let places: Vec<String> = self.places.iter().cloned().collect();
        let mut result = Vec::new();
        let mut current = BTreeSet::new();
        self.traps_recurse(&places, 0, &mut current, &mut result);
        result
    }

    fn traps_recurse(
        &self,
        places: &[String],
        index: usize,
        current: &mut BTreeSet<String>,
        result: &mut Vec<BTreeSet<String>>,
    ) {
        if result.len() >= 1000 {
            return;
        }
        if index == places.len() {
            if self.is_trap(current) {
                result.push(current.clone());
            }
            return;
        }
        self.traps_recurse(places, index + 1, current, result);
        current.insert(places[index].clone());
        self.traps_recurse(places, index + 1, current, result);
        current.remove(&places[index]);
    }

    /// Verify the siphon-trap property under a marking:
    /// every siphon contains a trap that is marked (contains at least one token).
    pub fn check_siphon_trap_property(&self, marking: &Marking) -> bool {
        let siphons = self.find_siphons();
        let traps = self.find_traps();

        for siphon in &siphons {
            let mut has_marked_contained_trap = false;
            for trap in &traps {
                if trap.iter().all(|p| siphon.contains(p)) {
                    let is_marked = trap.iter().any(|p| marking.get_tokens(p) > 0);
                    if is_marked {
                        has_marked_contained_trap = true;
                        break;
                    }
                }
            }
            if !has_marked_contained_trap {
                return false;
            }
        }
        true
    }

    /// Check if the Petri Net is free-choice:
    /// For every pair of transitions t1, t2: if their presets intersect, their presets must be identical.
    /// In addition, all arc weights in the preset must be 1.
    pub fn is_free_choice(&self) -> bool {
        for t1 in &self.transitions {
            let pre1 = match self.pre.get(t1) {
                Some(p) => p,
                None => continue,
            };
            for &w in pre1.values() {
                if w != 1 {
                    return false;
                }
            }
            for t2 in &self.transitions {
                if t1 == t2 {
                    continue;
                }
                let pre2 = match self.pre.get(t2) {
                    Some(p) => p,
                    None => continue,
                };
                let intersect = pre1.keys().any(|k| pre2.contains_key(k));
                if intersect {
                    if pre1.len() != pre2.len() {
                        return false;
                    }
                    for k in pre1.keys() {
                        if !pre2.contains_key(k) {
                            return false;
                        }
                    }
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_siphon_trap_properties() {
        let places: BTreeSet<String> = vec!["source", "p1", "sink"].into_iter().map(String::from).collect();
        let transitions: BTreeSet<String> = vec!["t1", "t2"].into_iter().map(String::from).collect();

        let mut pre = BTreeMap::new();
        let mut t1_pre = BTreeMap::new();
        t1_pre.insert("source".to_string(), 1);
        pre.insert("t1".to_string(), t1_pre);

        let mut t2_pre = BTreeMap::new();
        t2_pre.insert("p1".to_string(), 1);
        pre.insert("t2".to_string(), t2_pre);

        let mut post = BTreeMap::new();
        let mut t1_post = BTreeMap::new();
        t1_post.insert("p1".to_string(), 1);
        post.insert("t1".to_string(), t1_post);

        let mut t2_post = BTreeMap::new();
        t2_post.insert("sink".to_string(), 1);
        post.insert("t2".to_string(), t2_post);

        let net = PetriNet::new(places, transitions, pre, post);

        assert!(net.is_free_choice());

        // Siphons of this net:
        let siphons = net.find_siphons();
        let source_set: BTreeSet<String> = vec!["source".to_string()].into_iter().collect();
        assert!(siphons.contains(&source_set));
        assert!(net.is_siphon(&source_set));

        // Traps of this net:
        let traps = net.find_traps();
        let sink_set: BTreeSet<String> = vec!["sink".to_string()].into_iter().collect();
        assert!(traps.contains(&sink_set));
        assert!(net.is_trap(&sink_set));

        // Since {"source"} contains no trap, the siphon-trap property under initial marking (source=1)
        // should be false because the siphon {"source"} has no contained marked trap.
        let marking = Marking::initial("source".to_string());
        assert!(!net.check_siphon_trap_property(&marking));
    }
}
