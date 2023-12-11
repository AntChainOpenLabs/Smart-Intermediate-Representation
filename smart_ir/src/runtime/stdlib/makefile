CC=clang
CFLAGS=$(TARGET_FLAGS) -emit-llvm -O3 -ffreestanding -fno-builtin -Wall -Wno-unused-function -ftrapv -v

wasm/%.bc: %.c
	$(CC) -c $(CFLAGS) $< -o $@

WASM=$(addprefix wasm/,base64.bc)              \
     $(addprefix wasm/,call_log.bc)            \
     $(addprefix wasm/,cJSON.bc)               \
     $(addprefix wasm/,data_stream.bc)         \
     $(addprefix wasm/,data_stream_builtin.bc) \
     $(addprefix wasm/,hex.bc)                 \
     $(addprefix wasm/,json.bc)                \
     $(addprefix wasm/,math.bc)                \
     $(addprefix wasm/,mycov.bc)               \
     $(addprefix wasm/,mycrypto.bc)            \
     $(addprefix wasm/,qhash.bc)               \
     $(addprefix wasm/,qhashtbl.bc)            \
     $(addprefix wasm/,qstring.bc)             \
     $(addprefix wasm/,qvector.bc)             \
     $(addprefix wasm/,rlp.bc)                 \
     $(addprefix wasm/,ir_type.bc)             \
     $(addprefix wasm/,ssz.bc)                 \
     $(addprefix wasm/,stdlib.bc)              \
     $(addprefix wasm/,stream.bc)              \
     $(addprefix wasm/,storage_t.bc)           \
     $(addprefix wasm/,wasmheap.bc)            \
     $(addprefix wasm/,chain.bc)

all: $(WASM) makefile echo

$(WASM): TARGET_FLAGS=--target=wasm32

echo:
	echo "stdlib make done"

clean:
	rm -rf wasm/*.bc
