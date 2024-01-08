// Copyright (c) The Ant Group Core Contributors
// Copyright (c) The Smart Intermediate Representation Contributors
// SPDX-License-Identifier: Apache-2.0

// clang --target=wasm32 -c -emit-llvm -O3 -ffreestanding -fno-builtin -Wall
// stdlib.c

#include "stdlib.h"
#define MAX_ITOA_STR_SIZE 64
#define assert(x) (0)

#ifdef CC_LIB_TEST_MOCK
// co_call hostapi mock
extern int32_t co_call(const char *contract,
                             uint32_t contract_length,
                             const char *method,
                             uint32_t method_length,
                             const char *argpack,
                             uint32_t argpack_length) {
    return 0; // mock
}
// revert hostapi mock
extern void revert(int32_t error_code, const char* error_msg, uint32_t error_msg_len) {
    // mock
}
// get_call_result_length hostapi mock
extern int32_t get_call_result_length() {
    return 0; // mock
}
// get_call_result hostapi mock
extern void get_call_result(char *result) {
    //mock
}
#endif // CC_LIB_TEST_MOCK

#ifndef CC_LIB_TEST_MOCK
// sometimes llvm optimizers will include llvm.memset to set default value for struct
// which will import memset extern dependency
// so we add memset implementation for link
void*
memset(void *dest, uint8_t val, size_t length) {
    __memset(dest, val, length);
    return dest;
}
#endif

// co_call hostapi
extern int32_t co_call(const char *contract,
                             uint32_t contract_length,
                             const char *method,
                             uint32_t method_length,
                             const char *argpack,
                             uint32_t argpack_length);
// revert hostapi
extern void revert(int32_t error_code, const char* error_msg, uint32_t error_msg_len);
// get_call_result_length hostapi
extern int32_t get_call_result_length(void);
// get_call_result hostapi
extern void get_call_result(char *result);

uint256_t div256_u256_rem(uint256_t dividend, uint256_t divisor, uint256_t *remainder);

uint256_t div256_u256(uint256_t dividend, uint256_t divisor);

void
runtime_abort(char *msg, uint32_t msg_length,
              struct RuntimeContext *runtime_context)
{
    char *line_str = builtin_i32_toa(runtime_context->line, 10);
    char *col_str = builtin_i32_toa(runtime_context->col, 10);
    int i = 0;
    const char *msgs[] = { msg,    ", ",     runtime_context->file_name,
                           ":",    line_str, ":",
                           col_str };
    uint32_t offset[sizeof(msgs) / sizeof(char *) + 1];
    uint32_t msgs_num = sizeof(msgs) / sizeof(char *);
    uint32_t total_len = 0;
    for (i = 0; i < msgs_num; i++) {
        offset[i] = __strlen(msgs[i]);
        total_len += offset[i];
    }
    char *abort_msg = __malloc(total_len * sizeof(char));
    uint32_t cur_offset = 0;
    for (i = 0; i < msgs_num; i++) {
        memcpy(abort_msg + cur_offset, msgs[i], offset[i]);
        cur_offset += offset[i];
    }
    IR_ABORT(abort_msg, total_len);
    free(line_str);
    free(col_str);
    free(abort_msg);
}

void
__memset(void *_dest, uint8_t val, size_t length)
{
    if (length == 0) {
        return;
    }
    uint8_t *dest = _dest;

    do {
        *dest++ = val;
    } while (--length);
}

size_t
__strlen(const char *s)
{
    size_t count = 0;
    while (*s) {
        count++;
        s++;
    }
    return count;
}


#ifndef CC_LIB_TEST_MOCK
/*
 * Our memcpy can only deal with multiples of 8 bytes. This is
 * enough for simple allocator below.
 */
void *
memcpy(void *_dest, const void *_src, uint32_t length)
{
    uint8_t *dest = _dest;
    const uint8_t *src = _src;

    while (length--) {
        *dest++ = *src++;
    }
    return _dest;
}
#endif // CC_LIB_TEST_MOCK

bool
__memcmp(uint8_t *left, uint32_t left_len, uint8_t *right, uint32_t right_len)
{
    if (left_len != right_len)
        return false;

    while (left_len--) {
        if (*left++ != *right++)
            return false;
    }

    return true;
}


/*
 * Fast-ish clear, 8 bytes at a time.
 */
