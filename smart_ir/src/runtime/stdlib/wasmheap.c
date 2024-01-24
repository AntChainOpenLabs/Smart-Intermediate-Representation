// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

#include "./stdlib.h"

#define WASM_PAGE_SIZE (65536)

#define ARRAY_SIZE(ARRAY) (sizeof(ARRAY) / sizeof((ARRAY)[0]))
#define __abort(x) (0)

static unsigned int heap_ptr;
static unsigned int heap_top;
extern unsigned char __heap_base; // set by lld
static void *builtin_cache[8];

struct heap_block {
    size_t size;
    struct heap_block *prev; // unset if block allocated
    struct heap_block *next; // unset if block allocated
    unsigned char data[0];
};

// free blocks, ordered per their memory address.
struct heap_blocks {
    bool fixed_size;
    size_t size; // if fixed size, this indicates the block size.
                 // if not fixed, this indicates the minimum block size.
    struct heap_block start;
    struct heap_block end;
};

// all the free blocks: fixed size blocks of 4, 8, 16 and 64 bytes and then one
// free list for varying sized blocks of 128 bytes or more.
static struct heap_blocks heap_free[5] = {
    { true, 4 }, { true, 8 }, { true, 16 }, { true, 64 }, { false, 128 },
};

#ifdef DEBUG
#define HEAP_CHECK(blocks) heap_check(__FUNCTION__, blocks)
#else
#define HEAP_CHECK(blocks)
#endif

static void
init_free()
{
    for (int i = 0; i < ARRAY_SIZE(heap_free); i++) {
        heap_free[i].start = (struct heap_block){ 0, NULL, &heap_free[i].end };
        heap_free[i].end = (struct heap_block){ 0, &heap_free[i].start, NULL };
    }

    for (int i = 0; i < ARRAY_SIZE(builtin_cache); i++) {
        builtin_cache[i] = NULL;
    }
}

static void
heap_check(const char *name, struct heap_blocks *blocks)
{
    struct heap_block *start = &blocks->start;
    struct heap_block *end = &blocks->end;

    for (struct heap_block *b = start->next, *prev = start; b != end;
         prev = b, b = b->next) {
        if (prev == NULL || b == NULL || b->prev != prev) {
            __abort(name);
        }
    }

    for (struct heap_block *b = end->prev, *next = end; b != start;
         next = b, b = b->prev) {
        if (next == NULL || b == NULL || b->next != next) {
            __abort(name);
        }
    }
}

// try removing the last block(s) from the free list and adjusting the heap
// pointer accordingly.
static bool
compact_free(struct heap_blocks *blocks)
{
    struct heap_block *start = &blocks->start;
    struct heap_block *end = &blocks->end;
    unsigned int old_heap_ptr = heap_ptr;

    while (1) {
        struct heap_block *last = end->prev;

        if (last == start) {
            break;
        }

        if (((void *)(&last->data[0]) + last->size) != (void *)heap_ptr) {
            break;
        }

        heap_ptr -= sizeof(struct heap_block) + last->size;
        last->prev->next = end;
        end->prev = last->prev;
    }

    HEAP_CHECK(blocks);
    return old_heap_ptr != heap_ptr;
}

// NOTE(sr): In internal/compiler/wasm, we append segments to the data section.
// Since our memory layout is
//   |  <-- stack | -- data -- | heap -->  |
// we need to adjust the border between data and heap, i.e., where the heap
// starts. When initializing a module, the Start function emitted by the
// compiler will call this function with the new heap base.

static bool inited_before = false;

void
__init_heap()
{
    if (inited_before) {
        // disable re-call abi using the same vm(memory not free)
        return;
    }
    heap_ptr = __builtin_wasm_memory_grow(0, 0) * WASM_PAGE_SIZE;
    heap_top = __builtin_wasm_memory_grow(0, 1) * WASM_PAGE_SIZE;
    init_free();

    void* tmp = __malloc(1); // malloc 1 byte to let memory grow to available size
    free(tmp);

    inited_before = true;
}

void
__malloc_init_test(void)
{
    __init_heap();
}

static struct heap_block *
____malloc_reuse_fixed(struct heap_blocks *blocks);
static struct heap_block *
____malloc_reuse_varying(struct heap_blocks *blocks, size_t size);

unsigned int
heap_ptr_get(void)
{
    return heap_ptr;
}

unsigned int
heap_top_get(void)
{
    return heap_top;
}

void
heap_ptr_set(unsigned int ptr)
{
    heap_ptr = ptr;
    init_free();
}

void
heap_top_set(unsigned int top)
{
    heap_top = top;
    init_free();
}

// returns the free list applicable for the requested size.
static struct heap_blocks *
__blocks(size_t size)
{
    for (int i = 0; i < ARRAY_SIZE(heap_free) - 1; i++) {
        struct heap_blocks *candidate = &heap_free[i];

        if (size <= candidate->size) {
            return candidate;
        }
    }

    return &heap_free[ARRAY_SIZE(heap_free) - 1];
}

static void *
____malloc_new_allocation(size_t size)
{
    unsigned int ptr = heap_ptr;
    size_t block_size = sizeof(struct heap_block) + size;
    heap_ptr += block_size;

    if (heap_ptr >= heap_top) {
        unsigned int pages = (block_size / WASM_PAGE_SIZE) + 1;
        if (__builtin_wasm_memory_grow(0, pages) == -1) {
            __abort("__malloc: failed");
        };
        heap_top += (pages * WASM_PAGE_SIZE);
    }

    struct heap_block *b = (void *)ptr;
    b->size = size;
    b->prev = NULL;
    b->next = NULL;

    return b->data;
}

