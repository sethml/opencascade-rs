// Manual bindings for C++ iostream global objects.
//
// Provides extern "C" accessors for std::cout, std::cerr, std::clog,
// and std::cin as raw pointers to Standard_OStream / Standard_IStream.
// These are global singletons — the returned pointers are valid for
// the entire program lifetime.

#include <Standard_OStream.hxx>
#include <Standard_IStream.hxx>
#include <iostream>

extern "C" Standard_OStream* iostream_cout() {
    return &std::cout;
}

extern "C" Standard_OStream* iostream_cerr() {
    return &std::cerr;
}

extern "C" Standard_OStream* iostream_clog() {
    return &std::clog;
}

extern "C" Standard_IStream* iostream_cin() {
    return &std::cin;
}
