/**
 * declare.js
 * Declare Constraint Validator for LTL Process Rules
 * 
 * Supports parsing and verifying standard Declare templates against trace sequences.
 * Provides detailed reports including activation, satisfaction, and violation event indices.
 */

class DeclareValidator {
    constructor() {
        this.templates = {
            'Existence': {
                arity: 1,
                validate: (trace, a) => {
                    const activations = [];
                    const satisfactions = [];
                    const violations = [];
                    
                    const indices = this._findIndices(trace, a);
                    if (indices.length >= 1) {
                        satisfactions.push(indices[0]); // Satisfied at first occurrence
                        return { status: 'FULFILLED', activations, satisfactions, violations };
                    } else {
                        violations.push(-1); // Violated because it didn't occur
                        return { status: 'VIOLATED', activations, satisfactions, violations };
                    }
                }
            },
            'Absence': {
                arity: 1,
                validate: (trace, a) => {
                    const activations = [];
                    const satisfactions = [];
                    const violations = [];
                    
                    const indices = this._findIndices(trace, a);
                    if (indices.length === 0) {
                        return { status: 'FULFILLED', activations, satisfactions, violations };
                    } else {
                        // All occurrences are violations
                        return { status: 'VIOLATED', activations, satisfactions, violations: indices };
                    }
                }
            },
            'Exactly1': {
                arity: 1,
                validate: (trace, a) => {
                    const activations = [];
                    const satisfactions = [];
                    const violations = [];
                    
                    const indices = this._findIndices(trace, a);
                    if (indices.length === 1) {
                        satisfactions.push(indices[0]);
                        return { status: 'FULFILLED', activations, satisfactions, violations };
                    } else {
                        if (indices.length > 1) {
                            // The second and subsequent ones are violations
                            return { status: 'VIOLATED', activations, satisfactions, violations: indices.slice(1) };
                        } else {
                            violations.push(-1); // Missing occurrence
                            return { status: 'VIOLATED', activations, satisfactions, violations };
                        }
                    }
                }
            },
            'Init': {
                arity: 1,
                validate: (trace, a) => {
                    if (trace.length === 0) {
                        return { status: 'PENDING', activations: [], satisfactions: [], violations: [] };
                    }
                    const actName = this._getActivityName(trace[0]);
                    if (actName === a) {
                        return { status: 'FULFILLED', activations: [], satisfactions: [0], violations: [] };
                    } else {
                        return { status: 'VIOLATED', activations: [], satisfactions: [], violations: [0] };
                    }
                }
            },
            'Response': {
                arity: 2,
                validate: (trace, a, b) => {
                    const activations = [];
                    const satisfactions = [];
                    const violations = [];
                    let status = 'FULFILLED';

                    const aIndices = this._findIndices(trace, a);
                    const bIndices = this._findIndices(trace, b);

                    for (const aIdx of aIndices) {
                        activations.push(aIdx);
                        // Find first B after A
                        const satisfiedBy = bIndices.find(bIdx => bIdx > aIdx);
                        if (satisfiedBy !== undefined) {
                            satisfactions.push(satisfiedBy);
                        } else {
                            violations.push(aIdx); // This A was never responded to
                            status = 'VIOLATED';
                        }
                    }

                    return { status, activations, satisfactions, violations };
                }
            },
            'Precedence': {
                arity: 2,
                validate: (trace, a, b) => {
                    const activations = [];
                    const satisfactions = [];
                    const violations = [];
                    let status = 'FULFILLED';

                    const aIndices = this._findIndices(trace, a);
                    const bIndices = this._findIndices(trace, b);

                    for (const bIdx of bIndices) {
                        activations.push(bIdx);
                        // Find if there is an A before B
                        const precededBy = aIndices.find(aIdx => aIdx < bIdx);
                        if (precededBy !== undefined) {
                            satisfactions.push(precededBy);
                        } else {
                            violations.push(bIdx); // B occurred without prior A
                            status = 'VIOLATED';
                        }
                    }

                    return { status, activations, satisfactions, violations };
                }
            },
            'Succession': {
                arity: 2,
                validate: (trace, a, b) => {
                    const resp = this.templates['Response'].validate(trace, a, b);
                    const prec = this.templates['Precedence'].validate(trace, a, b);
                    
                    const status = (resp.status === 'VIOLATED' || prec.status === 'VIOLATED') ? 'VIOLATED' : 'FULFILLED';
                    return {
                        status,
                        activations: [...new Set([...resp.activations, ...prec.activations])].sort((x, y) => x - y),
                        satisfactions: [...new Set([...resp.satisfactions, ...prec.satisfactions])].sort((x, y) => x - y),
                        violations: [...new Set([...resp.violations, ...prec.violations])].sort((x, y) => x - y)
                    };
                }
            },
            'AlternateResponse': {
                arity: 2,
                validate: (trace, a, b) => {
                    const activations = [];
                    const satisfactions = [];
                    const violations = [];
                    let status = 'FULFILLED';

                    const aIndices = this._findIndices(trace, a);
                    const bIndices = this._findIndices(trace, b);

                    for (let i = 0; i < aIndices.length; i++) {
                        const aIdx = aIndices[i];
                        activations.push(aIdx);
                        const nextA = aIndices[i + 1] !== undefined ? aIndices[i + 1] : Infinity;

                        // Find first B after this A
                        const satisfiedBy = bIndices.find(bIdx => bIdx > aIdx && bIdx < nextA);
                        if (satisfiedBy !== undefined) {
                            satisfactions.push(satisfiedBy);
                        } else {
                            violations.push(aIdx); // No B occurred before the next A or end of trace
                            status = 'VIOLATED';
                        }
                    }

                    return { status, activations, satisfactions, violations };
                }
            },
            'AlternatePrecedence': {
                arity: 2,
                validate: (trace, a, b) => {
                    const activations = [];
                    const satisfactions = [];
                    const violations = [];
                    let status = 'FULFILLED';

                    const aIndices = this._findIndices(trace, a);
                    const bIndices = this._findIndices(trace, b);

                    for (let i = 0; i < bIndices.length; i++) {
                        const bIdx = bIndices[i];
                        activations.push(bIdx);
                        const prevB = bIndices[i - 1] !== undefined ? bIndices[i - 1] : -1;

                        // Find if there is an A between the previous B and this B
                        const precededBy = aIndices.find(aIdx => aIdx < bIdx && aIdx > prevB);
                        if (precededBy !== undefined) {
                            satisfactions.push(precededBy);
                        } else {
                            violations.push(bIdx); // B occurred without a new A since the last B
                            status = 'VIOLATED';
                        }
                    }

                    return { status, activations, satisfactions, violations };
                }
            },
            'ChainResponse': {
                arity: 2,
                validate: (trace, a, b) => {
                    const activations = [];
                    const satisfactions = [];
                    const violations = [];
                    let status = 'FULFILLED';

                    const aIndices = this._findIndices(trace, a);

                    for (const aIdx of aIndices) {
                        activations.push(aIdx);
                        const nextIdx = aIdx + 1;
                        if (nextIdx < trace.length) {
                            const nextAct = this._getActivityName(trace[nextIdx]);
                            if (nextAct === b) {
                                satisfactions.push(nextIdx);
                            } else {
                                violations.push(aIdx); // Next element is not B
                                status = 'VIOLATED';
                            }
                        } else {
                            // A is the last element
                            violations.push(aIdx);
                            status = 'VIOLATED';
                        }
                    }

                    return { status, activations, satisfactions, violations };
                }
            },
            'ChainPrecedence': {
                arity: 2,
                validate: (trace, a, b) => {
                    const activations = [];
                    const satisfactions = [];
                    const violations = [];
                    let status = 'FULFILLED';

                    const bIndices = this._findIndices(trace, b);

                    for (const bIdx of bIndices) {
                        activations.push(bIdx);
                        const prevIdx = bIdx - 1;
                        if (prevIdx >= 0) {
                            const prevAct = this._getActivityName(trace[prevIdx]);
                            if (prevAct === a) {
                                satisfactions.push(prevIdx);
                            } else {
                                violations.push(bIdx); // Previous element was not A
                                status = 'VIOLATED';
                            }
                        } else {
                            // B is the first element
                            violations.push(bIdx);
                            status = 'VIOLATED';
                        }
                    }

                    return { status, activations, satisfactions, violations };
                }
            },
            'NotCoExistence': {
                arity: 2,
                validate: (trace, a, b) => {
                    const aIndices = this._findIndices(trace, a);
                    const bIndices = this._findIndices(trace, b);

                    if (aIndices.length > 0 && bIndices.length > 0) {
                        return {
                            status: 'VIOLATED',
                            activations: [...aIndices, ...bIndices].sort((x, y) => x - y),
                            satisfactions: [],
                            violations: [...aIndices, ...bIndices].sort((x, y) => x - y)
                        };
                    } else {
                        return { status: 'FULFILLED', activations: [], satisfactions: [], violations: [] };
                    }
                }
            }
        };
    }