void
__bzero8(void *_dest, uint32_t length)
{
    uint64_t *dest = _dest;

    while (length--) {
        *dest++ = 0;
    }
}

// This function is used for abi decoding integers.
// ABI encoding is big endian, and can have integers of 8 to 256
// bits (1 to 32 bytes). This function copies length bytes and
// reverses the order since wasm is little endian.
void
__be32toleN(uint8_t *from, uint8_t *to, uint32_t length)
{
    from += 31;

    do {
        *to++ = *from--;
    } while (--length);
}

void
__beNtoleN(uint8_t *from, uint8_t *to, uint32_t length)
{
    from += length;

    do {
        *to++ = *--from;
    } while (--length);
}

// This function is for used for abi encoding integers
// ABI encoding is big endian.
void
__leNtobe32(uint8_t *from, uint8_t *to, uint32_t length)
{
    to += 31;

    do {
        *to-- = *from++;
    } while (--length);
}

void
__leNtobeN(uint8_t *from, uint8_t *to, uint32_t length)
{
    to += length;

    do {
        *--to = *from++;
    } while (--length);
}

// sabre wants the storage keys as a hex string. Convert the
// uint256 pointed to be by v into a hex string
char *
__u256ptohex(uint8_t *v, char *str)
{
    // the uint256 will be stored little endian so fill it in
    // reverse
    str += 63;

    for (int i = 0; i < 32; i++) {
        uint8_t l = (v[i] & 0x0f);
        *str-- = l > 9 ? l + 'a' : '0' + l;
        uint8_t h = (v[i] >> 4);
        *str-- = h > 9 ? h + 'a' : '0' + h;
    }

    return str;
}

// Create a new vector. If initial is -1 then clear the data. This
// is done since a null pointer valid in wasm
struct vector *
vector_new(uint32_t length, uint32_t size, uint8_t *initial)
{
    struct vector *v;
    size_t size_array = length * size;

    v = __malloc(sizeof(*v));
    v->len = length;
    v->cap = length;
    if (size_array > 0) {
        v->data = __malloc(size_array + 1);
        // set last byte to 0
        v->data[size_array] = 0;
    }
    else {
        v->data = NULL;
    }
    uint8_t *data = v->data;

    // If not NULL pointer
    if (initial) {
        memcpy(data, initial, size_array);
    }
    else {
        __memset(data, 0, size_array);
    }

    return v;
}

uint64_t
vector_hash(struct vector *v)
{
    uint64_t hash = 0;
    uint8_t *data = v->data;
    uint32_t len = v->len;

    while (len--) {
        hash += *data;
    }

    return hash;
}

struct vector *
vector_concat(struct vector *left_vec, struct vector *right_vec)
{
    uint8_t *left = left_vec->data;
    uint32_t left_len = left_vec->len;
    uint8_t *right = right_vec->data;
    uint32_t right_len = right_vec->len;
    size_t size_array = left_len + right_len;
    struct vector *v = __malloc(sizeof(*v) + size_array);
    v->len = size_array;
    v->cap = size_array;
    v->data = __malloc(size_array);

    uint8_t *data = v->data;

    while (left_len--) {
        *data++ = *left++;
    }

    while (right_len--) {
        *data++ = *right++;
    }

    return v;
}

struct vector *
vector_copy(struct vector *value)
{
    size_t size_array = value->len;
    struct vector *v = __malloc(sizeof(*v) + size_array);
    v->len = size_array;
    v->cap = size_array;
    v->data = __malloc(size_array); // need to be initialized (Otherwise, it
                                    // always points to the same memory)

    uint8_t *data = v->data;

    memcpy(data, value->data, size_array);

    return v;
}

void
vector_copy_to(struct vector *value, struct vector *v)
{
    size_t size_array = value->len;

    v->len = size_array;
    v->cap = size_array;

    uint8_t *data = v->data;

    memcpy(data, value->data, size_array);
}

uint8_t *
vector_bytes(struct vector *v)
{
    return v->data;
}

uint32_t
vector_len(struct vector *v)
{
    return v->len;
}

