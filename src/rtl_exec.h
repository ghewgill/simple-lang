#ifndef RTL_H
#define RTL_H

#include "cell.h"
#include "opstack.h"

class RtlException {
public:
    RtlException(const std::string &name, const std::string &info): name(name), info(info) {}
    std::string name;
    std::string info;
};

void rtl_exec_init(int argc, char *argv[]);
void rtl_call(opstack<Cell> &stack, const std::string &name, size_t &token);

#endif