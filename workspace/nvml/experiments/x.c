
typedef int wchar_t;
typedef struct {
  long long __ll;
  long double __ld;
} max_align_t;
typedef unsigned long size_t;
typedef long ptrdiff_t;
typedef unsigned long uintptr_t;
typedef long intptr_t;
typedef signed char int8_t;

typedef short int16_t;

typedef int int32_t;

typedef long int64_t;

typedef long intmax_t;

typedef unsigned char uint8_t;

typedef unsigned short uint16_t;

typedef unsigned int uint32_t;

typedef unsigned long uint64_t;
typedef unsigned long uintmax_t;

typedef int8_t int_fast8_t;
typedef int64_t int_fast64_t;

typedef int8_t int_least8_t;
typedef int16_t int_least16_t;
typedef int32_t int_least32_t;
typedef int64_t int_least64_t;

typedef uint8_t uint_fast8_t;
typedef uint64_t uint_fast64_t;

typedef uint8_t uint_least8_t;
typedef uint16_t uint_least16_t;
typedef uint32_t uint_least32_t;
typedef uint64_t uint_least64_t;
typedef int32_t int_fast16_t;
typedef int32_t int_fast32_t;
typedef uint32_t uint_fast16_t;
typedef uint32_t uint_fast32_t;

typedef struct pmemobjpool PMEMobjpool;
typedef struct pmemoid {
  uint64_t pool_uuid_lo;
  uint64_t off;
} PMEMoid;

static const PMEMoid OID_NULL = {0, 0};

PMEMobjpool *pmemobj_pool_by_ptr(const void *addr);
PMEMobjpool *pmemobj_pool_by_oid(PMEMoid oid);

extern int _pobj_cache_invalidate;
extern __thread struct _pobj_pcache {
  PMEMobjpool *pop;
  uint64_t uuid_lo;
  int invalidate;
} _pobj_cached_pool;

static inline void *pmemobj_direct_inline(PMEMoid oid) {
  if (oid.off == 0 || oid.pool_uuid_lo == 0)
    return ((void *)0);

  struct _pobj_pcache *cache = &_pobj_cached_pool;
  if (_pobj_cache_invalidate != cache->invalidate ||
      cache->uuid_lo != oid.pool_uuid_lo) {
    cache->invalidate = _pobj_cache_invalidate;

    if (!(cache->pop = pmemobj_pool_by_oid(oid))) {
      cache->uuid_lo = 0;
      return ((void *)0);
    }

    cache->uuid_lo = oid.pool_uuid_lo;
  }

  return (void *)((uintptr_t)cache->pop + oid.off);
}
PMEMoid pmemobj_oid(const void *addr);

size_t pmemobj_alloc_usable_size(PMEMoid oid);

uint64_t pmemobj_type_num(PMEMoid oid);
void *pmemobj_memcpy_persist(PMEMobjpool *pop, void *dest, const void *src,
                             size_t len);

void *pmemobj_memset_persist(PMEMobjpool *pop, void *dest, int c, size_t len);

void pmemobj_persist(PMEMobjpool *pop, const void *addr, size_t len);

void pmemobj_flush(PMEMobjpool *pop, const void *addr, size_t len);

void pmemobj_drain(PMEMobjpool *pop);
const char *pmemobj_check_version(unsigned major_required,
                                  unsigned minor_required);
void pmemobj_set_funcs(void *(*malloc_func)(size_t size),
                       void (*free_func)(void *ptr),
                       void *(*realloc_func)(void *ptr, size_t size),
                       char *(*strdup_func)(const char *s));

typedef int (*pmemobj_constr)(PMEMobjpool *pop, void *ptr, void *arg);

void _pobj_debug_notice(const char *func_name, const char *file, int line);

const char *pmemobj_errormsg(void);

int *__errno_location(void);
typedef struct __locale_struct *locale_t;

void *memcpy(void *restrict, const void *restrict, size_t);
void *memmove(void *, const void *, size_t);
void *memset(void *, int, size_t);
int memcmp(const void *, const void *, size_t);
void *memchr(const void *, int, size_t);

char *strcpy(char *restrict, const char *restrict);
char *strncpy(char *restrict, const char *restrict, size_t);

char *strcat(char *restrict, const char *restrict);
char *strncat(char *restrict, const char *restrict, size_t);

int strcmp(const char *, const char *);
int strncmp(const char *, const char *, size_t);

int strcoll(const char *, const char *);
size_t strxfrm(char *restrict, const char *restrict, size_t);

char *strchr(const char *, int);
char *strrchr(const char *, int);

size_t strcspn(const char *, const char *);
size_t strspn(const char *, const char *);
char *strpbrk(const char *, const char *);
char *strstr(const char *, const char *);
char *strtok(char *restrict, const char *restrict);

size_t strlen(const char *);

char *strerror(int);

int bcmp(const void *, const void *, size_t);
void bcopy(const void *, void *, size_t);
void bzero(void *, size_t);
char *index(const char *, int);
char *rindex(const char *, int);

int ffs(int);
int ffsl(long);
int ffsll(long long);

int strcasecmp(const char *, const char *);
int strncasecmp(const char *, const char *, size_t);