/// Utility function to decode a ULEB128 value to a buffer.
/// Returns the length in bytes of the encoded value.
int32_t
decode_uleb128(int32_t *v, uint8_t *buf, int32_t offset, int32_t len)
{
    char msg[] = "DataStreamDecodeError: decode offset out of range";
    uint8_t *ptr = buf + offset;
    if (offset >= len) {
        goto fail;
    }
    int32_t result = *(ptr++);
    if (result > 0x7f) {
        if (offset + 1 >= len)
            goto fail;
        int cur = *(ptr++);
        result = (result & 0x7f) | ((cur & 0x7f) << 7);
        if (cur > 0x7f) {
            if (offset + 2 >= len) {
                goto fail;
            }
            cur = *(ptr++);
            result |= (cur & 0x7f) << 14;
            if (cur > 0x7f) {
                if (offset + 3 >= len) {
                    goto fail;
                }
                cur = *(ptr++);
                result |= (cur & 0x7f) << 21;
                if (cur > 0x7f) {
                    /*
                     * Note: We don't check to see if cur is out
                     * of range here, meaning we tolerate garbage
                     * in the high four-order bits.
                     */
                    cur = *(ptr++);
                    result |= cur << 28;
                    return 5;
                }
                else {
                    *v = result;
                    return 4;
                }
            }
            else {
                *v = result;
                return 3;
            }
        }
        else {
            *v = result;
            return 2;
        }
    }
    else {
        *v = result;
        return 1;
    }
fail:
    IR_ABORT(msg, sizeof(msg) - 1);
    // just return max offset to make follow-up decode fail
    return 5;
}

int32_t
decode_uleb128_value(uint8_t *buffer, uint32_t offset, int32_t len)
{
    int32_t v = 0;
    decode_uleb128(&v, buffer, offset, len);
    return v;
}

/// Utility function to encode a ULEB128 value to a buffer.
/// Returns the length in bytes of the encoded value.
int32_t
encode_uleb128(int32_t value, uint8_t *buffer, uint32_t offset)
{
    uint8_t *orig_p = buffer + offset;
    uint8_t *p = buffer + offset;
    int32_t count = 0;
    do {
        uint8_t byte = value & 0x7f;
        value >>= 7;
        count++;
        if (value != 0 || count < 0)
            byte |= 0x80; // Mark this byte to show that more
                          // bytes will follow.
        *p++ = byte;
    } while (value != 0);

    // Pad with 0x80 and emit a null byte at the end.
    if (count < 0) {
        for (; count < 0 - 1; ++count)
            *p++ = '\x80';
        *p++ = '\x00';
    }

    return (int32_t)(p - orig_p);
}

/// Returns the value uleb128 encode/decode length.
int32_t
uleb128_value_length(uint32_t value)
{
    int32_t count = 0;
    do {
        uint8_t byte = value & 0x7f;
        value >>= 7;
        count++;
        if (value != 0 || count < 0)
            byte |= 0x80; // Mark this byte to show that more
                          // bytes will follow.
    } while (value != 0);

    return count;
}

uint8_t *
ptr_offset(uint8_t *buf, int32_t data_offset)
{
    return buf + data_offset;
}

int32_t
memcpy_offset(uint8_t *des, int32_t des_length, int32_t offset, uint8_t *src,
              int32_t src_len)
{
    assert(des != NULL);
    assert(src != NULL);
    assert(src_len + offset <= des_length);

    memcpy(des + offset, src, src_len);
    return offset + src_len;
}

size_t
__numlen(int256_t num)
{
    if (num == (int256_t) 0) {
        return 1;
    }
    int32_t flag = 0;
    int32_t len = 0;
    if (num == INT256_MIN) {
        num = num + (int256_t)1;
    }
    if (num < (int256_t)0) {
        num = -num;
        flag = 1;
    }
    for(; num >(int256_t) 0; ++len) {
        num = div256_u256(num, (int256_t) 10);
    }
    return len + flag;
}

size_t
__unumlen(uint256_t num)
{
    if (num == (uint256_t)0) {
        return 1;
    }
    int32_t len = 0;
    for(; num >(uint256_t) 0; ++len) {
        num = div256_u256(num, (uint256_t) 10);
    }
    return len;
}

#define UCHAR_MAX 255
#define ALIGN (sizeof(size_t))
#define ONES ((size_t)-1/UCHAR_MAX)
#define HIGHS (ONES * (UCHAR_MAX/2+1))
#define HASZERO(x) ((x)-ONES & ~(x) & HIGHS)

