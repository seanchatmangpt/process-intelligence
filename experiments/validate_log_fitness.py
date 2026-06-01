#!/usr/bin/env python3
"""
validate_log_fitness.py

Programmatically verifies the weighted log fitness equation:
    f(L, N) = 1 - (sum m / sum c) - (sum r / sum p)

Under the v30.1.1 ultimate standard.
"""

import sys


class PetriNet:
    """
    Represents a Petri net with places, transitions, and directed arcs.
    """
    def __init__(self):
        self.places = set()
        self.transitions = set()
        self.in_arcs = {}   # transition -> list of places
        self.out_arcs = {}  # transition -> list of places

    def add_place(self, name: str):
        self.places.add(name)

    def add_transition(self, name: str):
        self.transitions.add(name)
        if name not in self.in_arcs:
            self.in_arcs[name] = []
        if name not in self.out_arcs:
            self.out_arcs[name] = []

    def add_input_arc(self, place: str, transition: str):
        self.add_place(place)
        self.add_transition(transition)
        self.in_arcs[transition].append(place)

    def add_output_arc(self, transition: str, place: str):
        self.add_transition(transition)
        self.add_place(place)
        self.out_arcs[transition].append(place)


class TokenReplaySimulator:
    """
    Replays event traces on a Petri Net and tracks tokens produced, consumed, missing, and remaining.
    """
    def __init__(self, net: PetriNet):
        self.net = net

    def replay_trace(self, trace: list, initial_place: str, sink_place: str) -> dict:
        """
        Replays a single trace on the Petri Net.
        Returns:
            {
                "m": missing tokens,
                "c": consumed tokens,
                "r": remaining tokens,
                "p": produced tokens
            }
        """
        # Initialize marking
        marking = {place: 0 for place in self.net.places}
        marking[initial_place] = 1
        
        m = 0
        c = 0
        p = 1  # 1 token produced initially in initial_place

        # Replay transition sequence
        for transition in trace:
            if transition not in self.net.transitions:
                raise ValueError(f"Transition '{transition}' not found in the Petri Net.")
            
            # Consume tokens from input places
            in_places = self.net.in_arcs[transition]
            for place in in_places:
                if marking[place] < 1:
                    m += 1  # missing token
                    # Artificially add the token so transition can fire.
                    # Note: PM4Py standard doesn't count artificial token addition in 'p' produced tokens,
                    # but does count it in 'm' missing tokens.
                    marking[place] += 1
                
                # Consume token
                marking[place] -= 1
                c += 1

            # Produce tokens in output places
            out_places = self.net.out_arcs[transition]
            for place in out_places:
                marking[place] += 1
                p += 1

        # Termination: consume 1 token from sink place
        if marking[sink_place] < 1:
            m += 1  # missing token at sink
            marking[sink_place] += 1

        marking[sink_place] -= 1
        c += 1

        # Count remaining tokens in all places
        r = sum(marking.values())

        return {
            "m": m,
            "c": c,
            "r": r,
            "p": p
        }

    def compute_log_fitness(self, log: list, initial_place: str, sink_place: str) -> float:
        """
        Computes the weighted log fitness:
            f(L, N) = 1 - (sum m / sum c) - (sum r / sum p)
        """
        sum_m = 0
        sum_c = 0
        sum_r = 0
        sum_p = 0

        for trace in log:
            res = self.replay_trace(trace, initial_place, sink_place)
            sum_m += res["m"]
            sum_c += res["c"]
            sum_r += res["r"]
            sum_p += res["p"]

        # Handle boundary case: empty log or zero tokens
        if sum_c == 0 or sum_p == 0:
            return 0.0

        return 1.0 - (sum_m / sum_c) - (sum_r / sum_p)


