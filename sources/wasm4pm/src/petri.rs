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
        let is_wf_net = has_unique_source && has_unique_sink;

        let source_place = source_places.first().cloned();
        let sink_place = sink_places.first().cloned();

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
            };
        }

        let src = source_place.clone().unwrap();
        let snk = sink_place.clone().unwrap();

        let m0 = Marking::initial(src);
        let final_sink_marking = Marking::initial(snk.clone());

        let mut visited = BTreeSet::new();
        let mut edges = BTreeSet::new();
        let mut path = Vec::new();
        let mut is_1_bounded = true;

        // Run depth-first search to build reachability graph and check boundedness
        self.explore_reachability(
            m0,
            &mut path,
            &mut visited,
            &mut edges,
            &mut is_1_bounded,
        );

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
    ) {
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
                self.explore_reachability(next_marking, path, visited, edges, is_1_bounded);
            }
        }

        path.pop();
    }
}
