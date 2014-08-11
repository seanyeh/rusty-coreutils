rustc = rustc

PROGRAMS = base64 basename comm seq sleep wc yes

all: $(PROGRAMS)


# Create compile task for each program
define TEMPLATE =

$(1): bin/$(1)
bin/$(1):
	$(rustc) $(1)/$(1).rs -o bin/$(1)

endef

$(foreach prog,$(PROGRAMS),$(eval $(call TEMPLATE,$(prog))))


.PHONY : clean
clean:
	mkdir -p bin
	rm -rf bin/*
