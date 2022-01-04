SHELL=/bin/sh
DOCNAME=paper

.SUFFIXES:
.SUFFIXES: .bib .pdf .tex
.PHONY: clean

run: $(DOCNAME).pdf

$(DOCNAME).pdf: $(DOCNAME).bbl $(DOCNAME).tex
	xelatex $(DOCNAME).tex -draftmode
	xelatex $(DOCNAME).tex 

$(DOCNAME).bbl: $(DOCNAME).aux
	bibtex $(DOCNAME).aux

$(DOCNAME).aux: $(DOCNAME).bib
	xelatex $(DOCNAME).tex -draftmode
	xelatex $(DOCNAME).tex -draftmode

clean:
	rm -rf *.aux *.lof *.log *.lot *.toc *.bbl *.blg *pdf