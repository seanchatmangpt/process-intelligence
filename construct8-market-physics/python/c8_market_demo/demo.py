#!/usr/bin/env python3
"""
CONSTRUCT8 Market Physics Demo
Synthetic ticks and relational state-change tracker.
"""

class TickObservation:
    def __init__(self, inst_id, venue_id, price, volume, bid_price, ask_price, bid_vol, ask_vol, ts):
        self.instrument_id = inst_id
        self.venue_id = venue_id
        self.price = price
        self.volume = volume
        self.bid_price = bid_price
        self.ask_price = ask_price
        self.bid_volume = bid_vol
        self.ask_volume = ask_vol
        self.timestamp = ts

def detect_relation_break(bid, ask):
    return "RelationBreak(CrossedSpread)" if bid >= ask else "Normal"

def detect_capital_pressure(bid_vol, ask_vol):
    if bid_vol > ask_vol * 2:
        return "CapitalPressure(HeavyBuy)"
    elif ask_vol > bid_vol * 2:
        return "CapitalPressure(HeavySell)"
    return "Normal"

def main():
    print("=== CONSTRUCT8 Market Physics Python Demo ===")

    ticks = [
        TickObservation(1, 10, 100, 50, 99, 101, 10, 10, 1000),
        TickObservation(1, 10, 100, 50, 102, 101, 10, 10, 1001), # Crossed Spread
        TickObservation(1, 10, 100, 50, 99, 101, 10, 10, 1002),
        TickObservation(1, 10, 100, 50, 99, 101, 50, 10, 1003), # Heavy Buy
        TickObservation(1, 10, 100, 50, 99, 101, 10, 10, 1004)
    ]

    print(f"Ingesting {len(ticks)} synthetic ticks...")

    logic_tree = []
    graph_tree = []

    for i, t in enumerate(ticks):
        # Logic player tree only represents price
        logic_tree.append(f"PriceFeature:{t.price}")

        # Graph player tree represents relational states
        rel_break = detect_relation_break(t.bid_price, t.ask_price)
        cap_pressure = detect_capital_pressure(t.bid_volume, t.ask_volume)
        
        if rel_break != "Normal":
            graph_tree.append(rel_break)
        elif cap_pressure != "Normal":
            graph_tree.append(cap_pressure)
        else:
            graph_tree.append("NormalRelationalState")

    # Find representation gap (states visible to graph player but not logic player)
    missing = [state for state in graph_tree if "Normal" not in state]

    print("\n--- Representation Gap Analysis ---")
    print("Competitor (LogicPlayer) basis vectors: Price Features")
    print(f"Competitor state logs: {logic_tree}")
    print("Our (GraphPlayer) basis vectors: Relational Graph States")
    print(f"Our state logs: {graph_tree}")
    print(f"Missing states in competitor's model basis: {missing}")
    print(f"Calculated Representation Gap Score: {len(missing) * 0.25}")

if __name__ == "__main__":
    main()
