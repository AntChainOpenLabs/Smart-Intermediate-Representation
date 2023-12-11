/******************************************************************************
 * qLibc
 *
 * Copyright (c) 2010-2015 Seungyoung Kim.
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice,
 *    this list of conditions and the following disclaimer.
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS"
 * AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE
 * IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE
 * ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE
 * LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR
 * CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF
 * SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR PROFITS; OR BUSINESS
 * INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF LIABILITY, WHETHER IN
 * CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING NEGLIGENCE OR OTHERWISE)
 * ARISING IN ANY WAY OUT OF THE USE OF THIS SOFTWARE, EVEN IF ADVISED OF THE
 * POSSIBILITY OF SUCH DAMAGE.
 *****************************************************************************/

/**
 * Hash Table container.
 *
 * @file qhashtbl.h
 */

#ifndef QHASHTBL_H
#define QHASHTBL_H

#include <stdbool.h>
#include <stdint.h>
#include "./stdlib.h"
#include "./ir_type.h"

#ifdef __cplusplus
extern "C" {
#endif

/* types */
typedef struct qhashtbl_s qhashtbl_t;
typedef struct qhashtbl_obj_s qhashtbl_obj_t;

enum {
    QHASHTBL_THREADSAFE = (0x01) /*!< make it thread-safe */
};

extern char *
strdup(const char *s);
extern bool
__strcmp(const char *left, const char *right);

/* member functions
 *
 * All the member functions can be accessed in both ways:
 *  - tbl->put(tbl, ...);      // easier to switch the container type to other
 * kinds.
 *  - qhashtbl_put(tbl, ...);  // where avoiding pointer overhead is preferred.
 */
extern qhashtbl_t *
qhashtbl(size_t range, int8_t key_runtime_ty /* enum IRRuntimeType */, int options); /*!< qhashtbl constructor */

extern bool
qhashtbl_put(qhashtbl_t *tbl, int64_t key, const void *data, size_t size);
extern bool
qhashtbl_putstr(qhashtbl_t *tbl, int64_t key, const char *str);

extern void *
qhashtbl_get(qhashtbl_t *tbl, int64_t key, size_t *size, bool newmem);
extern char *
qhashtbl_getstr(qhashtbl_t *tbl, int64_t key, bool newmem);
extern bool
qhashtbl_contains_key(qhashtbl_t *tbl, int64_t key);

extern bool
qhashtbl_remove(qhashtbl_t *tbl, int64_t key);

extern bool
qhashtbl_getnext(qhashtbl_t *tbl, qhashtbl_obj_t *obj, bool newmem);

extern size_t
qhashtbl_size(qhashtbl_t *tbl);
extern void
qhashtbl_clear(qhashtbl_t *tbl);
extern void
qhashtbl_free(qhashtbl_t *tbl);

/**
 * qhashtbl container object structure
 */
struct qhashtbl_s {
    /* private variables - do not access directly */
    size_t num;             /*!< number of objects in this table */
    size_t range;           /*!< hash range, vertical number of slots */
    enum IRRuntimeType key_runtime_ty;
    qhashtbl_obj_t **slots; /*!< slot pointer container */
};

/**
 * qhashtbl object data structure
 */
struct qhashtbl_obj_s {
    uint32_t hash; /*!< 32bit-hash value of object name */
    int64_t key;    /*!< object key */
    void *data;    /*!< data */
    size_t size;   /*!< data size */
    bool has_value; /* used to judge the obj has key(eg. when key is integer type and key==0) */

    qhashtbl_obj_t *next; /*!< for chaining next collision object */
};

#define TABLE_KEY_IS_INT(tbl) ((tbl)->key_runtime_ty <= IRRuntimeIntegerTypeMaxEnum)

#ifdef __cplusplus
}
#endif

#endif /* QHASHTBL_H */
