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
 * @file qhashtbl.c Hash-table container implementation.
 *
 * qhashtbl implements a hash table, which maps keys to values. Key is a unique
 * string and value is any non-null object. The creator qhashtbl() has a
 * parameter that affect its performance: initial hash range. The hash range
 * is the number of slots(pointers) in the hash table. in the case of a hash
 * collision, a single slots stores multiple elements using linked-list
 * structure, which must be searched sequentially. So lower range than the
 * number of elements decreases the space overhead but increases the number of
 * hash collisions and consequently it increases the time cost to look up an
 * element.
 *
 * @code
 *  [Internal Structure Example for 10-slot hash table]
 *
 *  RANGE    NAMED-OBJECT-LIST
 *  =====    =================
 *  [ 0 ] -> [hash=320,key3=value] -> [hash=210,key5=value] -> [hash=110,...]
 *  [ 1 ] -> [hash=1,key1=value]
 *  [ 2 ]
 *  [ 3 ] -> [hash=873,key4=value]
 *  [ 4 ] -> [hash=2674,key11=value] -> [hash=214,key5=value]
 *  [ 5 ] -> [hash=8545,key10=value]
 *  [ 6 ] -> [hash=9226,key9=value]
 *  [ 7 ]
 *  [ 8 ] -> [hash=8,key6=value] -> [hash=88,key8=value]
 *  [ 9 ] -> [hash=12439,key7=value]
 * @endcode
 *
 * @code
 *  // create a hash-table with 10 hash-index range.
 *  // Please be aware, the hash-index range 10 does not mean the number of
 *  // objects which can be stored. You can put as many as you want but if
 *  // this range is too small, hash conflict will happen and fetch time will
 *  // slightly increase.
 *  qhashtbl_t *tbl = qhashtbl(0, QHASHTBL_THREADSAFE);
 *
 *  // put objects into table.
 *  tbl->put(tbl, "sample1", "binary", 6);
 *  tbl->putstr(tbl, "sample2", "string");
 *  tbl->putint(tbl, "sample3", 1);
 *
 *  // debug print out
 *  tbl->debug(tbl, stdout, true);
 *
 *  // get objects
 *  void *sample1 = tbl->get(tbl, "sample1", &size, true);
 *  char *sample2 = tbl->getstr(tbl, "sample2", false);
 *  int  sample3  = tbl->getint(tbl, "sample3");
 *
 *  // sample1 is memalloced
 *  free(sample1);
 *
 *  // release table
 *  tbl->free(tbl);
 * @endcode
 */

#include "./qinternal.h"
#include "./qhash.h"
#include "./qhashtbl.h"

#define DEFAULT_INDEX_RANGE (100) /*!< default value of hash-index range */

char *
strdup(const char *s)
{
    if (NULL == s) {
        return NULL;
    }
    size_t n = __strlen(s);
    char *buffer = __malloc(n + 1);
    if (NULL != buffer) {
        memcpy(buffer, s, (uint32_t)n + 1);
    }
    return buffer;
}

bool
__strcmp(const char *left, const char *right)
{
    size_t left_len = __strlen(left);
    size_t right_len = __strlen(right);
    if (left_len != right_len)
        return false;

    while (left_len--) {
        if (*left++ != *right++)
            return false;
    }

    return true;
}

static bool is_same_tbl_key(qhashtbl_t *tbl, int64_t iter_key, int64_t target_key, uint32_t iter_hash, uint32_t hash) {
    if (TABLE_KEY_IS_INT(tbl)) {
        // TODO: support i128/u128
        if (iter_key == target_key) {
            return true;
        }
    } else {
        // when string key
        if (iter_hash == hash && __strcmp((char*) iter_key, (char*) target_key)) {
            return true;
        }
    }
    return false;
}

/**
 * Initialize hash table.
 *
 * @param range     initial size of index range. Value of 0 will use default
 * value, DEFAULT_INDEX_RANGE;
 * @param options   combination of initialization options.
 *
 * @return a pointer of malloced qhashtbl_t, otherwise returns NULL.
 * @retval errno will be set in error condition.
 *  - ENOMEM : Memory allocation failure.
 *
 * @code
 *  // create a hash-table.
 *  qhashtbl_t *basic_hashtbl = qhashtbl(0, 0);
 *
 *  // create a large hash-table for millions of keys with thread-safe option.
 *  qhashtbl_t *small_hashtbl = qhashtbl(1000000, QHASHTBL_THREADSAFE);
 * @endcode
 *
 * @note
 *   Setting the right range is a magic.
 *   In practice, pick a value between (total keys / 3) ~ (total keys * 2).
 *   Available options:
 *   - QHASHTBL_THREADSAFE - make it thread-safe.
 */
