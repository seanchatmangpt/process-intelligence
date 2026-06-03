# ============================================================
# Makefile — PhD Thesis Manufacturing Pipeline
# Sean Chatman, Process Intelligence Research Foundry
# ============================================================
#
# Targets:
#   thesis       Full validated build (validate then pdf-build)
#   validate     Run Python validation script
#   pdf-build    Compile LaTeX to PDF (two-pass for TOC/refs)
#   clean        Remove build artifacts (aux, log, toc, out)
#   receipts     Run validation in receipt-only mode
#   watch        Watch for changes and rebuild (requires fswatch)
#   wordcount    Count words in compiled PDF (requires pdftotext)
#
# ============================================================

THESIS_DIR  := /Users/sac/process-intelligence/phd-thesis
BUILD_DIR   := $(THESIS_DIR)/build
PDFLATEX    := /Library/TeX/texbin/pdflatex
BIBTEX      := /Library/TeX/texbin/bibtex
PYTHON      := python3
VALIDATE    := $(PYTHON) $(THESIS_DIR)/scripts/validate_thesis.py
MAIN        := main
LATEX_FLAGS := -interaction=nonstopmode -output-directory=$(BUILD_DIR)

.PHONY: thesis validate pdf-build clean receipts watch wordcount help

## thesis: Full validated build — runs validate then pdf-build
thesis: validate pdf-build
	@echo ""
	@echo "=== Thesis manufacturing complete ==="
	@echo "Output: $(BUILD_DIR)/$(MAIN).pdf"

## validate: Run the thesis validation script
validate:
	@echo "=== Running thesis validation ==="
	@cd $(THESIS_DIR) && $(VALIDATE)

## pdf-build: Compile LaTeX to PDF (two-pass for cross-references and TOC)
pdf-build:
	@echo "=== PDF build pass 1/2 ==="
	@mkdir -p $(BUILD_DIR)
	@cd $(THESIS_DIR) && $(PDFLATEX) $(LATEX_FLAGS) $(MAIN).tex
	@echo ""
	@echo "=== Running BibTeX ==="
	@cd $(BUILD_DIR) && $(BIBTEX) $(MAIN) || true
	@echo ""
	@echo "=== PDF build pass 2/2 (resolving references) ==="
	@cd $(THESIS_DIR) && $(PDFLATEX) $(LATEX_FLAGS) $(MAIN).tex
	@echo ""
	@echo "=== PDF build complete: $(BUILD_DIR)/$(MAIN).pdf ==="

## clean: Remove LaTeX build artifacts (preserves PDF)
clean:
	@echo "=== Cleaning build artifacts ==="
	@rm -f $(BUILD_DIR)/*.aux
	@rm -f $(BUILD_DIR)/*.log
	@rm -f $(BUILD_DIR)/*.toc
	@rm -f $(BUILD_DIR)/*.out
	@rm -f $(BUILD_DIR)/*.bbl
	@rm -f $(BUILD_DIR)/*.blg
	@rm -f $(BUILD_DIR)/*.lof
	@rm -f $(BUILD_DIR)/*.lot
	@rm -f $(BUILD_DIR)/*.fls
	@rm -f $(BUILD_DIR)/*.fdb_latexmk
	@echo "=== Clean complete (PDF preserved) ==="

## clean-all: Remove ALL build artifacts including PDF
clean-all: clean
	@rm -f $(BUILD_DIR)/*.pdf
	@echo "=== Full clean complete ==="

## receipts: Run validation in receipt-only mode
receipts:
	@echo "=== Running receipt-only validation ==="
	@cd $(THESIS_DIR) && $(VALIDATE) --receipt-only

## watch: Watch for TeX changes and rebuild (requires fswatch)
watch:
	@echo "=== Watching for changes in $(THESIS_DIR) ==="
	@echo "=== Press Ctrl-C to stop ==="
	@fswatch -o $(THESIS_DIR)/chapters $(THESIS_DIR)/frontmatter \
	         $(THESIS_DIR)/projects $(THESIS_DIR)/main.tex \
	         $(THESIS_DIR)/glossary.tex | \
	  xargs -n1 -I{} make pdf-build

## wordcount: Estimate word count from compiled PDF
wordcount:
	@if [ -f "$(BUILD_DIR)/$(MAIN).pdf" ]; then \
	  echo "=== Word count estimate ==="; \
	  /Library/TeX/texbin/../../../bin/pdftotext $(BUILD_DIR)/$(MAIN).pdf - 2>/dev/null | wc -w || \
	  echo "pdftotext not available — install poppler for word count"; \
	else \
	  echo "No PDF found. Run 'make pdf-build' first."; \
	fi

## help: Show available targets
help:
	@echo ""
	@echo "PhD Thesis Manufacturing Pipeline"
	@echo "================================="
	@grep -E '^## ' $(MAKEFILE_LIST) | sed 's/## /  /'
	@echo ""
