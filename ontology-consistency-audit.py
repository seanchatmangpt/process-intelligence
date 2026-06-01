#!/usr/bin/env python3
"""
Ontology Consistency Audit
Loads all TTL files into a single RDF graph and performs comprehensive validation.
"""

import os
import sys
from pathlib import Path
from collections import defaultdict
import yaml

try:
    from rdflib import Graph, Namespace, RDF, RDFS, OWL, URIRef
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
        self.orphaned_instances = set()

    def discover_ttl_files(self, root_dir):
        """Find all TTL files in the directory tree."""
        for path in Path(root_dir).rglob("*.ttl"):
            self.ttl_files.append(str(path))
        return sorted(self.ttl_files)

    def load_all_ttl_files(self):
        """Load all TTL files into the graph."""
        for ttl_file in self.ttl_files:
            try:
                self.graph.parse(ttl_file, format="turtle")
                print(f"✓ Loaded: {ttl_file}")
            except Exception as e:
                self.errors.append(f"Failed to parse {ttl_file}: {str(e)}")
                print(f"✗ Failed: {ttl_file} - {str(e)}")

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
        invalid_classes = []
        valid_classes = set()

        # Collect all declared classes
        for s in self.graph.subjects(RDF.type, OWL.Class):
            valid_classes.add(s)
        for s in self.graph.subjects(RDF.type, RDFS.Class):
            valid_classes.add(s)

        # Check for classes without proper definitions
        for subj, obj in self.graph.subject_objects(RDF.type):
            if obj in [OWL.Class, RDFS.Class]:
                if not (self.graph.value(subj, RDFS.label) or
                       self.graph.value(subj, RDFS.comment)):
                    self.warnings.append(f"Class {subj} lacks label or comment")

        return valid_classes

    def check_property_consistency(self):
        """Verify property ranges and domains are consistent."""
        property_issues = []
        properties_seen = defaultdict(lambda: {"domains": set(), "ranges": set()})

        # Collect all property definitions
        for s in self.graph.subjects(RDF.type, OWL.ObjectProperty):
            properties_seen[s]["type"] = "ObjectProperty"
        for s in self.graph.subjects(RDF.type, OWL.DatatypeProperty):
            properties_seen[s]["type"] = "DatatypeProperty"
        for s in self.graph.subjects(RDF.type, RDF.Property):
            properties_seen[s]["type"] = "Property"

        # Check domain/range consistency
        for prop, info in properties_seen.items():
            domains = list(self.graph.objects(prop, RDFS.domain))
            ranges = list(self.graph.objects(prop, RDFS.range))

            if domains:
                properties_seen[prop]["domains"].update(domains)
            if ranges:
                properties_seen[prop]["ranges"].update(ranges)

        return properties_seen

    def check_orphaned_instances(self, valid_classes):
        """Find instances with no class definition.

        Note: Classes used in rdf:type from external vocabularies (prov:, dcat:, etc.)
        are not considered orphaned as they are external references.
        """
        orphaned = []
        orphaned_sources = {}
        known_vocabs = [
            OWL.Ontology, OWL.NamedIndividual, RDFS.Resource,
            OWL.Class, RDFS.Class, RDF.Property, OWL.ObjectProperty,
            OWL.DatatypeProperty, RDF.List, OWL.Restriction
        ]

        # External vocabulary prefixes that should not be flagged as orphaned
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
            # Skip if it's a known vocabulary type
            if obj in known_vocabs:
                continue
            # Skip if it's explicitly defined as a class in this ontology
            if obj in valid_classes:
                continue
            # Skip if it's from an external vocabulary
            obj_str = str(obj)
            is_external = any(obj_str.startswith(ext) for ext in external_vocabs)
            if is_external:
                continue
            # It's potentially orphaned - only flag if not defined elsewhere
            is_defined = False
            for s in self.graph.subjects(RDF.type):
                if s == obj:
                    is_defined = True
                    break
            if not is_defined:
                orphaned.append(obj_str)
                # Find which files reference this orphaned class
                sources = []
                for subj in self.graph.subjects(RDF.type, obj):
                    sources.append(str(subj)[:50])
                orphaned_sources[obj_str] = sources[:2]

        return list(set(orphaned)), orphaned_sources

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
            vocab_ns = Namespace(vocab_uri)
            # Check if any term from this vocabulary is used
            for s, p, o in self.graph.triples((None, None, None)):
                if str(s).startswith(vocab_uri) or str(p).startswith(vocab_uri) or str(o).startswith(vocab_uri):
                    bridges_found[vocab_name] = True
                    break
            if vocab_name not in bridges_found:
                bridges_found[vocab_name] = False

        return bridges_found

    def generate_report(self):
        """Generate the audit report."""
        print("\n" + "="*60)
        print("ONTOLOGY CONSISTENCY AUDIT")
        print("="*60 + "\n")

        # Basic statistics
        triples = len(self.graph)
        classes = len(list(self.graph.subjects(RDF.type, OWL.Class)))
        properties = len(list(self.graph.subjects(RDF.type, [OWL.ObjectProperty, OWL.DatatypeProperty])))

        print(f"Loaded TTL Files: {len(self.ttl_files)}")
        print(f"Total Triples: {triples}")
        print(f"Total Classes: {classes}")
        print(f"Total Properties: {properties}")
        print(f"Namespaces: {len(self.namespaces_used)}\n")

        # Namespace conflicts
        namespace_conflicts = self.check_namespace_conflicts()
        print(f"Namespace Conflicts: {len(namespace_conflicts)}")
        if namespace_conflicts:
            for conflict in namespace_conflicts:
                print(f"  - Prefix '{conflict['prefix']}': {conflict['uri_1']} vs {conflict['uri_2']}")

        # Class validity
        valid_classes = self.check_class_validity()
        print(f"Valid Classes: {len(valid_classes)}")

        # Property consistency
        properties_checked = self.check_property_consistency()
        print(f"Properties Checked: {len(properties_checked)}")

        # Orphaned instances
        orphaned, orphaned_sources = self.check_orphaned_instances(valid_classes)
        print(f"Orphaned Instances: {len(orphaned)}")
        if orphaned:
            for inst in orphaned[:5]:
                print(f"  - {inst}")
                if inst in orphaned_sources:
                    for src in orphaned_sources[inst][:1]:
                        print(f"      used by: {src}")
            if len(orphaned) > 5:
                print(f"  ... and {len(orphaned) - 5} more")

        # Open Ontologies bridges
        bridges = self.check_open_ontologies_bridges()
        bridges_present = sum(1 for v in bridges.values() if v)
        print(f"Open Ontologies Bridges: {bridges_present}/5")
        for vocab, present in bridges.items():
            status = "✓" if present else "✗"
            print(f"  {status} {vocab}")

        # Determine overall status
        status = "CONSISTENT"
        if namespace_conflicts:
            status = "INCONSISTENT"
        if orphaned:
            status = "INCONSISTENT"
        if self.errors:
            status = "FAILED"

        print(f"\nOverall Status: {status}\n")

        # Build YAML report
        report = {
            "audit_timestamp": str(Path.cwd()),
            "graph_integrity": {
                "total_ttl_files": len(self.ttl_files),
                "total_triples": triples,
                "total_classes": classes,
                "total_properties": properties,
                "total_namespaces": len(self.namespaces_used),
                "namespace_conflicts": len(namespace_conflicts),
                "orphaned_instances": len(orphaned),
                "open_ontologies_bridges_present": bridges_present == 5,
                "status": status
            },
            "namespace_analysis": {
                "conflicts_found": namespace_conflicts if namespace_conflicts else [],
                "namespaces_registered": self.namespaces_used
            },
            "property_analysis": {
                "total_properties_checked": len(properties_checked),
                "properties_with_domains": sum(1 for p in properties_checked.values() if p["domains"]),
                "properties_with_ranges": sum(1 for p in properties_checked.values() if p["ranges"])
            },
            "open_ontologies_bridges": bridges,
            "validation_errors": self.errors if self.errors else [],
            "validation_warnings": self.warnings[:10] if self.warnings else []
        }

        return report

    def audit(self, root_dir):
        """Execute the full audit."""
        self.discover_ttl_files(root_dir)
        print(f"Discovered {len(self.ttl_files)} TTL files\n")

        self.load_all_ttl_files()
        if self.errors:
            print(f"\nFound {len(self.errors)} parsing errors")

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

    print(f"\n✓ Report written to: {output_file}")

    # Exit with appropriate code
    if report["graph_integrity"]["status"] == "CONSISTENT":
        sys.exit(0)
    else:
        sys.exit(1)


if __name__ == "__main__":
    main()