qhashtbl_t *
qhashtbl(size_t range, int8_t key_runtime_ty /* enum IRRuntimeType */, int options)
{
    if (range == 0) {
        range = DEFAULT_INDEX_RANGE;
    }

    qhashtbl_t *tbl = (qhashtbl_t *)__malloc(sizeof(qhashtbl_t));
    if (tbl == NULL)
        goto malloc_failure;
    tbl->num = 0;

    // allocate table space
    tbl->slots = (qhashtbl_obj_t **)__malloc(sizeof(qhashtbl_obj_t *) * range);
    if (tbl->slots == NULL)
        goto malloc_failure;
    __memset(tbl->slots, 0x0, sizeof(qhashtbl_obj_t *) * range);

    // set table range.
    tbl->range = range;

    tbl->key_runtime_ty = (enum IRRuntimeType) key_runtime_ty;

    return tbl;

malloc_failure:

    if (tbl) {
        qhashtbl_free(tbl);
    }
    return NULL;
}

// TODO: when key is i128/u128

uint32_t
qhashtbl_hash(qhashtbl_t *tbl, int64_t key)
{
    if (TABLE_KEY_IS_INT(tbl)) {
        return key;
    }
    // switch
    const char *name = (const char *)((intptr_t)key);
    return qhashmurmur3_32(name, __strlen(name));
}

/**
 * qhashtbl->put(): Put an object into this table.
 *
 * @param tbl       qhashtbl_t container pointer.
 * @param name      key name
 * @param data      data object
 * @param size      size of data object
 *
 * @return true if successful, otherwise returns false
 * @retval errno will be set in error condition.
 *  - EINVAL : Invalid argument.
 *  - ENOMEM : Memory allocation failure.
 */
inline bool
qhashtbl_put(qhashtbl_t *tbl, int64_t key, const void *data, size_t data_size)
{
    if ((!TABLE_KEY_IS_INT(tbl) && key == 0) || data == NULL) {
        return false;
    }

    // get hash integer
    uint32_t hash =
        qhashtbl_hash(tbl, key);
    int idx = hash % tbl->range;

    // find existence key
    qhashtbl_obj_t *obj;
    for (obj = tbl->slots[idx]; obj != NULL; obj = obj->next) {
        if (is_same_tbl_key(tbl, obj->key, key, obj->hash, hash)) {
            break;
        }
    }

    // duplicate object
    int64_t dupkey = key; // not use use strdup(name), because key maybe not str
    void *dupdata = __malloc(data_size);
    if (dupdata == NULL) {
        free((void *)dupdata);
        return false;
    }
    memcpy(dupdata, data, data_size);

    // put into table
    if (obj == NULL) {
        // insert
        obj = (qhashtbl_obj_t *)__malloc(sizeof(qhashtbl_obj_t));
        if (obj == NULL) {
            free(dupdata);
            return false;
        }

        if (tbl->slots[idx] != NULL) {
            // insert at the beginning
            obj->next = tbl->slots[idx];
        } else {
            obj->next = NULL;
        }
        tbl->slots[idx] = obj;

        // increase counter
        tbl->num++;
    }
    else {
        // replace data
        free(obj->data);
    }

    // set data
    obj->hash = hash;
    obj->key = dupkey;
    obj->data = dupdata; // keep pointer to the value
    obj->size = data_size;
    obj->has_value = true;

    return true;
}

/**
 * qhashtbl->putstr(): Put a string into this table.
 *
 * @param tbl       qhashtbl_t container pointer.
 * @param name      key name.
 * @param str       string data.
 *
 * @return true if successful, otherwise returns false.
 * @retval errno will be set in error condition.
 *  - EINVAL : Invalid argument.
 *  - ENOMEM : Memory allocation failure.
 */
bool
qhashtbl_putstr(qhashtbl_t *tbl, int64_t key, const char *str)
{
    return qhashtbl_put(tbl, key, str, (str != NULL) ? (__strlen(str) + 1) : 0);
}

