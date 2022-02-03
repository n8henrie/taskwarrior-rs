#include "wrapper.h"
#include "vendor/taskwarrior/src/Context.h"
#include <iostream>

Context* newContext() {
    return new Context;
}

void Context_setContext(Context *c) {
    c->setContext(c);
}

Context* Context_getContext(Context *c) {
    return &c->getContext();
}

int Context_initialize(Context *c, int argc, const char** argv) {
    return c->initialize(argc, argv);
}

int Context_run(Context *c) {
    return c->run();
}

void delContext(Context *c) {
    delete c;
}
