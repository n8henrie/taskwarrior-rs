// shims.h
#pragma once
#include "Context.h"

inline std::unique_ptr<Context> Context_newContext() {
  return std::unique_ptr<Context>(new Context);
}

inline std::unique_ptr<Context> Context_getContext() {
  return std::unique_ptr<Context>(Context::getContext());
}

inline void Context_setContext(std::unique_ptr<Context> context) {
  Context::setContext(context.release());
}
