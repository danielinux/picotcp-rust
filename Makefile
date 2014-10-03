RUSTC?=rustc
CC?=gcc
PICOTCP?=~/picotcp

RUST_CODEGEN=

all: picotcp_rs

libpicotcp.a:
	cp $(PICOTCP)/build/lib/libpicotcp.a .

libdevtun.a: pico_dev_tun.o
	ar cru $@ $<
	ranlib $@

pico_dev_tun.o: $(PICOTCP)/modules/pico_dev_tun.c libpicotcp.a
	$(CC) -c -o $@ $< -I $(PICOTCP)/build/include -I $(PICOTCP)/build/modules

	
	
picotcp_rs: src/picotcp_rs.rs libpicotcp.a libdevtun.a
	$(RUSTC) $(RUST_CODEGEN) $<


clean: 
	rm -f picotcp_rs *.a *.o

.PHONY: clean
