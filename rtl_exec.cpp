#include <assert.h>
#include <iostream>
#include <stack>
#include <string>
#include <time.h>

#include <utf8.h>

#include "cell.h"
#include "number.h"
#include "rtl_platform.h"

typedef void (*Thunk)(std::stack<Cell> &stack, void *func);

static std::map<std::string, std::pair<Thunk, void *> > Functions;

static std::vector<std::string> g_argv;

namespace rtl {

Number abs(Number x)
{
    return number_abs(x);
}

std::string chr(Number x)
{
    assert(number_is_integer(x));
    std::string r;
    utf8::append(number_to_uint32(x), std::back_inserter(r));
    return r;
}

std::string concat(const std::string &a, const std::string &b)
{
    return a + b;
}

std::string input(const std::string &prompt)
{
    std::cout << prompt;
    std::string r;
    std::getline(std::cin, r);
    return r;
}

Number max(Number a, Number b)
{
    if (number_is_greater(a, b)) {
        return a;
    } else {
        return b;
    }
}

Number min(Number a, Number b)
{
    if (number_is_less(a, b)) {
        return a;
    } else {
        return b;
    }
}

Number num(const std::string &s)
{
    return number_from_string(s);
}

Number ord(const std::string &s)
{
    assert(utf8::distance(s.begin(), s.end()) == 1);
    auto it = s.begin();
    return number_from_uint32(utf8::next(it, s.end()));
}

void print(const std::string &s)
{
    std::cout << s << "\n";
}

std::string splice(const std::string &t, const std::string &s, Number offset, Number length)
{
    // TODO: utf8
    uint32_t o = number_to_uint32(offset);
    return s.substr(0, o) + t + s.substr(o + number_to_uint32(length));
}

std::string str(Number x)
{
    return number_to_string(x);
}

std::string strb(bool x)
{
    return x ? "TRUE" : "FALSE";
}

std::string substring(const std::string &s, Number offset, Number length)
{
    assert(number_is_integer(offset));
    assert(number_is_integer(length));
    auto start = s.begin();
    utf8::advance(start, number_to_uint32(offset), s.end());
    auto end = start;
    utf8::advance(end, number_to_uint32(length), s.end());
    return std::string(start, end);
}

Number math$acos(Number x)
{
    return number_acos(x);
}

Number math$asin(Number x)
{
    return number_asin(x);
}

Number math$atan(Number x)
{
    return number_atan(x);
}

Number math$ceil(Number x)
{
    return number_ceil(x);
}

Number math$cos(Number x)
{
    return number_cos(x);
}

Number math$exp(Number x)
{
    return number_exp(x);
}

Number math$floor(Number x)
{
    return number_floor(x);
}

Number math$log(Number x)
{
    return number_log(x);
}

Number math$sin(Number x)
{
    return number_sin(x);
}

Number math$sqrt(Number x)
{
    return number_sqrt(x);
}

Number math$tan(Number x)
{
    return number_tan(x);
}

std::vector<std::string> sys$argv()
{
    return g_argv;
}

Number time$now()
{
    return rtl_time_now();
}

} // namespace rtl

#include "thunks.inc"
#include "functions_exec.inc"

void rtl_exec_init(int argc, char *argv[])
{
    g_argv.clear();
    std::copy(argv, argv+argc, std::back_inserter(g_argv));
    for (auto f: BuiltinFunctions) {
        Functions[f.name] = std::make_pair(f.thunk, f.func);
    }
}

void rtl_call(std::stack<Cell> &stack, const std::string &name)
{
    auto f = Functions.find(name);
    if (f == Functions.end()) {
        fprintf(stderr, "simple: function not found: %s\n", name.c_str());
        abort();
    }
    f->second.first(stack, f->second.second);
}
