#include "/usr/local/include/php/Zend/zend.h"
#include "/usr/local/include/php/Zend/zend_API.h"
#include "/usr/local/include/php/Zend/zend_alloc.h"
#include "/usr/local/include/php/Zend/zend_alloc_sizes.h"
#include "/usr/local/include/php/Zend/zend_arena.h"
#include "/usr/local/include/php/Zend/zend_ast.h"
#include "/usr/local/include/php/Zend/zend_atomic.h"
#include "/usr/local/include/php/Zend/zend_attributes.h"
#include "/usr/local/include/php/Zend/zend_bitset.h"
#include "/usr/local/include/php/Zend/zend_build.h"
#include "/usr/local/include/php/Zend/zend_builtin_functions.h"
#include "/usr/local/include/php/Zend/zend_call_stack.h"
#include "/usr/local/include/php/Zend/zend_closures.h"
#include "/usr/local/include/php/Zend/zend_compile.h"
#include "/usr/local/include/php/Zend/zend_config.h"
#include "/usr/local/include/php/Zend/zend_constants.h"
#include "/usr/local/include/php/Zend/zend_cpuinfo.h"
#include "/usr/local/include/php/Zend/zend_dtrace.h"
#include "/usr/local/include/php/Zend/zend_enum.h"
#include "/usr/local/include/php/Zend/zend_errors.h"
#include "/usr/local/include/php/Zend/zend_exceptions.h"
#include "/usr/local/include/php/Zend/zend_execute.h"
#include "/usr/local/include/php/Zend/zend_extensions.h"
#include "/usr/local/include/php/Zend/zend_fibers.h"
#include "/usr/local/include/php/Zend/zend_float.h"
#include "/usr/local/include/php/Zend/zend_frameless_function.h"
#include "/usr/local/include/php/Zend/zend_gc.h"
#include "/usr/local/include/php/Zend/zend_gdb.h"
#include "/usr/local/include/php/Zend/zend_generators.h"
#include "/usr/local/include/php/Zend/zend_globals.h"
#include "/usr/local/include/php/Zend/zend_globals_macros.h"
#include "/usr/local/include/php/Zend/zend_hash.h"
#include "/usr/local/include/php/Zend/zend_highlight.h"
#include "/usr/local/include/php/Zend/zend_hrtime.h"
#include "/usr/local/include/php/Zend/zend_inheritance.h"
#include "/usr/local/include/php/Zend/zend_ini.h"
#include "/usr/local/include/php/Zend/zend_ini_scanner.h"
#include "/usr/local/include/php/Zend/zend_interfaces.h"
#include "/usr/local/include/php/Zend/zend_iterators.h"
#include "/usr/local/include/php/Zend/zend_language_scanner.h"
#include "/usr/local/include/php/Zend/zend_lazy_objects.h"
#include "/usr/local/include/php/Zend/zend_list.h"
#include "/usr/local/include/php/Zend/zend_llist.h"
#include "/usr/local/include/php/Zend/zend_long.h"
#include "/usr/local/include/php/Zend/zend_map_ptr.h"
#include "/usr/local/include/php/Zend/zend_max_execution_timer.h"
#include "/usr/local/include/php/Zend/zend_mmap.h"
#include "/usr/local/include/php/Zend/zend_modules.h"
#include "/usr/local/include/php/Zend/zend_multibyte.h"
#include "/usr/local/include/php/Zend/zend_multiply.h"
#include "/usr/local/include/php/Zend/zend_object_handlers.h"
#include "/usr/local/include/php/Zend/zend_objects.h"
#include "/usr/local/include/php/Zend/zend_objects_API.h"
#include "/usr/local/include/php/Zend/zend_observer.h"
#include "/usr/local/include/php/Zend/zend_operators.h"
#include "/usr/local/include/php/Zend/zend_portability.h"
#include "/usr/local/include/php/Zend/zend_property_hooks.h"
#include "/usr/local/include/php/Zend/zend_ptr_stack.h"
#include "/usr/local/include/php/Zend/zend_range_check.h"
#include "/usr/local/include/php/Zend/zend_signal.h"
#include "/usr/local/include/php/Zend/zend_simd.h"
#include "/usr/local/include/php/Zend/zend_smart_str.h"
#include "/usr/local/include/php/Zend/zend_smart_str_public.h"
#include "/usr/local/include/php/Zend/zend_smart_string.h"
#include "/usr/local/include/php/Zend/zend_smart_string_public.h"
#include "/usr/local/include/php/Zend/zend_sort.h"
#include "/usr/local/include/php/Zend/zend_stack.h"
#include "/usr/local/include/php/Zend/zend_stream.h"
#include "/usr/local/include/php/Zend/zend_string.h"
#include "/usr/local/include/php/Zend/zend_strtod.h"
#include "/usr/local/include/php/Zend/zend_strtod_int.h"
#include "/usr/local/include/php/Zend/zend_system_id.h"
#include "/usr/local/include/php/Zend/zend_type_info.h"
#include "/usr/local/include/php/Zend/zend_types.h"
#include "/usr/local/include/php/Zend/zend_variables.h"
#include "/usr/local/include/php/Zend/zend_virtual_cwd.h"
#include "/usr/local/include/php/Zend/zend_vm.h"
#include "/usr/local/include/php/Zend/zend_vm_handlers.h"
#include "/usr/local/include/php/Zend/zend_vm_opcodes.h"
#include "/usr/local/include/php/Zend/zend_weakrefs.h"
#include "/usr/local/include/php/main/php.h"
#include "/usr/local/include/php/main/php_output.h"
#include "/usr/local/include/php/ext/standard/info.h"
#include "/usr/local/include/php/ext/standard/php_var.h"

// Static wrappers

