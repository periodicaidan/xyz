LESS_SOURCES = $(wildcard static/styles/*.less)
CSS_OUTPUTS = $(LESS_SOURCES:%.less=%.css)

COMPONENTS_DIR = static/components
ELM_SOURCES = $(wildcard ${COMPONENTS_DIR}/src/*.elm)
JS_OUTPUTS = $(ELM_SOURCES:${COMPONENTS_DIR}/src/%.elm=${COMPONENTS_DIR}/%.js)

all: styles, components

styles: ${CSS_OUTPUTS}

components: ${JS_OUTPUTS}

%.css: %.less
	lessc $< $@

Components.js: ${ELM_SOURCES}
	elm make $< --optimize --output=${COMPONENTS_DIR}/$@