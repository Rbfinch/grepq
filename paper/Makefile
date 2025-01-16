clean: paper.pdf
	rm paper.pdf

all: paper.md paper.bib
	pandoc paper/paper.md -o paper/paper.pdf --metadata title="" --metadata author="" --pdf-engine=xelatex --citeproc --bibliography=paper/paper.bib