/**
 * qhashtbl->get(): Get an object from this table.
 *
 * @param tbl       qhashtbl_t container pointer.
 * @param name      key name.
 * @param size      if not NULL, oject size will be stored.
 * @param newmem    whether or not to allocate memory for the data.
 *
 * @return a pointer of data if the key is found, otherwise returns NULL.
 * @retval errno will be set in error condition.
 *  - ENOENT : No such key found.
 *  - EINVAL : Invalid argument.
 *  - ENOMEM : Memory allocation failure.
 *
 * @code
 *  qhashtbl_t *tbl = qhashtbl(0, 0);
 *  (...codes...)
 *
 *  // with newmem flag unset
 *  size_t size;
 *  void *data = (struct myobj*)tbl->get(tbl, "key_name", &size, false);
 *
 *  // with newmem flag set
 *  size_t size;
 *  void *data  = (struct myobj*)tbl->get(tbl, "key_name", &size, true);
 *  free(data);
 * @endcode
 *
 * @note
 *  If newmem flag is set, returned data will be malloced and should be
 *  deallocated by user. Otherwise returned pointer will point internal buffer
 *  directly and should not be de-allocated by user. In thread-safe mode,
 *  newmem flag must be set to true always.
 */
inline void*
qhashtbl_get(qhashtbl_t *tbl, int64_t key, size_t *size, bool newmem)
{
    if (!TABLE_KEY_IS_INT(tbl) && key == 0) { // NULL when key is string
        return NULL;
    }

   // get hash integer
   uint32_t hash =
       qhashtbl_hash(tbl, key);
   int idx = hash % tbl->range;

    // find key
    qhashtbl_obj_t *obj;
    for (obj = tbl->slots[idx]; obj != NULL; obj = obj->next) {
        if (is_same_tbl_key(tbl, obj->key, key, obj->hash, hash)) {
            break;
        }
    }

    void *data = NULL;
    if (obj != NULL) {
        if (newmem == false) {
            data = obj->data;
        }
        else {
            data = __malloc(obj->size);
            if (data == NULL) {
                return NULL;
            }
            memcpy(data, obj->data, obj->size);
        }
        if (size != NULL && data != NULL)
            *size = obj->size;
    }

    return data; // return pointer to the value
}

/**
 * qhashtbl->getstr(): Finds an object and returns as string type.
 *
 * @param tbl       qhashtbl_t container pointer.
 * @param name      key name
 * @param newmem    whether or not to allocate memory for the data.
 *
 * @return a pointer of data if the key is found, otherwise returns NULL.
 * @retval errno will be set in error condition.
 *  - ENOENT : No such key found.
 *  - EINVAL : Invalid argument.
 *  - ENOMEM : Memory allocation failure.
 *
 * @note
 *  If newmem flag is set, returned data will be malloced and should be
 *  deallocated by user.
 */
char *
qhashtbl_getstr(qhashtbl_t *tbl, int64_t key, const bool newmem)
{
    return qhashtbl_get(tbl, key, NULL, newmem);
}

bool
qhashtbl_contains_key(qhashtbl_t *tbl, int64_t key)
{
    if (!TABLE_KEY_IS_INT(tbl) && key == 0) { // NULL when key is string
        return false;
    }

    uint32_t hash =
        qhashtbl_hash(tbl, key);
    int idx = hash % tbl->range;

    // find key
    qhashtbl_obj_t *obj;
    for (obj = tbl->slots[idx]; obj != NULL; obj = obj->next) {
        if (is_same_tbl_key(tbl, obj->key, key, obj->hash, hash)) {
            break;
        }
    }
    return obj != NULL;
}

/**
 * qhashtbl->remove(): Remove an object from this table.
 *
 * @param tbl   qhashtbl_t container pointer.
 * @param name  key name
 *
 * @return true if successful, otherwise(not found) returns false
 * @retval errno will be set in error condition.
 *  - ENOENT : No such key found.
 *  - EINVAL : Invalid argument.
 */
bool
qhashtbl_remove(qhashtbl_t *tbl, int64_t key)
{
    if (!TABLE_KEY_IS_INT(tbl) && key == 0) { // NULL when key is string
        return false;
    }

    uint32_t hash =
        qhashtbl_hash(tbl, key);
    int idx = hash % tbl->range;

    // find key
    bool found = false;
    qhashtbl_obj_t *prev = NULL;
    qhashtbl_obj_t *obj;
    for (obj = tbl->slots[idx]; obj != NULL; obj = obj->next) {
        if (is_same_tbl_key(tbl, obj->key, key, obj->hash, hash)) {
            // adjust link
            if (prev == NULL)
                tbl->slots[idx] = obj->next;
            else
                prev->next = obj->next;

            // remove
            free(obj->data);
            free(obj);

            found = true;
            tbl->num--;
            break;
        }

        prev = obj;
    }

    return found;
}

