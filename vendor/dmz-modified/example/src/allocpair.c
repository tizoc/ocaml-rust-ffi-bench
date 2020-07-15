#include <caml/mlvalues.h>
#include <caml/alloc.h>
#include <caml/memory.h>
#include <caml/bigarray.h>

value caml_alloc_cell(uintnat tag, value a) {
  CAMLparam1(a);
  CAMLlocal1(r);
  r = caml_alloc(1, tag);
  Field(r, 0) = a;
  CAMLreturn(r);
}

value caml_alloc_pair(uintnat tag, value a, value b) {
  CAMLparam2(a, b);
  CAMLlocal1(r);
  r = caml_alloc(2, tag);
  Field(r, 0) = a;
  Field(r, 1) = b;
  CAMLreturn(r);
}

value caml_alloc_ntuple(uintnat numvals, value *vals) {
  CAMLparamN(vals, numvals);
  CAMLlocal1(r);
  r = caml_alloc(numvals, 0);
  for(int i = 0; i <= numvals; i++) {
    Field(r, i) = vals[i];
  }
  CAMLreturn(r);
}