def test_conformance():
    print("==================================================")
    print("Testing Petri Net Log Conformance and Token Replay")
    print("==================================================")

    # Setup Net 1: Linear Net for Case A and Case B
    # i -> Register -> p_1 -> Approve -> p_2 -> Ship -> o
    net1 = PetriNet()
    net1.add_input_arc("i", "Register")
    net1.add_output_arc("Register", "p_1")
    net1.add_input_arc("p_1", "Approve")
    net1.add_output_arc("Approve", "p_2")
    net1.add_input_arc("p_2", "Ship")
    net1.add_output_arc("Ship", "o")

    sim1 = TokenReplaySimulator(net1)

    # 1. Perfectly conforming trace (fitness = 1.0)
    trace_a = ["Register", "Approve", "Ship"]
    res_a = sim1.replay_trace(trace_a, "i", "o")
    print(f"Trace A (Conforming): {trace_a}")
    print(f"  Results: m={res_a['m']}, c={res_a['c']}, r={res_a['r']}, p={res_a['p']}")
    fitness_a = sim1.compute_log_fitness([trace_a], "i", "o")
    print(f"  Calculated Fitness: {fitness_a:.4f}")
    assert fitness_a == 1.0, f"Expected fitness 1.0, got {fitness_a}"
    assert res_a['m'] == 0
    assert res_a['r'] == 0

    # 2. Trace with both missing and remaining tokens
    trace_b = ["Approve", "Ship"]
    res_b = sim1.replay_trace(trace_b, "i", "o")
    print(f"Trace B (Missing & Remaining): {trace_b}")
    print(f"  Results: m={res_b['m']}, c={res_b['c']}, r={res_b['r']}, p={res_b['p']}")
    fitness_b = sim1.compute_log_fitness([trace_b], "i", "o")
    print(f"  Calculated Fitness: {fitness_b:.4f}")
    # Theoretical: 1 - 1/3 - 1/3 = 1/3
    assert abs(fitness_b - (1.0 / 3.0)) < 1e-9, f"Expected fitness 0.3333, got {fitness_b}"
    assert res_b['m'] == 1
    assert res_b['r'] == 1

    # Setup Net 2: Net with dangling production (to show remaining tokens only)
    # i -> ProduceDangling -> o (plus o receives extra token)
    net2 = PetriNet()
    net2.add_place("i")
    net2.add_place("o")
    net2.add_transition("ProduceDangling")
    net2.add_output_arc("ProduceDangling", "o")

    sim2 = TokenReplaySimulator(net2)

    # 3. Trace with remaining tokens only
    trace_c = ["ProduceDangling"]
    res_c = sim2.replay_trace(trace_c, "i", "o")
    print(f"Trace C (Remaining only): {trace_c}")
    print(f"  Results: m={res_c['m']}, c={res_c['c']}, r={res_c['r']}, p={res_c['p']}")
    fitness_c = sim2.compute_log_fitness([trace_c], "i", "o")
    print(f"  Calculated Fitness: {fitness_c:.4f}")
    # Theoretical: 1 - 0/1 - 1/2 = 0.5
    assert fitness_c == 0.5, f"Expected fitness 0.5, got {fitness_c}"
    assert res_c['m'] == 0
    assert res_c['r'] == 1

    # Setup Net 3: Net with extra consumption (to show missing tokens only)
    # i -> ConsumeExtra
    net3 = PetriNet()
    net3.add_place("i")
    net3.add_place("o")
    net3.add_transition("ConsumeExtra")
    net3.add_input_arc("i", "ConsumeExtra")

    sim3 = TokenReplaySimulator(net3)

    # 4. Trace with missing tokens only
    trace_d = ["ConsumeExtra"]
    res_d = sim3.replay_trace(trace_d, "i", "o")
    print(f"Trace D (Missing only): {trace_d}")
    print(f"  Results: m={res_d['m']}, c={res_d['c']}, r={res_d['r']}, p={res_d['p']}")
    fitness_d = sim3.compute_log_fitness([trace_d], "i", "o")
    print(f"  Calculated Fitness: {fitness_d:.4f}")
    # Theoretical: 1 - 1/2 - 0/1 = 0.5
    assert fitness_d == 0.5, f"Expected fitness 0.5, got {fitness_d}"
    assert res_d['m'] == 1
    assert res_d['r'] == 0

    # 5. Log-level check with multiple traces (Trace A and Trace B)
    # Trace A: m=0, c=4, r=0, p=4
    # Trace B: m=1, c=3, r=1, p=3
    # Total m=1, c=7, r=1, p=7
    # Log fitness: 1 - 1/7 - 1/7 = 5/7 = 0.7142857...
    combined_log = [trace_a, trace_b]
    log_fitness = sim1.compute_log_fitness(combined_log, "i", "o")
    print(f"Combined Log [Trace A, Trace B] Fitness: {log_fitness:.4f}")
    assert abs(log_fitness - (5.0 / 7.0)) < 1e-9, f"Expected fitness 0.7143, got {log_fitness}"

    print("\nSUCCESS: All fitness replay simulations and mathematical assertions passed!")
    print("==================================================")


if __name__ == "__main__":
    try:
        test_conformance()
        sys.exit(0)
    except AssertionError as e:
        print(f"FAILURE: Assertion error: {e}", file=sys.stderr)
        sys.exit(1)
    except Exception as e:
        print(f"FAILURE: Unexpected error: {e}", file=sys.stderr)
        sys.exit(1)
