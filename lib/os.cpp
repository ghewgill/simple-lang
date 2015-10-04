#ifdef _MSC_VER
#define _CRT_SECURE_NO_WARNINGS
#endif
#include <ctype.h>
#include <iso646.h>
#include <stdlib.h>
#include <string>

#include "number.h"

namespace rtl {

std::string os$getenv(const std::string &name)
{
    return getenv(name.c_str());
}

Number os$system(const std::string &command)
{
    std::string cmd = command;
#ifdef _WIN32
    // Terrible hack to change slashes to backslashes so cmd.exe isn't confused.
    // Probably better handled by calling a lower level function than system().
    for (std::string::iterator i = cmd.begin(); not isspace(*i); ++i) {
        if (*i == '/') {
            *i = '\\';
        }
    }
#endif
    return number_from_sint32(system(cmd.c_str()));
}

}