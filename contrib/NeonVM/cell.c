#include <assert.h>
#include <stdarg.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "cell.h"
#include "util.h"

Cell *cell_fromNumber(BID_UINT128 n)
{
    Cell *x = cell_newCell();
    x->type = Number;
    x->number = n;
    return x;
}

Cell *cell_fromString(const char *s)
{
    Cell *x = cell_newCell();
    x->type = String;
    x->string = _strdup(s);
    return x;
}

Cell *cell_fromCell(const Cell *c)
{
    assert(c != NULL);

    Cell *x = cell_newCell();
    x->type = c->type;
    switch (c->type) {
        case Address:
            cell_copyCell(x, c);
            break;
        case String:
            x->string = _strdup(c->string);
            x->address = NULL;
            x->number = bid128_from_uint32(0);
            break;
        case Number:
            x->number = c->number;
            x->string = NULL;
            x->address = NULL;
            break;
        case None:
        default:
            x->number = bid128_from_uint32(0);
            x->string = NULL;
            x->address = NULL;
    }
    return x;
}

Cell *cell_fromAddress(Cell *c)
{
    Cell *x = cell_newCell();
    x->type = Address;
    x->address = c;
    return x;
}

void cell_copyCell(Cell *dest, const Cell *source)
{
    assert(source != NULL);
    assert(dest != NULL);

    dest->number = source->number;
    dest->string = _strdup(source->string);
    dest->address = source->address;
    dest->type = source->type;
}

void cell_resetCell(Cell *c)
{
    c->number = bid128_from_uint32(0);
    c->string = NULL;
    c->address = NULL;
    c->type = None;
}

Cell *cell_newCell()
{
    Cell *c = malloc(sizeof(Cell));
    if (c == NULL) {
        fatal_error("Could not allocate new cell object.");
    }

    c->number = bid128_from_uint32(0);
    c->string = NULL;
    c->address = NULL;
    c->type = None;
    return c;
}

void cell_freeCell(Cell *c)
{
    free(c->string);
    free(c);
}
