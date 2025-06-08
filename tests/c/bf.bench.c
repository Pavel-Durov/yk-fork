// Compiler:
//   env-var: YKB_EXTRA_CC_FLAGS=-O0
// Run-time:
//   env-var: YKD_SERIALISE_COMPILATION=1
//   stderr:
//     ...

// This is bf_base.c from https://github.com/ykjit/ykcbf modified to hard-code the input to the
// interpreter (hello.bf from the same repo).

#include <err.h>
#include <fcntl.h>
#include <stdbool.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/stat.h>
#include <unistd.h>
#include <yk.h>
#include <yk_testing.h>

#define CELLS_LEN 99999999

int interp(char *prog, char *prog_end, char *cells, char *cells_end, YkMT *mt,
           YkLocation *yklocs) {
  char *inst = prog;
  char *cell = cells;
  while (inst < prog_end) {
    yk_mt_control_point(mt, &yklocs[inst - prog]);
    switch (*inst) {
    case '>': {
      if (cell++ == cells_end)
        errx(1, "out of memory");
      break;
    }
    case '<': {
      if (cell > cells)
        cell--;
      break;
    }
    case '+': {
      (*cell)++;
      break;
    }
    case '-': {
      (*cell)--;
      break;
    }
    case '.': {
      if (putchar(*cell) == EOF)
        err(1, "(stdout)");
      break;
    }
    case ',': {
      if (read(STDIN_FILENO, cell, 1) == -1)
        err(1, "(stdin)");
      break;
    }
    case '[': {
      if (*cell == 0) {
        int count = 0;
        while (true) {
          inst++;
          if (*inst == ']') {
            if (count == 0)
              break;
            count--;
          } else if (*inst == '[')
            count++;
        }
      }
      break;
    }
    case ']': {
      if (*cell != 0) {
        int count = 0;
        while (true) {
          inst--;
          if (*inst == '[') {
            if (count == 0)
              break;
            count--;
          } else if (*inst == ']')
            count++;
        }
      }
      break;
    }
    default:
      break;
    }
    inst++;
  }
  return 0;
}

// https://github.com/cwfitzgerald/brainfuck-benchmark/tree/master/benches
// https://github.com/baris-inandi/bfgo/blob/main/examples/primes.bf
int main(int argc, char *argv[]) {  
  if (argc != 2) {
    fprintf(stderr, "Usage: %s <brainfuck_file>\n", argv[0]);
    fprintf(stderr, "Examples:\n");
    fprintf(stderr, "  %s ./bf_examples/hello_world.bf\n", argv[0]);
    fprintf(stderr, "  %s /home/pd/yk-fork/tests/bf_examples/bench.bf\n", argv[0]);
    exit(1);
  }
  
  char *filename = argv[1];
  
  printf("test: filename: %s\n", filename);
  // Open and read the file
  FILE *file = fopen(filename, "r");
  if (file == NULL) {
    err(1, "Failed to open file: %s", filename);
  }
  
  // Get file size
  fseek(file, 0, SEEK_END);
  long file_size = ftell(file);
  fseek(file, 0, SEEK_SET);
  
  if (file_size < 0) {
    fclose(file);
    err(1, "Failed to get file size");
  }
  
  // Allocate buffer and read file contents
  char *prog = malloc(file_size + 1);
  if (prog == NULL) {
    fclose(file);
    err(1, "Failed to allocate memory for program");
  }
  
  size_t bytes_read = fread(prog, 1, file_size, file);
  fclose(file);
  
  if (bytes_read != (size_t)file_size) {
    free(prog);
    err(1, "Failed to read complete file");
  }
  
  prog[file_size] = '\0'; // Null terminate
  
  size_t prog_len = file_size;
  
  char *cells = calloc(1, CELLS_LEN);
  if (cells == NULL) {
    free(prog);
    err(1, "out of memory");
  }
  char *cells_end = cells + CELLS_LEN;

  YkMT *mt = yk_mt_new(NULL);
  yk_mt_hot_threshold_set(mt, 0);

  YkLocation *yklocs = calloc(prog_len, sizeof(YkLocation));
  if (yklocs == NULL) {
    free(prog);
    free(cells);
    err(1, "out of memory");
  }
  for (size_t i = 0; i < prog_len; i++) {
    if (prog[i] == ']')
      yklocs[i] = yk_location_new();
    else
      yklocs[i] = yk_location_null();
  }

  interp(prog, &prog[prog_len], cells, cells_end, mt, yklocs);

  free(prog);
  free(cells);
  free(yklocs);
  return 0;
}