int strcasecmp_l(const char *, const char *, locale_t);
int strncasecmp_l(const char *, const char *, size_t, locale_t);

char *strtok_r(char *restrict, const char *restrict, char **restrict);
int strerror_r(int, char *, size_t);
char *stpcpy(char *restrict, const char *restrict);
char *stpncpy(char *restrict, const char *restrict, size_t);
size_t strnlen(const char *, size_t);
char *strdup(const char *);
char *strndup(const char *, size_t);
char *strsignal(int);
char *strerror_l(int, locale_t);
int strcoll_l(const char *, const char *, locale_t);
size_t strxfrm_l(char *restrict, const char *restrict, size_t, locale_t);

void *memccpy(void *restrict, const void *restrict, int, size_t);

char *strsep(char **, const char *);
size_t strlcat(char *, const char *, size_t);
size_t strlcpy(char *, const char *, size_t);

typedef unsigned long __jmp_buf[8];

typedef struct __jmp_buf_tag {
  __jmp_buf __jb;
  unsigned long __fl;
  unsigned long __ss[128 / sizeof(long)];
} jmp_buf[1];

typedef jmp_buf sigjmp_buf;
int sigsetjmp(sigjmp_buf, int);
_Noreturn void siglongjmp(sigjmp_buf, int);

int _setjmp(jmp_buf);
_Noreturn void _longjmp(jmp_buf, int);

int setjmp(jmp_buf);
_Noreturn void longjmp(jmp_buf, int);

enum pobj_tx_stage {
  TX_STAGE_NONE,
  TX_STAGE_WORK,
  TX_STAGE_ONCOMMIT,
  TX_STAGE_ONABORT,
  TX_STAGE_FINALLY,

  MAX_TX_STAGE
};

enum pobj_tx_stage pmemobj_tx_stage(void);

enum pobj_tx_param {
  TX_PARAM_NONE,
  TX_PARAM_MUTEX,
  TX_PARAM_RWLOCK,
  TX_PARAM_CB,
};
enum __attribute__((deprecated(
    "enum pobj_tx_lock is deprecated, use enum pobj_tx_param"))) pobj_tx_lock {
  TX_LOCK_NONE __attribute__((
      deprecated("enum pobj_tx_lock is deprecated, use enum pobj_tx_param"))) =
      TX_PARAM_NONE,
  TX_LOCK_MUTEX __attribute__((
      deprecated("enum pobj_tx_lock is deprecated, use enum pobj_tx_param"))) =
      TX_PARAM_MUTEX,
  TX_LOCK_RWLOCK __attribute__((
      deprecated("enum pobj_tx_lock is deprecated, use enum pobj_tx_param"))) =
      TX_PARAM_RWLOCK,
};

typedef void (*pmemobj_tx_callback)(PMEMobjpool *pop, enum pobj_tx_stage stage,
                                    void *);
int pmemobj_tx_begin(PMEMobjpool *pop, jmp_buf env, ...);

int pmemobj_tx_lock(enum pobj_tx_param type, void *lockp);
void pmemobj_tx_abort(int errnum);

void pmemobj_tx_commit(void);
int pmemobj_tx_end(void);
void pmemobj_tx_process(void);

int pmemobj_tx_errno(void);
int pmemobj_tx_add_range(PMEMoid oid, uint64_t off, size_t size);
int pmemobj_tx_add_range_direct(const void *ptr, size_t size);

int pmemobj_tx_xadd_range(PMEMoid oid, uint64_t off, size_t size,
                          uint64_t flags);

int pmemobj_tx_xadd_range_direct(const void *ptr, size_t size, uint64_t flags);
PMEMoid pmemobj_tx_alloc(size_t size, uint64_t type_num);
PMEMoid pmemobj_tx_xalloc(size_t size, uint64_t type_num, uint64_t flags);
PMEMoid pmemobj_tx_zalloc(size_t size, uint64_t type_num);
PMEMoid pmemobj_tx_realloc(PMEMoid oid, size_t size, uint64_t type_num);
PMEMoid pmemobj_tx_zrealloc(PMEMoid oid, size_t size, uint64_t type_num);
PMEMoid pmemobj_tx_strdup(const char *s, uint64_t type_num);
PMEMoid pmemobj_tx_wcsdup(const wchar_t *s, uint64_t type_num);
int pmemobj_tx_free(PMEMoid oid);
static inline pmemobj_tx_callback
_pobj_validate_cb_sig(pmemobj_tx_callback cb) {
  return cb;
}
static inline void *TX_MEMCPY(void *dest, const void *src, size_t num) {
  pmemobj_tx_add_range_direct(dest, num);
  return memcpy(dest, src, num);
}

static inline void *TX_MEMSET(void *dest, int c, size_t num) {
  pmemobj_tx_add_range_direct(dest, num);
  return memset(dest, c, num);
}

typedef __builtin_va_list va_list;

typedef __builtin_va_list __isoc_va_list;
typedef long ssize_t;
typedef long off_t;
typedef struct _IO_FILE FILE;
typedef union _G_fpos64_t {
  char __opaque[16];
  double __align;
} fpos_t;