#ifndef CC_LIB_TEST_MOCK
void *__attribute__((noinline)) __malloc(size_t size)
{
    // Look for the first free block that is large enough. Split the found block
    // if necessary.

    struct heap_blocks *blocks = __blocks(size);
    HEAP_CHECK(blocks);

    struct heap_block *b = blocks->fixed_size
                               ? ____malloc_reuse_fixed(blocks)
                               : ____malloc_reuse_varying(blocks, size);

    if (b != NULL) {
        return b->data;
    }

    // Allocate a new block.

    if (blocks->fixed_size) {
        size = blocks->size;
    }

    return ____malloc_new_allocation(size);
}
#endif // CC_LIB_TEST_MOCK

// returns a free block from the list, if available.
static struct heap_block *
____malloc_reuse_fixed(struct heap_blocks *blocks)
{
    struct heap_block *end = &blocks->end;
    struct heap_block *b = blocks->start.next;

    if (b != NULL && b != end) {
        b->prev->next = b->next;
        b->next->prev = b->prev;
        b->prev = NULL;
        b->next = NULL;

        HEAP_CHECK(blocks);

        return b;
    }

    return NULL;
}

// finds a free block at least of given size, splitting the found block if the
// remaining block exceeds the minimum size.
static struct heap_block *
____malloc_reuse_varying(struct heap_blocks *blocks, size_t size)
{
    struct heap_block *start = &blocks->start;
    struct heap_block *end = &blocks->end;
    size_t min_size = blocks->size;

    for (struct heap_block *b = start->next; b != end; b = b->next) {
        if (b->size >= (sizeof(struct heap_block) + min_size + size)) {
            struct heap_block *remaining = (void *)(&b->data[0]) + size;
            remaining->size = b->size - (sizeof(struct heap_block) + size);
            remaining->prev = b->prev;
            remaining->next = b->next;
            remaining->prev->next = remaining;
            remaining->next->prev = remaining;

            b->size = size;
            b->prev = NULL;
            b->next = NULL;

            HEAP_CHECK(blocks);

            return b;
        }
        else if (b->size >= size) {
            b->prev->next = b->next;
            b->next->prev = b->prev;
            b->prev = NULL;
            b->next = NULL;

            HEAP_CHECK(blocks);

            return b;
        }
    }
    return NULL;
}

#ifndef CC_LIB_TEST_MOCK
void
free(void *ptr)
{
    struct heap_block *block = ptr - sizeof(struct heap_block);

#ifdef DEBUG
    if (ptr == NULL) {
        __abort("free: null pointer");
    }

    if (block->prev != NULL || block->next != NULL) {
        __abort("free: double free");
    }
#endif

    struct heap_blocks *blocks = __blocks(block->size);
    struct heap_block *start = &blocks->start;
    struct heap_block *end = &blocks->end;
    bool fixed_size = blocks->fixed_size;

    HEAP_CHECK(blocks);

    // Find the free block available just before this block and try to
    // defragment, by trying to merge with this block with the found
    // block and the one after.

    struct heap_block *prev = start;
    for (struct heap_block *b = prev->next; b < block && b != end;
         prev = b, b = b->next)
        ;

    if (!fixed_size) {
        struct heap_block *prev_end = (void *)(&prev->data[0]) + prev->size;
        struct heap_block *block_end = (void *)(&block->data[0]) + block->size;

        if (prev_end == block) {
            prev->size += sizeof(struct heap_block) + block->size;
            compact_free(blocks);
            return;
        }

        if (block_end == prev->next) {
            struct heap_block *next = prev->next;
            block->prev = prev;
            block->next = next->next;
            block->size += sizeof(struct heap_block) + next->size;

            prev->next = block;
            block->next->prev = block;
            compact_free(blocks);
            return;
        }
    }

    // List the block as free.

    block->prev = prev;
    block->next = prev->next;
    prev->next = block;
    block->next->prev = block;
    compact_free(blocks);
}

void *
realloc(void *ptr, size_t size)
{
    struct heap_block *block = ptr - sizeof(struct heap_block);
    void *p = __malloc(size);

    memcpy(p, ptr, block->size < size ? block->size : size);
    free(ptr);
    return p;
}
#endif // CC_LIB_TEST_MOCK

static void **
__builtin_cache(size_t i)
{
    if (i >= ARRAY_SIZE(builtin_cache)) {
        __abort("__malloc: illegal builtin cache index");
    }

    return &builtin_cache[i];
}

void *
builtin_cache_get(size_t i)
{
    return *__builtin_cache(i);
}

void
builtin_cache_set(size_t i, void *p)
{
    *__builtin_cache(i) = p;
}

// Compact all the free blocks. This is for testing only.
void
heap_compact(void)
{
    for (bool progress = true; progress;) {
        progress = false;
        for (int i = 0; i < ARRAY_SIZE(heap_free); i++) {
            progress |= compact_free(&heap_free[i]);
        }
    }
}

// Count the number of free blocks. This is for testing only.
size_t
heap_free_blocks(void)
{
    size_t blocks1 = 0, blocks2 = 0;

    for (int i = 0; i < ARRAY_SIZE(heap_free); i++) {
        for (struct heap_block *b = heap_free[i].start.next;
             b != &heap_free[i].end; b = b->next, blocks1++)
            ;
        for (struct heap_block *b = heap_free[i].end.prev;
             b != &heap_free[i].start; b = b->prev, blocks2++)
            ;

        if (blocks1 != blocks2) {
            __abort("__malloc: corrupted heap");
        }

        HEAP_CHECK(&heap_free[i]);
    }

    return blocks1;
}