uint8_t zval_get_type__extern(const zval *pz) { return zval_get_type(pz); }
uint8_t zval_gc_type__extern(uint32_t gc_type_info) { return zval_gc_type(gc_type_info); }
uint32_t zval_gc_flags__extern(uint32_t gc_type_info) { return zval_gc_flags(gc_type_info); }
uint32_t zval_gc_info__extern(uint32_t gc_type_info) { return zval_gc_info(gc_type_info); }
uint32_t zend_gc_refcount__extern(const zend_refcounted_h *p) { return zend_gc_refcount(p); }
uint32_t zend_gc_set_refcount__extern(zend_refcounted_h *p, uint32_t rc) { return zend_gc_set_refcount(p, rc); }
uint32_t zend_gc_addref__extern(zend_refcounted_h *p) { return zend_gc_addref(p); }
void zend_gc_try_addref__extern(zend_refcounted_h *p) { zend_gc_try_addref(p); }
void zend_gc_try_delref__extern(zend_refcounted_h *p) { zend_gc_try_delref(p); }
uint32_t zend_gc_delref__extern(zend_refcounted_h *p) { return zend_gc_delref(p); }
uint32_t zend_gc_addref_ex__extern(zend_refcounted_h *p, uint32_t rc) { return zend_gc_addref_ex(p, rc); }
uint32_t zend_gc_delref_ex__extern(zend_refcounted_h *p, uint32_t rc) { return zend_gc_delref_ex(p, rc); }
uint32_t zval_refcount_p__extern(const zval *pz) { return zval_refcount_p(pz); }
uint32_t zval_set_refcount_p__extern(zval *pz, uint32_t rc) { return zval_set_refcount_p(pz, rc); }
uint32_t zval_addref_p__extern(zval *pz) { return zval_addref_p(pz); }
uint32_t zval_delref_p__extern(zval *pz) { return zval_delref_p(pz); }
bool zend_may_modify_arg_in_place__extern(const zval *arg) { return zend_may_modify_arg_in_place(arg); }
void * zend_llist_get_first__extern(zend_llist *l) { return zend_llist_get_first(l); }
void * zend_llist_get_last__extern(zend_llist *l) { return zend_llist_get_last(l); }
void * zend_llist_get_next__extern(zend_llist *l) { return zend_llist_get_next(l); }
void * zend_llist_get_prev__extern(zend_llist *l) { return zend_llist_get_prev(l); }
zend_hrtime_t zend_hrtime__extern(void) { return zend_hrtime(); }
void gc_check_possible_root__extern(zend_refcounted *ref) { gc_check_possible_root(ref); }
void gc_check_possible_root_no_ref__extern(zend_refcounted *ref) { gc_check_possible_root_no_ref(ref); }
void zend_get_gc_buffer_add_zval__extern(zend_get_gc_buffer *gc_buffer, zval *zv) { zend_get_gc_buffer_add_zval(gc_buffer, zv); }
void zend_get_gc_buffer_add_obj__extern(zend_get_gc_buffer *gc_buffer, zend_object *obj) { zend_get_gc_buffer_add_obj(gc_buffer, obj); }
void zend_get_gc_buffer_add_ht__extern(zend_get_gc_buffer *gc_buffer, HashTable *ht) { zend_get_gc_buffer_add_ht(gc_buffer, ht); }
void zend_get_gc_buffer_add_ptr__extern(zend_get_gc_buffer *gc_buffer, void *ptr) { zend_get_gc_buffer_add_ptr(gc_buffer, ptr); }
void zend_get_gc_buffer_use__extern(zend_get_gc_buffer *gc_buffer, zval **table, int *n) { zend_get_gc_buffer_use(gc_buffer, table, n); }
zend_ulong zend_string_hash_val__extern(zend_string *s) { return zend_string_hash_val(s); }
void zend_string_forget_hash_val__extern(zend_string *s) { zend_string_forget_hash_val(s); }
uint32_t zend_string_refcount__extern(const zend_string *s) { return zend_string_refcount(s); }
uint32_t zend_string_addref__extern(zend_string *s) { return zend_string_addref(s); }
uint32_t zend_string_delref__extern(zend_string *s) { return zend_string_delref(s); }
zend_string * zend_string_alloc__extern(size_t len, bool persistent) { return zend_string_alloc(len, persistent); }
zend_string * zend_string_safe_alloc__extern(size_t n, size_t m, size_t l, bool persistent) { return zend_string_safe_alloc(n, m, l, persistent); }
zend_string * zend_string_init__extern(const char *str, size_t len, bool persistent) { return zend_string_init(str, len, persistent); }
zend_string * zend_string_init_fast__extern(const char *str, size_t len) { return zend_string_init_fast(str, len); }
zend_string * zend_string_copy__extern(zend_string *s) { return zend_string_copy(s); }
zend_string * zend_string_dup__extern(zend_string *s, bool persistent) { return zend_string_dup(s, persistent); }
zend_string * zend_string_separate__extern(zend_string *s, bool persistent) { return zend_string_separate(s, persistent); }
zend_string * zend_string_realloc__extern(zend_string *s, size_t len, bool persistent) { return zend_string_realloc(s, len, persistent); }
zend_string * zend_string_extend__extern(zend_string *s, size_t len, bool persistent) { return zend_string_extend(s, len, persistent); }
zend_string * zend_string_truncate__extern(zend_string *s, size_t len, bool persistent) { return zend_string_truncate(s, len, persistent); }
zend_string * zend_string_safe_realloc__extern(zend_string *s, size_t n, size_t m, size_t l, bool persistent) { return zend_string_safe_realloc(s, n, m, l, persistent); }
void zend_string_free__extern(zend_string *s) { zend_string_free(s); }
void zend_string_efree__extern(zend_string *s) { zend_string_efree(s); }
void zend_string_release__extern(zend_string *s) { zend_string_release(s); }
void zend_string_release_ex__extern(zend_string *s, bool persistent) { zend_string_release_ex(s, persistent); }
bool zend_string_equals_cstr__extern(const zend_string *s1, const char *s2, size_t s2_length) { return zend_string_equals_cstr(s1, s2, s2_length); }
bool zend_string_equal_content__extern(const zend_string *s1, const zend_string *s2) { return zend_string_equal_content(s1, s2); }
bool zend_string_equals__extern(const zend_string *s1, const zend_string *s2) { return zend_string_equals(s1, s2); }
bool zend_string_starts_with_cstr__extern(const zend_string *str, const char *prefix, size_t prefix_length) { return zend_string_starts_with_cstr(str, prefix, prefix_length); }
bool zend_string_starts_with__extern(const zend_string *str, const zend_string *prefix) { return zend_string_starts_with(str, prefix); }
bool zend_string_starts_with_cstr_ci__extern(const zend_string *str, const char *prefix, size_t prefix_length) { return zend_string_starts_with_cstr_ci(str, prefix, prefix_length); }
bool zend_string_starts_with_ci__extern(const zend_string *str, const zend_string *prefix) { return zend_string_starts_with_ci(str, prefix); }
zend_ulong zend_inline_hash_func__extern(const char *str, size_t len) { return zend_inline_hash_func(str, len); }
zval * zend_hash_find_ex__extern(const HashTable *ht, zend_string *key, bool known_hash) { return zend_hash_find_ex(ht, key, known_hash); }
bool zend_hash_exists__extern(const HashTable *ht, zend_string *key) { return zend_hash_exists(ht, key); }
bool zend_hash_str_exists__extern(const HashTable *ht, const char *str, size_t len) { return zend_hash_str_exists(ht, str, len); }
bool zend_hash_index_exists__extern(const HashTable *ht, zend_ulong h) { return zend_hash_index_exists(ht, h); }
zend_result zend_hash_has_more_elements_ex__extern(const HashTable *ht, const HashPosition *pos) { return zend_hash_has_more_elements_ex(ht, pos); }
zend_result zend_hash_has_more_elements__extern(const HashTable *ht) { return zend_hash_has_more_elements(ht); }
zend_result zend_hash_move_forward__extern(HashTable *ht) { return zend_hash_move_forward(ht); }
zend_result zend_hash_move_backwards__extern(HashTable *ht) { return zend_hash_move_backwards(ht); }
zend_hash_key_type zend_hash_get_current_key__extern(const HashTable *ht, zend_string **str_index, zend_ulong *num_index) { return zend_hash_get_current_key(ht, str_index, num_index); }
void zend_hash_get_current_key_zval__extern(const HashTable *ht, zval *key) { zend_hash_get_current_key_zval(ht, key); }
zend_hash_key_type zend_hash_get_current_key_type__extern(const HashTable *ht) { return zend_hash_get_current_key_type(ht); }
zval * zend_hash_get_current_data__extern(const HashTable *ht) { return zend_hash_get_current_data(ht); }
void zend_hash_internal_pointer_reset__extern(HashTable *ht) { zend_hash_internal_pointer_reset(ht); }
void zend_hash_internal_pointer_end__extern(HashTable *ht) { zend_hash_internal_pointer_end(ht); }
void zend_hash_sort__extern(HashTable *ht, bucket_compare_func_t compare_func, bool renumber) { zend_hash_sort(ht, compare_func, renumber); }
void zend_array_sort__extern(HashTable *ht, bucket_compare_func_t compare_func, bool renumber) { zend_array_sort(ht, compare_func, renumber); }
uint32_t zend_hash_num_elements__extern(const HashTable *ht) { return zend_hash_num_elements(ht); }
zend_long zend_hash_next_free_element__extern(const HashTable *ht) { return zend_hash_next_free_element(ht); }
void zend_hash_iterators_update__extern(const HashTable *ht, HashPosition from, HashPosition to) { zend_hash_iterators_update(ht, from, to); }
void zend_array_release__extern(zend_array *array) { zend_array_release(array); }
void zend_hash_release__extern(zend_array *array) { zend_hash_release(array); }
bool _zend_handle_numeric_str__extern(const char *key, size_t length, zend_ulong *idx) { return _zend_handle_numeric_str(key, length, idx); }
zval * zend_hash_find_ind__extern(const HashTable *ht, zend_string *key) { return zend_hash_find_ind(ht, key); }
zval * zend_hash_find_ex_ind__extern(const HashTable *ht, zend_string *key, bool known_hash) { return zend_hash_find_ex_ind(ht, key, known_hash); }
bool zend_hash_exists_ind__extern(const HashTable *ht, zend_string *key) { return zend_hash_exists_ind(ht, key); }
zval * zend_hash_str_find_ind__extern(const HashTable *ht, const char *str, size_t len) { return zend_hash_str_find_ind(ht, str, len); }
bool zend_hash_str_exists_ind__extern(const HashTable *ht, const char *str, size_t len) { return zend_hash_str_exists_ind(ht, str, len); }
zval * zend_symtable_add_new__extern(HashTable *ht, zend_string *key, zval *pData) { return zend_symtable_add_new(ht, key, pData); }
zval * zend_symtable_update__extern(HashTable *ht, zend_string *key, zval *pData) { return zend_symtable_update(ht, key, pData); }
zval * zend_symtable_update_ind__extern(HashTable *ht, zend_string *key, zval *pData) { return zend_symtable_update_ind(ht, key, pData); }
zend_result zend_symtable_del__extern(HashTable *ht, zend_string *key) { return zend_symtable_del(ht, key); }
zend_result zend_symtable_del_ind__extern(HashTable *ht, zend_string *key) { return zend_symtable_del_ind(ht, key); }
zval * zend_symtable_find__extern(const HashTable *ht, zend_string *key) { return zend_symtable_find(ht, key); }
zval * zend_symtable_find_ind__extern(const HashTable *ht, zend_string *key) { return zend_symtable_find_ind(ht, key); }
bool zend_symtable_exists__extern(const HashTable *ht, zend_string *key) { return zend_symtable_exists(ht, key); }
bool zend_symtable_exists_ind__extern(const HashTable *ht, zend_string *key) { return zend_symtable_exists_ind(ht, key); }
zval * zend_symtable_str_update__extern(HashTable *ht, const char *str, size_t len, zval *pData) { return zend_symtable_str_update(ht, str, len, pData); }
zval * zend_symtable_str_update_ind__extern(HashTable *ht, const char *str, size_t len, zval *pData) { return zend_symtable_str_update_ind(ht, str, len, pData); }
zend_result zend_symtable_str_del__extern(HashTable *ht, const char *str, size_t len) { return zend_symtable_str_del(ht, str, len); }
zend_result zend_symtable_str_del_ind__extern(HashTable *ht, const char *str, size_t len) { return zend_symtable_str_del_ind(ht, str, len); }
zval * zend_symtable_str_find__extern(HashTable *ht, const char *str, size_t len) { return zend_symtable_str_find(ht, str, len); }
bool zend_symtable_str_exists__extern(HashTable *ht, const char *str, size_t len) { return zend_symtable_str_exists(ht, str, len); }
void * zend_hash_add_ptr__extern(HashTable *ht, zend_string *key, void *pData) { return zend_hash_add_ptr(ht, key, pData); }
void * zend_hash_add_new_ptr__extern(HashTable *ht, zend_string *key, void *pData) { return zend_hash_add_new_ptr(ht, key, pData); }
void * zend_hash_str_add_ptr__extern(HashTable *ht, const char *str, size_t len, void *pData) { return zend_hash_str_add_ptr(ht, str, len, pData); }
void * zend_hash_str_add_new_ptr__extern(HashTable *ht, const char *str, size_t len, void *pData) { return zend_hash_str_add_new_ptr(ht, str, len, pData); }
void * zend_hash_update_ptr__extern(HashTable *ht, zend_string *key, void *pData) { return zend_hash_update_ptr(ht, key, pData); }
void * zend_hash_str_update_ptr__extern(HashTable *ht, const char *str, size_t len, void *pData) { return zend_hash_str_update_ptr(ht, str, len, pData); }
void * zend_hash_add_mem__extern(HashTable *ht, zend_string *key, void *pData, size_t size) { return zend_hash_add_mem(ht, key, pData, size); }
void * zend_hash_add_new_mem__extern(HashTable *ht, zend_string *key, void *pData, size_t size) { return zend_hash_add_new_mem(ht, key, pData, size); }
void * zend_hash_str_add_mem__extern(HashTable *ht, const char *str, size_t len, void *pData, size_t size) { return zend_hash_str_add_mem(ht, str, len, pData, size); }
void * zend_hash_str_add_new_mem__extern(HashTable *ht, const char *str, size_t len, void *pData, size_t size) { return zend_hash_str_add_new_mem(ht, str, len, pData, size); }
void * zend_hash_update_mem__extern(HashTable *ht, zend_string *key, void *pData, size_t size) { return zend_hash_update_mem(ht, key, pData, size); }
void * zend_hash_str_update_mem__extern(HashTable *ht, const char *str, size_t len, void *pData, size_t size) { return zend_hash_str_update_mem(ht, str, len, pData, size); }
void * zend_hash_index_add_ptr__extern(HashTable *ht, zend_ulong h, void *pData) { return zend_hash_index_add_ptr(ht, h, pData); }
void * zend_hash_index_add_new_ptr__extern(HashTable *ht, zend_ulong h, void *pData) { return zend_hash_index_add_new_ptr(ht, h, pData); }
void * zend_hash_index_update_ptr__extern(HashTable *ht, zend_ulong h, void *pData) { return zend_hash_index_update_ptr(ht, h, pData); }
void * zend_hash_index_add_mem__extern(HashTable *ht, zend_ulong h, void *pData, size_t size) { return zend_hash_index_add_mem(ht, h, pData, size); }
void * zend_hash_next_index_insert_ptr__extern(HashTable *ht, void *pData) { return zend_hash_next_index_insert_ptr(ht, pData); }
void * zend_hash_index_update_mem__extern(HashTable *ht, zend_ulong h, void *pData, size_t size) { return zend_hash_index_update_mem(ht, h, pData, size); }
void * zend_hash_next_index_insert_mem__extern(HashTable *ht, void *pData, size_t size) { return zend_hash_next_index_insert_mem(ht, pData, size); }
void * zend_hash_find_ptr__extern(const HashTable *ht, zend_string *key) { return zend_hash_find_ptr(ht, key); }
void * zend_hash_find_ex_ptr__extern(const HashTable *ht, zend_string *key, bool known_hash) { return zend_hash_find_ex_ptr(ht, key, known_hash); }
void * zend_hash_str_find_ptr__extern(const HashTable *ht, const char *str, size_t len) { return zend_hash_str_find_ptr(ht, str, len); }
void * zend_hash_index_find_ptr__extern(const HashTable *ht, zend_ulong h) { return zend_hash_index_find_ptr(ht, h); }
zval * zend_hash_index_find_deref__extern(const HashTable *ht, zend_ulong h) { return zend_hash_index_find_deref(ht, h); }
zval * zend_hash_find_deref__extern(const HashTable *ht, zend_string *str) { return zend_hash_find_deref(ht, str); }
zval * zend_hash_str_find_deref__extern(const HashTable *ht, const char *str, size_t len) { return zend_hash_str_find_deref(ht, str, len); }
void * zend_symtable_str_find_ptr__extern(const HashTable *ht, const char *str, size_t len) { return zend_symtable_str_find_ptr(ht, str, len); }
void * zend_hash_get_current_data_ptr_ex__extern(const HashTable *ht, const HashPosition *pos) { return zend_hash_get_current_data_ptr_ex(ht, pos); }
bool zend_array_is_list__extern(const zend_array *array) { return zend_array_is_list(array); }
zval * _zend_hash_append_ex__extern(HashTable *ht, zend_string *key, zval *zv, bool key_guaranteed_interned) { return _zend_hash_append_ex(ht, key, zv, key_guaranteed_interned); }
zval * _zend_hash_append__extern(HashTable *ht, zend_string *key, zval *zv) { return _zend_hash_append(ht, key, zv); }
zval * _zend_hash_append_ptr_ex__extern(HashTable *ht, zend_string *key, void *ptr, bool key_guaranteed_interned) { return _zend_hash_append_ptr_ex(ht, key, ptr, key_guaranteed_interned); }
zval * _zend_hash_append_ptr__extern(HashTable *ht, zend_string *key, void *ptr) { return _zend_hash_append_ptr(ht, key, ptr); }
void _zend_hash_append_ind__extern(HashTable *ht, zend_string *key, zval *ptr) { _zend_hash_append_ind(ht, key, ptr); }
zend_ast * zend_ast_create_ex_0__extern(zend_ast_kind kind, zend_ast_attr attr) { return zend_ast_create_ex_0(kind, attr); }
zend_ast * zend_ast_create_ex_1__extern(zend_ast_kind kind, zend_ast_attr attr, zend_ast *child) { return zend_ast_create_ex_1(kind, attr, child); }
zend_ast * zend_ast_create_ex_2__extern(zend_ast_kind kind, zend_ast_attr attr, zend_ast *child1, zend_ast *child2) { return zend_ast_create_ex_2(kind, attr, child1, child2); }
zend_ast * zend_ast_create_ex_3__extern(zend_ast_kind kind, zend_ast_attr attr, zend_ast *child1, zend_ast *child2, zend_ast *child3) { return zend_ast_create_ex_3(kind, attr, child1, child2, child3); }
zend_ast * zend_ast_create_ex_4__extern(zend_ast_kind kind, zend_ast_attr attr, zend_ast *child1, zend_ast *child2, zend_ast *child3, zend_ast *child4) { return zend_ast_create_ex_4(kind, attr, child1, child2, child3, child4); }
zend_ast * zend_ast_create_ex_5__extern(zend_ast_kind kind, zend_ast_attr attr, zend_ast *child1, zend_ast *child2, zend_ast *child3, zend_ast *child4, zend_ast *child5) { return zend_ast_create_ex_5(kind, attr, child1, child2, child3, child4, child5); }
size_t zend_ast_size__extern(uint32_t children) { return zend_ast_size(children); }
bool zend_ast_is_special__extern(const zend_ast *ast) { return zend_ast_is_special(ast); }
bool zend_ast_is_decl__extern(const zend_ast *ast) { return zend_ast_is_decl(ast); }
bool zend_ast_is_list__extern(const zend_ast *ast) { return zend_ast_is_list(ast); }
zend_ast_list * zend_ast_get_list__extern(zend_ast *ast) { return zend_ast_get_list(ast); }
zval * zend_ast_get_zval__extern(zend_ast *ast) { return zend_ast_get_zval(ast); }
zend_string * zend_ast_get_str__extern(zend_ast *ast) { return zend_ast_get_str(ast); }
zend_ast_op_array * zend_ast_get_op_array__extern(zend_ast *ast) { return zend_ast_get_op_array(ast); }
zend_string * zend_ast_get_constant_name__extern(zend_ast *ast) { return zend_ast_get_constant_name(ast); }
uint32_t zend_ast_get_num_children__extern(const zend_ast *ast) { return zend_ast_get_num_children(ast); }
uint32_t zend_ast_get_lineno__extern(zend_ast *ast) { return zend_ast_get_lineno(ast); }
zend_ast * zend_ast_create_binary_op__extern(uint32_t opcode, zend_ast *op0, zend_ast *op1) { return zend_ast_create_binary_op(opcode, op0, op1); }
zend_ast * zend_ast_create_assign_op__extern(uint32_t opcode, zend_ast *op0, zend_ast *op1) { return zend_ast_create_assign_op(opcode, op0, op1); }
zend_ast * zend_ast_create_cast__extern(uint32_t type, zend_ast *op0) { return zend_ast_create_cast(type, op0); }
zend_ast * zend_ast_list_rtrim__extern(zend_ast *ast) { return zend_ast_list_rtrim(ast); }
void zval_ptr_dtor_nogc__extern(zval *zval_ptr) { zval_ptr_dtor_nogc(zval_ptr); }
void i_zval_ptr_dtor__extern(zval *zval_ptr) { i_zval_ptr_dtor(zval_ptr); }
void zval_copy_ctor__extern(zval *zvalue) { zval_copy_ctor(zvalue); }
void zval_opt_copy_ctor__extern(zval *zvalue) { zval_opt_copy_ctor(zvalue); }
void zval_ptr_dtor_str__extern(zval *zval_ptr) { zval_ptr_dtor_str(zval_ptr); }
size_t zend_print_variable__extern(zval *var) { return zend_print_variable(var); }
bool zend_object_is_lazy__extern(const zend_object *obj) { return zend_object_is_lazy(obj); }
bool zend_object_is_lazy_proxy__extern(const zend_object *obj) { return zend_object_is_lazy_proxy(obj); }
bool zend_lazy_object_initialized__extern(const zend_object *obj) { return zend_lazy_object_initialized(obj); }
bool zend_lazy_object_must_init__extern(const zend_object *obj) { return zend_lazy_object_must_init(obj); }
bool zend_lazy_object_initialize_on_serialize__extern(const zend_object *obj) { return zend_lazy_object_initialize_on_serialize(obj); }
HashTable * zend_std_get_properties_ex__extern(zend_object *object) { return zend_std_get_properties_ex(object); }
size_t zend_safe_address__extern(size_t nmemb, size_t size, size_t offset, bool *overflow) { return zend_safe_address(nmemb, size, offset, overflow); }
size_t zend_safe_address_guarded__extern(size_t nmemb, size_t size, size_t offset) { return zend_safe_address_guarded(nmemb, size, offset); }
size_t zend_safe_addmult__extern(size_t nmemb, size_t size, size_t offset, const char *message) { return zend_safe_addmult(nmemb, size, offset, message); }
bool instanceof_function__extern(const zend_class_entry *instance_ce, const zend_class_entry *ce) { return instanceof_function(instance_ce, ce); }
zend_long zend_dval_to_lval__extern(double d) { return zend_dval_to_lval(d); }
zend_long zend_dval_to_lval_silent__extern(double d) { return zend_dval_to_lval_silent(d); }
zend_long zend_dval_to_lval_cap__extern(double d, const zend_string *s) { return zend_dval_to_lval_cap(d, s); }
bool zend_is_long_compatible__extern(double d, zend_long l) { return zend_is_long_compatible(d, l); }
zend_long zend_dval_to_lval_safe__extern(double d) { return zend_dval_to_lval_safe(d); }
uint8_t is_numeric_string_ex__extern(const char *str, size_t length, zend_long *lval, double *dval, bool allow_errors, int *oflow_info, bool *trailing_data) { return is_numeric_string_ex(str, length, lval, dval, allow_errors, oflow_info, trailing_data); }
uint8_t is_numeric_string__extern(const char *str, size_t length, zend_long *lval, double *dval, bool allow_errors) { return is_numeric_string(str, length, lval, dval, allow_errors); }
const char * zend_memnstr__extern(const char *haystack, const char *needle, size_t needle_len, const char *end) { return zend_memnstr(haystack, needle, needle_len, end); }
const void * zend_memrchr__extern(const void *s, int c, size_t n) { return zend_memrchr(s, c, n); }
const char * zend_memnrstr__extern(const char *haystack, const char *needle, size_t needle_len, const char *end) { return zend_memnrstr(haystack, needle, needle_len, end); }
size_t zend_strnlen__extern(const char *s, size_t maxlen) { return zend_strnlen(s, maxlen); }
void * zend_mempcpy__extern(void *dest, const void *src, size_t n) { return zend_mempcpy(dest, src, n); }
zend_long zval_get_long__extern(const zval *op) { return zval_get_long(op); }
zend_long zval_get_long_ex__extern(const zval *op, bool is_strict) { return zval_get_long_ex(op, is_strict); }
double zval_get_double__extern(const zval *op) { return zval_get_double(op); }
zend_string * zval_get_string__extern(zval *op) { return zval_get_string(op); }
zend_string * zval_get_tmp_string__extern(zval *op, zend_string **tmp) { return zval_get_tmp_string(op, tmp); }
void zend_tmp_string_release__extern(zend_string *tmp) { zend_tmp_string_release(tmp); }
zend_string * zval_try_get_string__extern(zval *op) { return zval_try_get_string(op); }
zend_string * zval_try_get_tmp_string__extern(zval *op, zend_string **tmp) { return zval_try_get_tmp_string(op, tmp); }
bool try_convert_to_string__extern(zval *op) { return try_convert_to_string(op); }
bool i_zend_is_true__extern(const zval *op) { return i_zend_is_true(op); }
zend_string * zend_string_tolower__extern(zend_string *str) { return zend_string_tolower(str); }
zend_string * zend_string_toupper__extern(zend_string *str) { return zend_string_toupper(str); }
void fast_long_increment_function__extern(zval *op1) { fast_long_increment_function(op1); }
void fast_long_decrement_function__extern(zval *op1) { fast_long_decrement_function(op1); }
void fast_long_add_function__extern(zval *result, zval *op1, zval *op2) { fast_long_add_function(result, op1, op2); }
void fast_long_sub_function__extern(zval *result, zval *op1, zval *op2) { fast_long_sub_function(result, op1, op2); }
bool zend_fast_equal_strings__extern(zend_string *s1, zend_string *s2) { return zend_fast_equal_strings(s1, s2); }
bool fast_equal_check_function__extern(zval *op1, zval *op2) { return fast_equal_check_function(op1, op2); }
bool fast_equal_check_long__extern(zval *op1, zval *op2) { return fast_equal_check_long(op1, op2); }
bool fast_equal_check_string__extern(zval *op1, zval *op2) { return fast_equal_check_string(op1, op2); }
bool fast_is_identical_function__extern(zval *op1, zval *op2) { return fast_is_identical_function(op1, op2); }
bool fast_is_not_identical_function__extern(zval *op1, zval *op2) { return fast_is_not_identical_function(op1, op2); }
char * zend_print_ulong_to_buf__extern(char *buf, zend_ulong num) { return zend_print_ulong_to_buf(buf, num); }
char * zend_print_long_to_buf__extern(char *buf, zend_long num) { return zend_print_long_to_buf(buf, num); }
void zend_unwrap_reference__extern(zval *op) { zend_unwrap_reference(op); }
bool zend_strnieq__extern(const char *ptr1, const char *ptr2, size_t num) { return zend_strnieq(ptr1, ptr2, num); }
const char * zend_memnistr__extern(const char *haystack, const char *needle, size_t needle_len, const char *end) { return zend_memnistr(haystack, needle, needle_len, end); }
znode * zend_ast_get_znode__extern(zend_ast *ast) { return zend_ast_get_znode(ast); }
uint32_t zend_visibility_to_set_visibility__extern(uint32_t visibility) { return zend_visibility_to_set_visibility(visibility); }
bool zend_atomic_bool_exchange_ex__extern(zend_atomic_bool *obj, bool desired) { return zend_atomic_bool_exchange_ex(obj, desired); }
int zend_atomic_int_exchange_ex__extern(zend_atomic_int *obj, int desired) { return zend_atomic_int_exchange_ex(obj, desired); }
bool zend_atomic_bool_compare_exchange_ex__extern(zend_atomic_bool *obj, bool *expected, bool desired) { return zend_atomic_bool_compare_exchange_ex(obj, expected, desired); }
bool zend_atomic_int_compare_exchange_ex__extern(zend_atomic_int *obj, int *expected, int desired) { return zend_atomic_int_compare_exchange_ex(obj, expected, desired); }
bool zend_atomic_bool_load_ex__extern(const zend_atomic_bool *obj) { return zend_atomic_bool_load_ex(obj); }
int zend_atomic_int_load_ex__extern(const zend_atomic_int *obj) { return zend_atomic_int_load_ex(obj); }
void zend_atomic_bool_store_ex__extern(zend_atomic_bool *obj, bool desired) { zend_atomic_bool_store_ex(obj, desired); }
void zend_atomic_int_store_ex__extern(zend_atomic_int *obj, int desired) { zend_atomic_int_store_ex(obj, desired); }
void zend_ptr_stack_3_push__extern(zend_ptr_stack *stack, void *a, void *b, void *c) { zend_ptr_stack_3_push(stack, a, b, c); }
void zend_ptr_stack_2_push__extern(zend_ptr_stack *stack, void *a, void *b) { zend_ptr_stack_2_push(stack, a, b); }
void zend_ptr_stack_3_pop__extern(zend_ptr_stack *stack, void **a, void **b, void **c) { zend_ptr_stack_3_pop(stack, a, b, c); }
void zend_ptr_stack_2_pop__extern(zend_ptr_stack *stack, void **a, void **b) { zend_ptr_stack_2_pop(stack, a, b); }
void zend_ptr_stack_push__extern(zend_ptr_stack *stack, void *ptr) { zend_ptr_stack_push(stack, ptr); }
void * zend_ptr_stack_pop__extern(zend_ptr_stack *stack) { return zend_ptr_stack_pop(stack); }
void * zend_ptr_stack_top__extern(zend_ptr_stack *stack) { return zend_ptr_stack_top(stack); }
void zend_object_store_ctor_failed__extern(zend_object *obj) { zend_object_store_ctor_failed(obj); }
void zend_object_release__extern(zend_object *obj) { zend_object_release(obj); }
size_t zend_object_properties_size__extern(zend_class_entry *ce) { return zend_object_properties_size(ce); }
void * zend_object_alloc__extern(size_t obj_size, zend_class_entry *ce) { return zend_object_alloc(obj_size, ce); }
zend_property_info * zend_get_property_info_for_slot_self__extern(zend_object *obj, zval *slot) { return zend_get_property_info_for_slot_self(obj, slot); }
zend_property_info * zend_get_property_info_for_slot__extern(zend_object *obj, zval *slot) { return zend_get_property_info_for_slot(obj, slot); }
zend_property_info * zend_get_typed_property_info_for_slot__extern(zend_object *obj, zval *slot) { return zend_get_typed_property_info_for_slot(obj, slot); }
bool zend_check_method_accessible__extern(const zend_function *fn, const zend_class_entry *scope) { return zend_check_method_accessible(fn, scope); }
zend_arena * zend_arena_create__extern(size_t size) { return zend_arena_create(size); }
void zend_arena_destroy__extern(zend_arena *arena) { zend_arena_destroy(arena); }
void * zend_arena_alloc__extern(zend_arena **arena_ptr, size_t size) { return zend_arena_alloc(arena_ptr, size); }
void * zend_arena_calloc__extern(zend_arena **arena_ptr, size_t count, size_t unit_size) { return zend_arena_calloc(arena_ptr, count, unit_size); }
void * zend_arena_checkpoint__extern(zend_arena *arena) { return zend_arena_checkpoint(arena); }
void zend_arena_release__extern(zend_arena **arena_ptr, void *checkpoint) { zend_arena_release(arena_ptr, checkpoint); }
bool zend_arena_contains__extern(zend_arena *arena, void *ptr) { return zend_arena_contains(arena, ptr); }
void * zend_call_stack_position__extern(void) { return zend_call_stack_position(); }
bool zend_call_stack_overflowed__extern(void *stack_limit) { return zend_call_stack_overflowed(stack_limit); }
void * zend_call_stack_limit__extern(void *base, size_t size, size_t reserved_size) { return zend_call_stack_limit(base, size, reserved_size); }
size_t zend_call_stack_default_size__extern(void) { return zend_call_stack_default_size(); }
const char * zend_get_unmangled_property_name__extern(const zend_string *mangled_prop) { return zend_get_unmangled_property_name(mangled_prop); }
bool zend_check_arg_send_type__extern(const zend_function *zf, uint32_t arg_num, uint32_t mask) { return zend_check_arg_send_type(zf, arg_num, mask); }
zend_constant * zend_get_special_const__extern(const char *name, size_t name_len) { return zend_get_special_const(name, name_len); }
void zend_copy_to_variable__extern(zval *variable_ptr, zval *value, uint8_t value_type) { zend_copy_to_variable(variable_ptr, value, value_type); }
zval * zend_assign_to_variable__extern(zval *variable_ptr, zval *value, uint8_t value_type, bool strict) { return zend_assign_to_variable(variable_ptr, value, value_type, strict); }
zval * zend_assign_to_variable_ex__extern(zval *variable_ptr, zval *value, zend_uchar value_type, bool strict, zend_refcounted **garbage_ptr) { return zend_assign_to_variable_ex(variable_ptr, value, value_type, strict, garbage_ptr); }
void zend_safe_assign_to_variable_noref__extern(zval *variable_ptr, zval *value) { zend_safe_assign_to_variable_noref(variable_ptr, value); }
void zend_cast_zval_to_object__extern(zval *result, zval *expr, uint8_t op1_type) { zend_cast_zval_to_object(result, expr, op1_type); }
void zend_cast_zval_to_array__extern(zval *result, zval *expr, uint8_t op1_type) { zend_cast_zval_to_array(result, expr, op1_type); }
zend_vm_stack zend_vm_stack_new_page__extern(size_t size, zend_vm_stack prev) { return zend_vm_stack_new_page(size, prev); }
void zend_vm_init_call_frame__extern(zend_execute_data *call, uint32_t call_info, zend_function *func, uint32_t num_args, void *object_or_called_scope) { zend_vm_init_call_frame(call, call_info, func, num_args, object_or_called_scope); }
zend_execute_data * zend_vm_stack_push_call_frame_ex__extern(uint32_t used_stack, uint32_t call_info, zend_function *func, uint32_t num_args, void *object_or_called_scope) { return zend_vm_stack_push_call_frame_ex(used_stack, call_info, func, num_args, object_or_called_scope); }
uint32_t zend_vm_calc_used_stack__extern(uint32_t num_args, zend_function *func) { return zend_vm_calc_used_stack(num_args, func); }
zend_execute_data * zend_vm_stack_push_call_frame__extern(uint32_t call_info, zend_function *func, uint32_t num_args, void *object_or_called_scope) { return zend_vm_stack_push_call_frame(call_info, func, num_args, object_or_called_scope); }
void zend_vm_stack_free_extra_args_ex__extern(uint32_t call_info, zend_execute_data *call) { zend_vm_stack_free_extra_args_ex(call_info, call); }
void zend_vm_stack_free_extra_args__extern(zend_execute_data *call) { zend_vm_stack_free_extra_args(call); }
void zend_vm_stack_free_args__extern(zend_execute_data *call) { zend_vm_stack_free_args(call); }
void zend_vm_stack_free_call_frame_ex__extern(uint32_t call_info, zend_execute_data *call) { zend_vm_stack_free_call_frame_ex(call_info, call); }
void zend_vm_stack_free_call_frame__extern(zend_execute_data *call) { zend_vm_stack_free_call_frame(call); }
void zend_vm_stack_extend_call_frame__extern(zend_execute_data **call, uint32_t passed_args, uint32_t additional_args) { zend_vm_stack_extend_call_frame(call, passed_args, additional_args); }
zend_function * zend_active_function__extern(void) { return zend_active_function(); }
void * zend_get_bad_ptr__extern(void) { return zend_get_bad_ptr(); }
zend_result zend_register_class_alias__extern(const char *name, zend_class_entry *ce) { return zend_register_class_alias(name, ce); }
HashTable * zend_class_constants_table__extern(zend_class_entry *ce) { return zend_class_constants_table(ce); }
zval * zend_class_default_properties_table__extern(zend_class_entry *ce) { return zend_class_default_properties_table(ce); }
void zend_class_set_backed_enum_table__extern(zend_class_entry *ce, HashTable *backed_enum_table) { zend_class_set_backed_enum_table(ce, backed_enum_table); }
HashTable * zend_class_backed_enum_table__extern(zend_class_entry *ce) { return zend_class_backed_enum_table(ce); }
void add_assoc_long__extern(zval *arg, const char *key, zend_long n) { add_assoc_long(arg, key, n); }
void add_assoc_null__extern(zval *arg, const char *key) { add_assoc_null(arg, key); }
void add_assoc_bool__extern(zval *arg, const char *key, bool b) { add_assoc_bool(arg, key, b); }
void add_assoc_resource__extern(zval *arg, const char *key, zend_resource *r) { add_assoc_resource(arg, key, r); }
void add_assoc_double__extern(zval *arg, const char *key, double d) { add_assoc_double(arg, key, d); }
void add_assoc_str__extern(zval *arg, const char *key, zend_string *str) { add_assoc_str(arg, key, str); }
void add_assoc_string__extern(zval *arg, const char *key, const char *str) { add_assoc_string(arg, key, str); }
void add_assoc_stringl__extern(zval *arg, const char *key, const char *str, size_t length) { add_assoc_stringl(arg, key, str, length); }
void add_assoc_array__extern(zval *arg, const char *key, zend_array *arr) { add_assoc_array(arg, key, arr); }
void add_assoc_object__extern(zval *arg, const char *key, zend_object *obj) { add_assoc_object(arg, key, obj); }
void add_assoc_reference__extern(zval *arg, const char *key, zend_reference *ref) { add_assoc_reference(arg, key, ref); }
void add_assoc_zval__extern(zval *arg, const char *key, zval *value) { add_assoc_zval(arg, key, value); }
zend_result add_index_zval__extern(zval *arg, zend_ulong index, zval *value) { return add_index_zval(arg, index, value); }
zend_result add_next_index_zval__extern(zval *arg, zval *value) { return add_next_index_zval(arg, value); }
void add_property_long__extern(zval *arg, const char *key, zend_long n) { add_property_long(arg, key, n); }
void add_property_null__extern(zval *arg, const char *key) { add_property_null(arg, key); }
void add_property_bool__extern(zval *arg, const char *key, bool b) { add_property_bool(arg, key, b); }
void add_property_resource__extern(zval *arg, const char *key, zend_resource *r) { add_property_resource(arg, key, r); }
void add_property_double__extern(zval *arg, const char *key, double d) { add_property_double(arg, key, d); }
void add_property_str__extern(zval *arg, const char *key, zend_string *str) { add_property_str(arg, key, str); }
void add_property_string__extern(zval *arg, const char *key, const char *str) { add_property_string(arg, key, str); }
void add_property_stringl__extern(zval *arg, const char *key, const char *str, size_t length) { add_property_stringl(arg, key, str, length); }
void add_property_array__extern(zval *arg, const char *key, zend_array *arr) { add_property_array(arg, key, arr); }
void add_property_object__extern(zval *arg, const char *key, zend_object *obj) { add_property_object(arg, key, obj); }
void add_property_reference__extern(zval *arg, const char *key, zend_reference *ref) { add_property_reference(arg, key, ref); }
void add_property_zval__extern(zval *arg, const char *key, zval *value) { add_property_zval(arg, key, value); }
bool zend_fcc_equals__extern(const zend_fcall_info_cache *a, const zend_fcall_info_cache *b) { return zend_fcc_equals(a, b); }
void zend_fcc_addref__extern(zend_fcall_info_cache *fcc) { zend_fcc_addref(fcc); }
void zend_fcc_dup__extern(zend_fcall_info_cache *dest, const zend_fcall_info_cache *src) { zend_fcc_dup(dest, src); }
void zend_fcc_dtor__extern(zend_fcall_info_cache *fcc) { zend_fcc_dtor(fcc); }
void zend_get_gc_buffer_add_fcc__extern(zend_get_gc_buffer *gc_buffer, zend_fcall_info_cache *fcc) { zend_get_gc_buffer_add_fcc(gc_buffer, fcc); }
zend_result zend_call_function_with_return_value__extern(zend_fcall_info *fci, zend_fcall_info_cache *fci_cache, zval *retval) { return zend_call_function_with_return_value(fci, fci_cache, retval); }
void zend_call_known_fcc__extern(const zend_fcall_info_cache *fcc, zval *retval_ptr, uint32_t param_count, zval *params, HashTable *named_params) { zend_call_known_fcc(fcc, retval_ptr, param_count, params, named_params); }
void zend_call_known_instance_method__extern(zend_function *fn, zend_object *object, zval *retval_ptr, uint32_t param_count, zval *params) { zend_call_known_instance_method(fn, object, retval_ptr, param_count, params); }
void zend_call_known_instance_method_with_0_params__extern(zend_function *fn, zend_object *object, zval *retval_ptr) { zend_call_known_instance_method_with_0_params(fn, object, retval_ptr); }
void zend_call_known_instance_method_with_1_params__extern(zend_function *fn, zend_object *object, zval *retval_ptr, zval *param) { zend_call_known_instance_method_with_1_params(fn, object, retval_ptr, param); }
zend_result zend_forbid_dynamic_call__extern(void) { return zend_forbid_dynamic_call(); }
const char * zend_get_object_type__extern(const zend_class_entry *ce) { return zend_get_object_type(ce); }
const char * zend_get_object_type_uc__extern(const zend_class_entry *ce) { return zend_get_object_type_uc(ce); }
bool zend_str_has_nul_byte__extern(const zend_string *str) { return zend_str_has_nul_byte(str); }
bool zend_char_has_nul_byte__extern(const char *s, size_t known_length) { return zend_char_has_nul_byte(s, known_length); }
zval * zend_try_array_init_size__extern(zval *zv, uint32_t size) { return zend_try_array_init_size(zv, size); }
zval * zend_try_array_init__extern(zval *zv) { return zend_try_array_init(zv); }
bool zend_parse_arg_bool_ex__extern(const zval *arg, bool *dest, bool *is_null, bool check_null, uint32_t arg_num, bool frameless) { return zend_parse_arg_bool_ex(arg, dest, is_null, check_null, arg_num, frameless); }
bool zend_parse_arg_bool__extern(const zval *arg, bool *dest, bool *is_null, bool check_null, uint32_t arg_num) { return zend_parse_arg_bool(arg, dest, is_null, check_null, arg_num); }
bool zend_parse_arg_long_ex__extern(zval *arg, zend_long *dest, bool *is_null, bool check_null, uint32_t arg_num, bool frameless) { return zend_parse_arg_long_ex(arg, dest, is_null, check_null, arg_num, frameless); }
bool zend_parse_arg_long__extern(zval *arg, zend_long *dest, bool *is_null, bool check_null, uint32_t arg_num) { return zend_parse_arg_long(arg, dest, is_null, check_null, arg_num); }
bool zend_parse_arg_double__extern(const zval *arg, double *dest, bool *is_null, bool check_null, uint32_t arg_num) { return zend_parse_arg_double(arg, dest, is_null, check_null, arg_num); }
bool zend_parse_arg_number__extern(zval *arg, zval **dest, bool check_null, uint32_t arg_num) { return zend_parse_arg_number(arg, dest, check_null, arg_num); }
bool zend_parse_arg_number_or_str__extern(zval *arg, zval **dest, bool check_null, uint32_t arg_num) { return zend_parse_arg_number_or_str(arg, dest, check_null, arg_num); }
bool zend_parse_arg_str_ex__extern(zval *arg, zend_string **dest, bool check_null, uint32_t arg_num, bool frameless) { return zend_parse_arg_str_ex(arg, dest, check_null, arg_num, frameless); }
bool zend_parse_arg_str__extern(zval *arg, zend_string **dest, bool check_null, uint32_t arg_num) { return zend_parse_arg_str(arg, dest, check_null, arg_num); }
bool zend_parse_arg_string__extern(zval *arg, char **dest, size_t *dest_len, bool check_null, uint32_t arg_num) { return zend_parse_arg_string(arg, dest, dest_len, check_null, arg_num); }
bool zend_parse_arg_path_str__extern(zval *arg, zend_string **dest, bool check_null, uint32_t arg_num) { return zend_parse_arg_path_str(arg, dest, check_null, arg_num); }
bool zend_parse_arg_path__extern(zval *arg, char **dest, size_t *dest_len, bool check_null, uint32_t arg_num) { return zend_parse_arg_path(arg, dest, dest_len, check_null, arg_num); }
bool zend_parse_arg_iterable__extern(zval *arg, zval **dest, bool check_null) { return zend_parse_arg_iterable(arg, dest, check_null); }
bool zend_parse_arg_array__extern(zval *arg, zval **dest, bool check_null, bool or_object) { return zend_parse_arg_array(arg, dest, check_null, or_object); }
bool zend_parse_arg_array_ht__extern(const zval *arg, HashTable **dest, bool check_null, bool or_object, bool separate) { return zend_parse_arg_array_ht(arg, dest, check_null, or_object, separate); }
bool zend_parse_arg_array_ht_or_long__extern(zval *arg, HashTable **dest_ht, zend_long *dest_long, bool *is_null, bool allow_null, uint32_t arg_num) { return zend_parse_arg_array_ht_or_long(arg, dest_ht, dest_long, is_null, allow_null, arg_num); }
bool zend_parse_arg_object__extern(zval *arg, zval **dest, zend_class_entry *ce, bool check_null) { return zend_parse_arg_object(arg, dest, ce, check_null); }
bool zend_parse_arg_obj__extern(const zval *arg, zend_object **dest, zend_class_entry *ce, bool check_null) { return zend_parse_arg_obj(arg, dest, ce, check_null); }
bool zend_parse_arg_obj_or_long__extern(zval *arg, zend_object **dest_obj, zend_class_entry *ce, zend_long *dest_long, bool *is_null, bool allow_null, uint32_t arg_num) { return zend_parse_arg_obj_or_long(arg, dest_obj, ce, dest_long, is_null, allow_null, arg_num); }
bool zend_parse_arg_resource__extern(zval *arg, zval **dest, bool check_null) { return zend_parse_arg_resource(arg, dest, check_null); }
bool zend_parse_arg_func__extern(zval *arg, zend_fcall_info *dest_fci, zend_fcall_info_cache *dest_fcc, bool check_null, char **error, bool free_trampoline) { return zend_parse_arg_func(arg, dest_fci, dest_fcc, check_null, error, free_trampoline); }
void zend_parse_arg_zval__extern(zval *arg, zval **dest, bool check_null) { zend_parse_arg_zval(arg, dest, check_null); }
void zend_parse_arg_zval_deref__extern(zval *arg, zval **dest, bool check_null) { zend_parse_arg_zval_deref(arg, dest, check_null); }
bool zend_parse_arg_array_ht_or_str__extern(zval *arg, HashTable **dest_ht, zend_string **dest_str, bool allow_null, uint32_t arg_num) { return zend_parse_arg_array_ht_or_str(arg, dest_ht, dest_str, allow_null, arg_num); }
bool zend_parse_arg_str_or_long__extern(zval *arg, zend_string **dest_str, zend_long *dest_long, bool *is_null, bool allow_null, uint32_t arg_num) { return zend_parse_arg_str_or_long(arg, dest_str, dest_long, is_null, allow_null, arg_num); }
bool zend_parse_arg_obj_or_class_name__extern(zval *arg, zend_class_entry **destination, bool allow_null) { return zend_parse_arg_obj_or_class_name(arg, destination, allow_null); }
bool zend_parse_arg_obj_or_str__extern(zval *arg, zend_object **destination_object, zend_class_entry *base_ce, zend_string **destination_string, bool allow_null, uint32_t arg_num) { return zend_parse_arg_obj_or_str(arg, destination_object, base_ce, destination_string, allow_null, arg_num); }
zend_attribute * zend_add_class_attribute__extern(zend_class_entry *ce, zend_string *name, uint32_t argc) { return zend_add_class_attribute(ce, name, argc); }
zend_attribute * zend_add_function_attribute__extern(zend_function *func, zend_string *name, uint32_t argc) { return zend_add_function_attribute(func, name, argc); }
zend_attribute * zend_add_parameter_attribute__extern(zend_function *func, uint32_t offset, zend_string *name, uint32_t argc) { return zend_add_parameter_attribute(func, offset, name, argc); }
zend_attribute * zend_add_property_attribute__extern(zend_class_entry *ce, zend_property_info *info, zend_string *name, uint32_t argc) { return zend_add_property_attribute(ce, info, name, argc); }
zend_attribute * zend_add_class_constant_attribute__extern(zend_class_entry *ce, zend_class_constant *c, zend_string *name, uint32_t argc) { return zend_add_class_constant_attribute(ce, c, name, argc); }
zend_attribute * zend_add_global_constant_attribute__extern(zend_constant *c, zend_string *name, uint32_t argc) { return zend_add_global_constant_attribute(c, name, argc); }
int zend_ulong_ntz__extern(zend_ulong num) { return zend_ulong_ntz(num); }
int zend_ulong_nlz__extern(zend_ulong num) { return zend_ulong_nlz(num); }
uint32_t zend_bitset_len__extern(uint32_t n) { return zend_bitset_len(n); }
bool zend_bitset_in__extern(zend_bitset set, uint32_t n) { return zend_bitset_in(set, n); }
void zend_bitset_incl__extern(zend_bitset set, uint32_t n) { zend_bitset_incl(set, n); }
void zend_bitset_excl__extern(zend_bitset set, uint32_t n) { zend_bitset_excl(set, n); }
void zend_bitset_clear__extern(zend_bitset set, uint32_t len) { zend_bitset_clear(set, len); }
bool zend_bitset_empty__extern(zend_bitset set, uint32_t len) { return zend_bitset_empty(set, len); }
void zend_bitset_fill__extern(zend_bitset set, uint32_t len) { zend_bitset_fill(set, len); }
bool zend_bitset_equal__extern(zend_bitset set1, zend_bitset set2, uint32_t len) { return zend_bitset_equal(set1, set2, len); }
void zend_bitset_copy__extern(zend_bitset set1, zend_bitset set2, uint32_t len) { zend_bitset_copy(set1, set2, len); }
void zend_bitset_intersection__extern(zend_bitset set1, zend_bitset set2, uint32_t len) { zend_bitset_intersection(set1, set2, len); }
void zend_bitset_union__extern(zend_bitset set1, zend_bitset set2, uint32_t len) { zend_bitset_union(set1, set2, len); }
void zend_bitset_difference__extern(zend_bitset set1, zend_bitset set2, uint32_t len) { zend_bitset_difference(set1, set2, len); }
void zend_bitset_union_with_intersection__extern(zend_bitset set1, zend_bitset set2, zend_bitset set3, zend_bitset set4, uint32_t len) { zend_bitset_union_with_intersection(set1, set2, set3, set4, len); }
void zend_bitset_union_with_difference__extern(zend_bitset set1, zend_bitset set2, zend_bitset set3, zend_bitset set4, uint32_t len) { zend_bitset_union_with_difference(set1, set2, set3, set4, len); }
bool zend_bitset_subset__extern(zend_bitset set1, zend_bitset set2, uint32_t len) { return zend_bitset_subset(set1, set2, len); }
int zend_bitset_first__extern(zend_bitset set, uint32_t len) { return zend_bitset_first(set, len); }
int zend_bitset_last__extern(zend_bitset set, uint32_t len) { return zend_bitset_last(set, len); }
int zend_bitset_pop_first__extern(zend_bitset set, uint32_t len) { return zend_bitset_pop_first(set, len); }
int zend_cpu_supports_sse2__extern(void) { return zend_cpu_supports_sse2(); }
int zend_cpu_supports_sse3__extern(void) { return zend_cpu_supports_sse3(); }
int zend_cpu_supports_ssse3__extern(void) { return zend_cpu_supports_ssse3(); }
int zend_cpu_supports_sse41__extern(void) { return zend_cpu_supports_sse41(); }
int zend_cpu_supports_sse42__extern(void) { return zend_cpu_supports_sse42(); }
int zend_cpu_supports_avx__extern(void) { return zend_cpu_supports_avx(); }
int zend_cpu_supports_avx2__extern(void) { return zend_cpu_supports_avx2(); }
int zend_cpu_supports_avx512__extern(void) { return zend_cpu_supports_avx512(); }
int zend_cpu_supports_avx512_vbmi__extern(void) { return zend_cpu_supports_avx512_vbmi(); }
int zend_cpu_supports_pclmul__extern(void) { return zend_cpu_supports_pclmul(); }
zval * zend_enum_fetch_case_name__extern(zend_object *zobj) { return zend_enum_fetch_case_name(zobj); }
zval * zend_enum_fetch_case_value__extern(zend_object *zobj) { return zend_enum_fetch_case_value(zobj); }
void zend_rethrow_exception__extern(zend_execute_data *execute_data) { zend_rethrow_exception(execute_data); }
zend_fiber * zend_fiber_from_context__extern(zend_fiber_context *context) { return zend_fiber_from_context(context); }
zend_fiber_context * zend_fiber_get_context__extern(zend_fiber *fiber) { return zend_fiber_get_context(fiber); }
zend_generator * zend_generator_get_current__extern(zend_generator *generator) { return zend_generator_get_current(generator); }
void zend_do_inheritance__extern(zend_class_entry *ce, zend_class_entry *parent_ce) { zend_do_inheritance(ce, parent_ce); }
zval * zend_call_method_with_0_params__extern(zend_object *object, zend_class_entry *obj_ce, zend_function **fn_proxy, const char *function_name, zval *retval) { return zend_call_method_with_0_params(object, obj_ce, fn_proxy, function_name, retval); }
zval * zend_call_method_with_1_params__extern(zend_object *object, zend_class_entry *obj_ce, zend_function **fn_proxy, const char *function_name, zval *retval, zval *arg1) { return zend_call_method_with_1_params(object, obj_ce, fn_proxy, function_name, retval, arg1); }
zval * zend_call_method_with_2_params__extern(zend_object *object, zend_class_entry *obj_ce, zend_function **fn_proxy, const char *function_name, zval *retval, zval *arg1, zval *arg2) { return zend_call_method_with_2_params(object, obj_ce, fn_proxy, function_name, retval, arg1, arg2); }
void zend_mmap_set_name__extern(const void *start, size_t len, const char *name) { zend_mmap_set_name(start, len, name); }
bool zend_observer_handler_is_unobserved__extern(zend_observer_fcall_begin_handler *handler) { return zend_observer_handler_is_unobserved(handler); }
bool zend_observer_fcall_has_no_observers__extern(zend_execute_data *execute_data, bool allow_generator, zend_observer_fcall_begin_handler **handler) { return zend_observer_fcall_has_no_observers(execute_data, allow_generator, handler); }
void zend_observer_fcall_begin_specialized__extern(zend_execute_data *execute_data, bool allow_generator) { zend_observer_fcall_begin_specialized(execute_data, allow_generator); }
void zend_observer_fcall_end__extern(zend_execute_data *execute_data, zval *return_value) { zend_observer_fcall_end(execute_data, return_value); }
void zend_observer_function_declared_notify__extern(zend_op_array *op_array, zend_string *name) { zend_observer_function_declared_notify(op_array, name); }
void zend_observer_class_linked_notify__extern(zend_class_entry *ce, zend_string *name) { zend_observer_class_linked_notify(ce, name); }
void zend_observer_error_notify__extern(int type, zend_string *error_filename, uint32_t error_lineno, zend_string *message) { zend_observer_error_notify(type, error_filename, error_lineno, message); }
size_t smart_str_alloc__extern(smart_str *str, size_t len, bool persistent) { return smart_str_alloc(str, len, persistent); }
char * smart_str_extend_ex__extern(smart_str *dest, size_t len, bool persistent) { return smart_str_extend_ex(dest, len, persistent); }
char * smart_str_extend__extern(smart_str *dest, size_t length) { return smart_str_extend(dest, length); }
void smart_str_free_ex__extern(smart_str *str, bool persistent) { smart_str_free_ex(str, persistent); }
void smart_str_free__extern(smart_str *str) { smart_str_free(str); }
void smart_str_0__extern(smart_str *str) { smart_str_0(str); }
size_t smart_str_get_len__extern(smart_str *str) { return smart_str_get_len(str); }
void smart_str_trim_to_size_ex__extern(smart_str *str, bool persistent) { smart_str_trim_to_size_ex(str, persistent); }
void smart_str_trim_to_size__extern(smart_str *dest) { smart_str_trim_to_size(dest); }
zend_string * smart_str_extract_ex__extern(smart_str *str, bool persistent) { return smart_str_extract_ex(str, persistent); }
zend_string * smart_str_extract__extern(smart_str *dest) { return smart_str_extract(dest); }
void smart_str_appendc_ex__extern(smart_str *dest, char ch, bool persistent) { smart_str_appendc_ex(dest, ch, persistent); }
void smart_str_appendl_ex__extern(smart_str *dest, const char *str, size_t len, bool persistent) { smart_str_appendl_ex(dest, str, len, persistent); }
void smart_str_append_ex__extern(smart_str *dest, const zend_string *src, bool persistent) { smart_str_append_ex(dest, src, persistent); }
void smart_str_append_smart_str_ex__extern(smart_str *dest, const smart_str *src, bool persistent) { smart_str_append_smart_str_ex(dest, src, persistent); }
void smart_str_append_long_ex__extern(smart_str *dest, zend_long num, bool persistent) { smart_str_append_long_ex(dest, num, persistent); }
void smart_str_append_long__extern(smart_str *dest, zend_long num) { smart_str_append_long(dest, num); }
void smart_str_append_unsigned_ex__extern(smart_str *dest, zend_ulong num, bool persistent) { smart_str_append_unsigned_ex(dest, num, persistent); }
void smart_str_append_unsigned__extern(smart_str *dest, zend_ulong num) { smart_str_append_unsigned(dest, num); }
void smart_str_appendl__extern(smart_str *dest, const char *src, size_t length) { smart_str_appendl(dest, src, length); }
void smart_str_appends_ex__extern(smart_str *dest, const char *src, bool persistent) { smart_str_appends_ex(dest, src, persistent); }
void smart_str_appends__extern(smart_str *dest, const char *src) { smart_str_appends(dest, src); }
void smart_str_append__extern(smart_str *dest, const zend_string *src) { smart_str_append(dest, src); }
void smart_str_appendc__extern(smart_str *dest, char ch) { smart_str_appendc(dest, ch); }
void smart_str_append_smart_str__extern(smart_str *dest, const smart_str *src) { smart_str_append_smart_str(dest, src); }
void smart_str_setl__extern(smart_str *dest, const char *src, size_t len) { smart_str_setl(dest, src, len); }
void smart_str_sets__extern(smart_str *dest, const char *src) { smart_str_sets(dest, src); }
size_t smart_string_alloc__extern(smart_string *str, size_t len, bool persistent) { return smart_string_alloc(str, len, persistent); }
void smart_string_free_ex__extern(smart_string *str, bool persistent) { smart_string_free_ex(str, persistent); }
void smart_string_0__extern(smart_string *str) { smart_string_0(str); }
void smart_string_appendc_ex__extern(smart_string *dest, char ch, bool persistent) { smart_string_appendc_ex(dest, ch, persistent); }
void smart_string_appendl_ex__extern(smart_string *dest, const char *str, size_t len, bool persistent) { smart_string_appendl_ex(dest, str, len, persistent); }
void smart_string_append_long_ex__extern(smart_string *dest, zend_long num, bool persistent) { smart_string_append_long_ex(dest, num, persistent); }
void smart_string_append_unsigned_ex__extern(smart_string *dest, zend_ulong num, bool persistent) { smart_string_append_unsigned_ex(dest, num, persistent); }
void smart_string_setl__extern(smart_string *dest, char *src, size_t len) { smart_string_setl(dest, src, len); }
void smart_string_reset__extern(smart_string *str) { smart_string_reset(str); }
void * zend_weakrefs_hash_add_ptr__extern(HashTable *ht, zend_object *key, void *ptr) { return zend_weakrefs_hash_add_ptr(ht, key, ptr); }
void zend_weakrefs_hash_destroy__extern(HashTable *ht) { zend_weakrefs_hash_destroy(ht); }
zend_ulong zend_object_to_weakref_key__extern(const zend_object *object) { return zend_object_to_weakref_key(object); }
zend_object * zend_weakref_key_to_object__extern(zend_ulong key) { return zend_weakref_key_to_object(key); }
void php_set_error_handling__extern(zend_error_handling_t error_handling, zend_class_entry *exception_class) { php_set_error_handling(error_handling, exception_class); }
void php_std_error_handling__extern(void) { php_std_error_handling(); }
php_stream * php_stream_from_zval_no_verify_no_error__extern(zval *zval) { return php_stream_from_zval_no_verify_no_error(zval); }
bool php_stream_zend_parse_arg_into_stream__extern(zval *arg, php_stream **destination_stream_ptr, bool check_null, uint32_t arg_num) { return php_stream_zend_parse_arg_into_stream(arg, destination_stream_ptr, check_null, arg_num); }
bool php_is_stream_path__extern(const char *filename) { return php_is_stream_path(filename); }