extern FILE *const stdin;
extern FILE *const stdout;
extern FILE *const stderr;

FILE *fopen(const char *restrict, const char *restrict);
FILE *freopen(const char *restrict, const char *restrict, FILE *restrict);
int fclose(FILE *);

int remove(const char *);
int rename(const char *, const char *);

int feof(FILE *);
int ferror(FILE *);
int fflush(FILE *);
void clearerr(FILE *);

int fseek(FILE *, long, int);
long ftell(FILE *);
void rewind(FILE *);

int fgetpos(FILE *restrict, fpos_t *restrict);
int fsetpos(FILE *, const fpos_t *);

size_t fread(void *restrict, size_t, size_t, FILE *restrict);
size_t fwrite(const void *restrict, size_t, size_t, FILE *restrict);

int fgetc(FILE *);
int getc(FILE *);
int getchar(void);
int ungetc(int, FILE *);

int fputc(int, FILE *);
int putc(int, FILE *);
int putchar(int);

char *fgets(char *restrict, int, FILE *restrict);

int fputs(const char *restrict, FILE *restrict);
int puts(const char *);

int printf(const char *restrict, ...);
int fprintf(FILE *restrict, const char *restrict, ...);
int sprintf(char *restrict, const char *restrict, ...);
int snprintf(char *restrict, size_t, const char *restrict, ...);

int vprintf(const char *restrict, __isoc_va_list);
int vfprintf(FILE *restrict, const char *restrict, __isoc_va_list);
int vsprintf(char *restrict, const char *restrict, __isoc_va_list);
int vsnprintf(char *restrict, size_t, const char *restrict, __isoc_va_list);

int scanf(const char *restrict, ...);
int fscanf(FILE *restrict, const char *restrict, ...);
int sscanf(const char *restrict, const char *restrict, ...);
int vscanf(const char *restrict, __isoc_va_list);
int vfscanf(FILE *restrict, const char *restrict, __isoc_va_list);
int vsscanf(const char *restrict, const char *restrict, __isoc_va_list);

void perror(const char *);

int setvbuf(FILE *restrict, char *restrict, int, size_t);
void setbuf(FILE *restrict, char *restrict);

char *tmpnam(char *);
FILE *tmpfile(void);

FILE *fmemopen(void *restrict, size_t, const char *restrict);
FILE *open_memstream(char **, size_t *);
FILE *fdopen(int, const char *);
FILE *popen(const char *, const char *);
int pclose(FILE *);
int fileno(FILE *);
int fseeko(FILE *, off_t, int);
off_t ftello(FILE *);
int dprintf(int, const char *restrict, ...);
int vdprintf(int, const char *restrict, __isoc_va_list);
void flockfile(FILE *);
int ftrylockfile(FILE *);
void funlockfile(FILE *);
int getc_unlocked(FILE *);
int getchar_unlocked(void);
int putc_unlocked(int, FILE *);
int putchar_unlocked(int);
ssize_t getdelim(char **restrict, size_t *restrict, int, FILE *restrict);
ssize_t getline(char **restrict, size_t *restrict, FILE *restrict);
int renameat(int, const char *, int, const char *);
char *ctermid(char *);

char *tempnam(const char *, const char *);

char *cuserid(char *);
void setlinebuf(FILE *);
void setbuffer(FILE *, char *, size_t);
int fgetc_unlocked(FILE *);
int fputc_unlocked(int, FILE *);
int fflush_unlocked(FILE *);
size_t fread_unlocked(void *, size_t, size_t, FILE *);
size_t fwrite_unlocked(const void *, size_t, size_t, FILE *);
void clearerr_unlocked(FILE *);
int feof_unlocked(FILE *);
int ferror_unlocked(FILE *);
int fileno_unlocked(FILE *);
int getw(FILE *);
int putw(int, FILE *);
char *fgetln(FILE *, size_t *);
int asprintf(char **, const char *, ...);
int vasprintf(char **, const char *, __isoc_va_list);

typedef uint8_t _pobj_layout_mylayout_ref[0 + 1];
typedef uint8_t root_toid_type_num[(0) + 1];
union root_toid {
  PMEMoid oid;
  struct root *_type;
  root_toid_type_num *_type_num;
};
;
typedef uint8_t
    node_toid_type_num[((1 + 1 - (sizeof(_pobj_layout_mylayout_ref)))) + 1];
union node_toid {
  PMEMoid oid;
  struct node *_type;
  node_toid_type_num *_type_num;
};
;
typedef uint8_t
    foo_toid_type_num[((2 + 1 - (sizeof(_pobj_layout_mylayout_ref)))) + 1];
union foo_toid {
  PMEMoid oid;
  struct foo *_type;
  foo_toid_type_num *_type_num;
};
;
typedef char
    _pobj_layout_mylayout_cnt[3 + 1 - (sizeof(_pobj_layout_mylayout_ref))];
;

struct root {
  union node_toid node;
};

struct node {
  union node_toid next;
  union foo_toid foo;
};

const char *layout_name = "mylayout";
int num_of_types = (sizeof(_pobj_layout_mylayout_cnt) - 1);
