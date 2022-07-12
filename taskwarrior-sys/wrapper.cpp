#include "wrapper.h"
#include "vendor/taskwarrior/src/Context.h"
#include <iostream>
#include <string>

Context *newContext() { return new Context; }

void Context_setContext(Context *c) { c->setContext(c); }

Context *Context_getContext(Context *c) { return &c->getContext(); }

int Context_initialize(Context *c, int argc, const char **argv) {
    return c->initialize(argc, argv);
}

int Context_run(Context *c) { return c->run(); }

int Context_dispatch(Context *c, char *out) {
    std::string str = out;

    int retval = c->dispatch(str);
    strcpy(out, str.c_str());

    return retval;
}

void delContext(Context *c) { delete c; }
