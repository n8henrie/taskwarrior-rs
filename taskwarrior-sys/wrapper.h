#ifdef __cplusplus
extern "C" {
#endif

typedef struct Context Context;

Context* newContext();
void Context_setContext(Context *c);
Context* Context_getContext(Context *c);
int Context_initialize(Context *c, int, const char**);
int Context_run(Context *c);
void delContext(Context *c);

#ifdef __cplusplus
}
#endif