char *__stpcpy(char *__restrict d, const char *__restrict s)
{
    size_t *wd;
    const size_t *ws;

    if ((uintptr_t)s % ALIGN == (uintptr_t)d % ALIGN) {
        for (; (uintptr_t)s % ALIGN; s++, d++)
            if (!(*d=*s)) return d;
        wd=(void *)d; ws=(const void *)s;
        for (; !HASZERO(*ws); *wd++ = *ws++);
        d=(void *)wd; s=(const void *)ws;
    }
    for (; (*d=*s); s++, d++);

    return d;
}

char *__strcpy(char *__restrict dest, const char *__restrict src)
{
#if 1
    __stpcpy(dest, src);
    return dest;
#else
    const unsigned char *s = src;
	unsigned char *d = dest;
	while ((*d++ = *s++));
	return dest;
#endif
}

int32_t
__isupper(int32_t c)
{
    return (unsigned)c - 'A' < 26;
}

int32_t
__tolower(int32_t c)
{
    if (__isupper(c)) return c | 32;
    return c;
}

int32_t
__strncmp(const char *_l, const char *_r, size_t n)
{
    const unsigned char *l=(void *)_l, *r=(void *)_r;
    if (!n--) return 0;
    for (; *l && *r && n && *l == *r ; l++, r++, n--);
    return *l - *r;
}

int32_t
__isdigit(int32_t c)
{
    return (unsigned)c-'0' < 10;
}

int32_t
__isspace(int32_t c)
{
    return c == ' ' || (unsigned)c-'\t' < 5;
}

int32_t
__isalpha(int32_t c)
{
    return ((unsigned)c|32)-'a' < 26;
}

int256_t __strtoi256(const char *__restrict nptr,
                     char **__restrict endptr,
                     int base) {
    register const unsigned char *s = (const unsigned char *) nptr;
    register uint256_t acc;
    register int c;
    register uint256_t cutoff;
    register int neg = 0, any;
    uint256_t cutlim;
    /*
     * Skip white space and pick up leading +/- sign if any.
     * If base is 0, allow 0x for hex and 0 for octal, else
     * assume decimal; if base is already 16, allow 0x.
     */
    do {
        c = *s++;
    } while (__isspace(c));
    if (c == '-') {
        neg = 1;
        c = *s++;
    } else if (c == '+')
        c = *s++;
    if ((base == 0 || base == 16) &&
        c == '0' && (*s == 'x' || *s == 'X')) {
        c = s[1];
        s += 2;
        base = 16;
    }
    if (base == 0)
        base = c == '0' ? 8 : 10;
    /*
     * Compute the cutoff value between legal numbers and illegal
     * numbers.  That is the largest legal value, divided by the
     * base.  An input number that is greater than this value, if
     * followed by a legal input character, is too big.  One that
     * is equal to this value may be valid or not; the limit
     * between valid and invalid numbers is then based on the last
     * digit.  For instance, if the range for longs is
     * [-2147483648..2147483647] and the input base is 10,
     * cutoff will be set to 214748364 and cutlim to either
     * 7 (neg==0) or 8 (neg==1), meaning that if we have accumulated
     * a value > 214748364, or equal but the next digit is > 7 (or 8),
     * the number is too big, and we will return a range error.
     *
     * Set any if any `digits' consumed; make it negative to indicate
     * overflow.
     */
    cutoff = neg ? -(int256_t) INT256_MIN : INT256_MAX;

    cutoff =  div256_u256_rem((uint256_t) cutoff, (uint256_t) base, &cutlim);
    for (acc = 0, any = 0;; c = *s++) {
        if (__isdigit(c))
            c -= '0';
        else if (__isalpha(c))
            c -= __isupper(c) ? 'A' - 10 : 'a' - 10;
        else
            break;
        if (c >= base)
            break;
        if (any < 0 || acc > cutoff || (acc == cutoff && c > cutlim))
            any = -1;
        else {
            any = 1;
            acc *= (uint256_t)base;
            acc += (uint256_t)c;
        }
    }
    if (any < 0) {
        acc = neg ? INT256_MIN : INT256_MAX;
    } else if (neg)
        acc = -acc;
    if (endptr != 0)
        *endptr = (char *) (any ? (char *) s - 1 : nptr);
    return (acc);
}