    /**
     * Helper to get activity name from trace event (which can be string or object).
     */
    _getActivityName(event) {
        if (!event) return null;
        if (typeof event === 'string') return event;
        return event.activity || event.name || '';
    }

    /**
     * Finds all indices in trace where the activity matches `actName`.
     */
    _findIndices(trace, actName) {
        const indices = [];
        for (let i = 0; i < trace.length; i++) {
            if (this._getActivityName(trace[i]) === actName) {
                indices.push(i);
            }
        }
        return indices;
    }

    /**
     * Parses a rule string, e.g., "Response(ReceiveRequest, Approve)"
     * Returns { template, params: [param1, param2, ...] } or null if invalid.
     */
    parseRule(ruleStr) {
        const trimmed = ruleStr.trim();
        const match = trimmed.match(/^([a-zA-Z0-9_]+)\s*\(([^)]+)\)$/);
        if (!match) return null;

        const template = match[1];
        const params = match[2].split(',').map(s => s.trim());

        if (this.templates[template]) {
            const expectedArity = this.templates[template].arity;
            if (params.length === expectedArity) {
                return { template, params, original: trimmed };
            }
        }
        return null;
    }

    /**
     * Verifies a trace against a set of rule strings or parsed rule objects.
     * Returns detailed evaluation results.
     */
    verifyTrace(trace, rules) {
        let allCompliant = true;
        const results = [];

        for (const rule of rules) {
            let parsed = typeof rule === 'string' ? this.parseRule(rule) : rule;
            if (!parsed) {
                results.push({
                    rule: typeof rule === 'string' ? rule : JSON.stringify(rule),
                    error: 'Failed to parse rule',
                    status: 'VIOLATED'
                });
                allCompliant = false;
                continue;
            }

            const tmpl = this.templates[parsed.template];
            if (!tmpl) {
                results.push({
                    rule: parsed.original,
                    error: `Unknown template: ${parsed.template}`,
                    status: 'VIOLATED'
                });
                allCompliant = false;
                continue;
            }

            try {
                const evaluation = tmpl.validate(trace, ...parsed.params);
                if (evaluation.status === 'VIOLATED') {
                    allCompliant = false;
                }
                results.push({
                    rule: parsed.original || `${parsed.template}(${parsed.params.join(', ')})`,
                    template: parsed.template,
                    params: parsed.params,
                    status: evaluation.status,
                    activations: evaluation.activations,
                    satisfactions: evaluation.satisfactions,
                    violations: evaluation.violations
                });
            } catch (err) {
                allCompliant = false;
                results.push({
                    rule: parsed.original || `${parsed.template}(${parsed.params.join(', ')})`,
                    error: `Validation error: ${err.message}`,
                    status: 'VIOLATED'
                });
            }
        }

        return {
            compliant: allCompliant,
            results: results
        };
    }
}

// Export for usage in ESModules or global window object
if (typeof module !== 'undefined' && module.exports) {
    module.exports = DeclareValidator;
} else {
    window.DeclareValidator = DeclareValidator;
}
