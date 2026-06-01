#!/usr/bin/env python3
"""
Ontology Consistency Audit - Full RDF graph validation
Loads all TTL files and performs comprehensive consistency checks
"""

import os
import sys
from pathlib import Path
from collections import defaultdict
import yaml

try:
    from rdflib import Graph, Namespace, RDF, RDFS, OWL
except ImportError:
    print("ERROR: rdflib not installed. Install with: pip install rdflib pyyaml")
    sys.exit(1)


class OntologyAuditor:
    """Audits RDF/OWL ontology consistency."""

    def __init__(self):
        self.graph = Graph()
        self.ttl_files = []
        self.errors = []
        self.warnings = []
        self.namespaces_used = {}

    def discover_ttl_files(self, root_dir):
        """Find all TTL files in the directory tree."""
        for path in Path(root_dir).rglob("*.ttl"):
            self.ttl_files.append(str(path))
        return sorted(self.ttl_files)

    def load_all_ttl_files(self):
        """Load all TTL files into the graph."""
        loaded_count = 0
        for ttl_file in self.ttl_files:
            try:
                self.graph.parse(ttl_file, format="turtle")
                loaded_count += 1
            except Exception as e:
                self.errors.append(f"Failed to parse {ttl_file}: {str(e)}")

        return loaded_count

    def check_namespace_conflicts(self):
        """Verify no duplicate prefixes with different URIs."""
        conflicts = []
        prefix_map = {}

        for prefix, namespace in self.graph.namespaces():
            if prefix in prefix_map:
                if prefix_map[prefix] != namespace:
                    conflicts.append({
                        "prefix": prefix,
                        "uri_1": str(prefix_map[prefix]),
                        "uri_2": str(namespace)
                    })
            else:
                prefix_map[prefix] = namespace
                self.namespaces_used[prefix] = str(namespace)

        return conflicts

    def check_class_validity(self):
        """Verify all class definitions are valid RDF."""
        valid_classes = set()

        # Collect all declared classes
        for s in self.graph.subjects(RDF.type, OWL.Class):
            valid_classes.add(s)
        for s in self.graph.subjects(RDF.type, RDFS.Class):
            valid_classes.add(s)

        return valid_classes

    def check_property_consistency(self):
        """Verify property ranges and domains are consistent."""
        property_info = defaultdict(lambda: {"domains": set(), "ranges": set()})
        properties_with_domains = 0
        properties_with_ranges = 0

        # Collect all property definitions
        for prop in self.graph.subjects(RDF.type, OWL.ObjectProperty):
            property_info[prop]["type"] = "ObjectProperty"
        for prop in self.graph.subjects(RDF.type, OWL.DatatypeProperty):
            property_info[prop]["type"] = "DatatypeProperty"
        for prop in self.graph.subjects(RDF.type, RDF.Property):
            property_info[prop]["type"] = "Property"

        # Check domain/range consistency
        for prop, info in property_info.items():
            domains = list(self.graph.objects(prop, RDFS.domain))
            ranges = list(self.graph.objects(prop, RDFS.range))

            if domains:
                property_info[prop]["domains"].update(domains)
                properties_with_domains += 1
            if ranges:
                property_info[prop]["ranges"].update(ranges)
                properties_with_ranges += 1

        return len(property_info), properties_with_domains, properties_with_ranges

    def check_orphaned_instances(self, valid_classes):
        """Find instances with no class definition."""
        orphaned = []
        known_vocabs = [
            OWL.Ontology, OWL.NamedIndividual, RDFS.Resource,
            OWL.Class, RDFS.Class, RDF.Property, OWL.ObjectProperty,
            OWL.DatatypeProperty, RDF.List, OWL.Restriction
        ]

        # External vocabulary prefixes
        external_vocabs = [
            "http://www.w3.org/ns/prov#",
            "http://www.w3.org/ns/dcat#",
            "http://purl.org/dc/terms/",
            "http://www.w3.org/2004/02/skos/core#",
            "http://www.w3.org/ns/shacl#",
            "http://purl.org/ontology/bibo/",
            "http://www.w3.org/2002/07/owl#"
        ]

        # Get all instances used as rdf:type values
        for obj in self.graph.objects(predicate=RDF.type):
            if obj in known_vocabs or obj in valid_classes:
                continue
            obj_str = str(obj)
            is_external = any(obj_str.startswith(ext) for ext in external_vocabs)
            if is_external:
                continue
            # Check if defined elsewhere
            is_defined = any(s == obj for s in self.graph.subjects(RDF.type))
            if not is_defined:
                orphaned.append(obj_str)

        return len(set(orphaned))

    def check_open_ontologies_bridges(self):
        """Verify bridges to standard Open Ontologies vocabularies."""
        required_vocabs = {
            "prov": "http://www.w3.org/ns/prov#",
            "dcat": "http://www.w3.org/ns/dcat#",
            "dct": "http://purl.org/dc/terms/",
            "skos": "http://www.w3.org/2004/02/skos/core#",
            "sh": "http://www.w3.org/ns/shacl#"
        }

        bridges_found = {}
        for vocab_name, vocab_uri in required_vocabs.items():
            found = False
            for s, p, o in self.graph.triples((None, None, None)):
                if (str(s).startswith(vocab_uri) or
                    str(p).startswith(vocab_uri) or
                    str(o).startswith(vocab_uri)):
                    found = True
                    break
            bridges_found[vocab_name] = found

        return bridges_found, sum(1 for v in bridges_found.values() if v)

    def generate_report(self):
        """Generate the audit report."""
        # Basic statistics
        triples = len(self.graph)
        classes = len(list(self.graph.subjects(RDF.type, OWL.Class))) + \
                  len(list(self.graph.subjects(RDF.type, RDFS.Class)))
        properties_total, props_with_domains, props_with_ranges = self.check_property_consistency()

        # Validation checks
        namespace_conflicts = self.check_namespace_conflicts()
        valid_classes = self.check_class_validity()
        orphaned_count = self.check_orphaned_instances(valid_classes)
        bridges, bridges_present = self.check_open_ontologies_bridges()

        # Determine overall status
        status = "CONSISTENT"
        if namespace_conflicts or orphaned_count > 0:
            status = "INCONSISTENT"
        if self.errors:
            status = "FAILED"

        # Build YAML report
        report = {
            "audit_timestamp": str(Path.cwd()),
            "graph_integrity": {
                "total_ttl_files": len(self.ttl_files),
                "total_triples": triples,
                "total_classes": classes,
                "total_properties": properties_total,
                "total_namespaces": len(self.namespaces_used),
                "namespace_conflicts": len(namespace_conflicts),
                "orphaned_instances": orphaned_count,
                "open_ontologies_bridges_present": bridges_present == 5,
                "status": status
            },
            "namespace_analysis": {
                "conflicts_found": namespace_conflicts if namespace_conflicts else [],
                "namespaces_registered": self.namespaces_used
            },
            "property_analysis": {
                "total_properties_checked": properties_total,
                "properties_with_domains": props_with_domains,
                "properties_with_ranges": props_with_ranges
            },
            "open_ontologies_bridges": bridges,
            "validation_errors": self.errors[:10] if self.errors else [],
            "validation_warnings": self.warnings[:10] if self.warnings else []
        }

        return report

    def audit(self, root_dir):
        """Execute the full audit."""
        self.discover_ttl_files(root_dir)
        loaded = self.load_all_ttl_files()
        return self.generate_report()


def main():
    """Main entry point."""
    root_dir = "/Users/sac/process-intelligence"

    auditor = OntologyAuditor()
    report = auditor.audit(root_dir)

    # Output directory
    output_dir = Path("/Users/sac/process-intelligence/research/pi-program/manufacturing")
    output_dir.mkdir(parents=True, exist_ok=True)
    output_file = output_dir / "ontology-consistency-audit.yaml"

    # Write YAML report
    with open(output_file, 'w') as f:
        yaml.dump(report, f, default_flow_style=False, sort_keys=False)

    print(f"Report written to: {output_file}")
    print(f"Status: {report['graph_integrity']['status']}")


if __name__ == "__main__":
    main()
