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

#ifndef QINTERNAL_H
#define QINTERNAL_H

/*
 * Macro Functions
 */
#define ASSERT(c) (c)
#define CONST_STRLEN(s) (sizeof(s) - 1)
#define IS_EMPTY_STR(s) ((*s == '\0') ? true : false)
#define ENDING_CHAR(s) (*(s + __strlen(s) - 1))

#define DYNAMIC_VSPRINTF(s, f)                                       \
    do {                                                             \
        size_t _strsize;                                             \
        for (_strsize = 1024;; _strsize *= 2) {                      \
            s = (char *)malloc(_strsize);                            \
            if (s == NULL) {                                         \
                DEBUG("DYNAMIC_VSPRINTF(): can't allocate memory."); \
                break;                                               \
            }                                                        \
            va_list _arglist;                                        \
            va_start(_arglist, f);                                   \
            int _n = vsnprintf(s, _strsize, f, _arglist);            \
            va_end(_arglist);                                        \
            if (_n >= 0 && _n < _strsize)                            \
                break;                                               \
            free(s);                                                 \
        }                                                            \
    } while (0)

/*
 * Debug Macros
 */
#ifdef BUILD_DEBUG
#define DEBUG(fmt, args...) \
    fprintf(stderr, "[DEBUG] " fmt " (%s:%d)\n", ##args, __FILE__, __LINE__);
#else
#ifdef __cplusplus
#define DEBUG(fmt, args...) static_cast<void>(0)
#else
#define DEBUG(fmt, args...) (void)(0)
#endif
#endif /* BUILD_DEBUG */

/*
 * Other internal use
 */
#define MAX_HUMANOUT (60)

#endif /* QINTERNAL_H */
