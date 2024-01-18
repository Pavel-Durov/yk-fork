#ifndef __YKLLVMWRAP_H
#define __YKLLVMWRAP_H

#ifdef __cplusplus
extern "C" {
#endif

extern "C" struct IRFunctionNameIndex {
  unsigned int index;
  const char *name;
};

unsigned int get_function_names(struct BitcodeSection *Bitcode,
                                const unsigned int *vec, size_t vec_size,
                                IRFunctionNameIndex **result, size_t *len);

void free_key_values(IRFunctionNameIndex *result);

#ifdef __cplusplus
}
#endif

#endif // __YKLLVMWRAP_H