/**
 * qhashtbl->getnext(): Get next element.
 *
 * @param tbl       qhashtbl_t container pointer.
 * @param obj       found data will be stored in this object
 * @param newmem    whether or not to allocate memory for the data.
 *
 * @return true if found otherwise returns false
 * @retval errno will be set in error condition.
 *  - ENOENT : No next element.
 *  - EINVAL : Invalid argument.
 *  - ENOMEM : Memory allocation failure.
 *
 * @code
 *  qhashtbl_t *tbl = qhashtbl(0, 0);
 *  (...add data into list...)
 *
 *  qhashtbl_obj_t obj;
 *  __memset((void*)&obj, 0, sizeof(obj)); // must be cleared before call
 *  tbl->lock(tbl);  // lock it when thread condition is expected
 *  while(tbl->getnext(tbl, &obj, false) == true) {  // newmem is false
 *     printf("NAME=%s, DATA=%s, SIZE=%zu\n",
 *     obj.name, (char*)obj.data, obj.size);
 *     // do free obj.name and obj.data if newmem was set to true;
 *  }
 *  tbl->unlock(tbl);
 * @endcode
 *
 * @note
 *  locking must be provided on user code when all element scan must be
 *  guaranteed where multiple threads concurrently update the table.
 *  It's ok not to lock the table on the user code even in thread condition,
 *  when concurreny is importand and all element scan in a path doesn't need
 *  to be guaranteed. In this case, new data inserted during the traversal
 *  will be show up in this scan or next scan. Make sure newmem flag is set
 *  if deletion is expected during the scan.
 *  Object obj should be initialized with 0 by using __memset() before first call.
 */
inline bool __attribute__((artificial)) __attribute__((always_inline))
qhashtbl_getnext(qhashtbl_t *tbl, qhashtbl_obj_t *obj, const bool newmem)
{
    if (obj == NULL) {
        return NULL;
    }

    bool found = false;

    qhashtbl_obj_t *cursor = NULL;
    int prev_slot_index = obj->hash % tbl->range;
    int slot_index = 0;
    if (obj->has_value) {
        slot_index = prev_slot_index + 1;
        cursor = obj->next;
    }

    if (cursor != NULL) {
        // has link
        found = true;
    }
    else {
        // search from next index
        while(slot_index < tbl->range) {
            if (tbl->slots[slot_index]) {
                cursor = tbl->slots[slot_index];
                found = true;
                break;
            }

            slot_index++;
        }
    }

    if (cursor != NULL) {
        if (newmem == true) {
            obj->key = cursor->key;
            obj->data = __malloc(cursor->size);
            if (obj->data == NULL) {
                DEBUG("getnext(): Unable to allocate memory.");
                free(obj->data);
                return false;
            }
            memcpy(obj->data, cursor->data, cursor->size);
            obj->has_value = cursor->has_value;
        }
        else {
            obj->key = cursor->key;
            obj->data = cursor->data;
            obj->has_value = cursor->has_value;
        }
        obj->hash = cursor->hash;
        obj->size = cursor->size;
        obj->next = cursor->next;
    }

    return found;
}

/**
 * qhashtbl->size(): Returns the number of keys in this hashtable.
 *
 * @param tbl   qhashtbl_t container pointer.
 *
 * @return number of elements stored
 */
size_t
qhashtbl_size(qhashtbl_t *tbl)
{
    return tbl->num;
}

/**
 * qhashtbl->clear(): Clears this hashtable so that it contains no keys.
 *
 * @param tbl   qhashtbl_t container pointer.
 */
void
qhashtbl_clear(qhashtbl_t *tbl)
{
    int idx;
    for (idx = 0; idx < tbl->range && tbl->num > 0; idx++) {
        if (tbl->slots[idx] == NULL)
            continue;
        qhashtbl_obj_t *obj = tbl->slots[idx];
        tbl->slots[idx] = NULL;
        while (obj != NULL) {
            qhashtbl_obj_t *next = obj->next;
            free(obj->data);
            free(obj);
            obj = next;

            tbl->num--;
        }
    }
}

/**
 * qhashtbl->free(): De-allocate hash table
 *
 * @param tbl   qhashtbl_t container pointer.
 */
void
qhashtbl_free(qhashtbl_t *tbl)
{
    qhashtbl_clear(tbl);
    free(tbl->slots);
    free(tbl);
}