uint256_t __strtou256(const char *__restrict nptr,
                     char **__restrict endptr,
                     int base)
{
    register const unsigned char *s = (const unsigned char *)nptr;
    register uint256_t acc;
    register int c;
    register uint256_t cutoff;
    register int neg = 0, any;
    uint256_t cutlim;
    /*
     * See strtol for comments as to the logic used.
     */
    do {
        c = *s++;
    } while (__isspace(c));
    if (c == '-') {
        neg = 1;
        c = *s++;
    } else if (c == '+')
        c = *s++;
    if ((base == 0 || base == 16) &&
        c == '0' && (*s == 'x' || *s == 'X')) {
        c = s[1];
        s += 2;
        base = 16;
    }
    if (base == 0)
        base = c == '0' ? 8 : 10;
    cutoff = div256_u256_rem(UINT256_MAX, (uint256_t) base, &cutlim);
    for (acc = 0, any = 0;; c = *s++) {
        if (__isdigit(c))
            c -= '0';
        else if (__isalpha(c))
            c -= __isupper(c) ? 'A' - 10 : 'a' - 10;
        else
            break;
        if (c >= base)
            break;
        if (any < 0 || acc > cutoff || (acc == cutoff && c > cutlim))
            any = -1;
        else {
            any = 1;
            acc *= (uint256_t)base;
            acc += (uint256_t)c;
        }
    }
    if (any < 0) {
        acc = UINT256_MAX;
    } else if (neg)
        acc = -acc;
    if (endptr != 0)
        *endptr = (char *) (any ? (char *)s - 1 : nptr);
    return (acc);
}

// defined in chain.h
extern void builtin_revert(int32_t err_code, struct vector* msg_str);

void builtin_co_call_or_revert(const char *contract,
                                uint32_t contract_length,
                                const char *method,
                                uint32_t method_length,
                                const char *argpack,
                                uint32_t argpack_length) {
    int32_t err_code = co_call(contract, contract_length,
                           method, method_length,
                           argpack, argpack_length);
    if (err_code != 0) {
        // get error message from hostapi
        uint32_t error_len = (uint32_t) get_call_result_length();
        char *err_msg;
        if (error_len > 0) {
            err_msg = __malloc(error_len);
            get_call_result(err_msg);
            revert(err_code, err_msg, error_len);
        } else {
            char default_err[] = "co_call Reverted";
            struct vector* err_ir_str = vector_new(sizeof(default_err) - 1, 1, default_err);
            builtin_revert(err_code, err_ir_str);
        }
    }
}



int __fls(uint128_t value ) {
   return value - (value & (value - 1));
}

uint256_t div256_128(uint256_t n, uint128_t base, uint128_t *rem)
{
    uint256_t rem_256 = n ;
	uint256_t b = (uint256_t) base;
	uint256_t res, d = 1;
	uint128_t high = (uint128_t) (n >> (uint256_t) 128);

	res = 0;
	if (high >= base) {
		high /= base;
		res = (uint256_t) high << (uint256_t) 128;
		rem_256 -= ((uint256_t)high* (uint256_t)base) << (uint256_t) 128;
	}
   
	while (b >(uint256_t) 0 && b < rem_256) {
		b = b+b;
		d = d+d;
	}

	do {
		if (rem_256  >= b) {
			rem_256 -= b;
			res += d;
		}
		b >>= 1;
		d >>= 1;
	} while (d);
    *rem = (uint128_t) rem_256;
	return res;
}


uint256_t div256_u256_rem(uint256_t dividend, uint256_t divisor, uint256_t *remainder)
{
	uint128_t high = (uint128_t) (divisor >>(uint256_t) 128);
	uint256_t quot;

	if (high == 0) {
		uint128_t rem128;
		quot = div256_128(dividend, (uint128_t) divisor, &rem128); 
		*remainder = (uint256_t) rem128;
	} else {
        uint128_t rem128;
		int n = __fls(high);
        quot = div256_128(dividend >> n, divisor >> n,&rem128);

		if (quot != 0)
			quot--;
        
		*remainder = dividend - quot * divisor;
		if (*remainder >= divisor) {
			quot++;
			*remainder -= divisor;
		}
	}

	return quot;
}

uint256_t div256_u256(uint256_t dividend, uint256_t divisor) {
    uint256_t rem;
    return div256_u256_rem(dividend, divisor, &rem);
}