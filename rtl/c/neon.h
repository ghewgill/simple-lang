typedef struct {
    double dval;
} Ne_Number;

typedef struct {
    unsigned char *ptr;
    int len;
} Ne_String;

typedef struct {
    Ne_String *str;
} Ne_Object;

void Ne_Number_assign(Ne_Number *dest, const Ne_Number *src);
void Ne_Number_init_literal(Ne_Number *num, double n);
void Ne_String_init_literal(Ne_String *str, const char *s);
void Ne_object__makeString(Ne_Object *obj, const Ne_String *s);
void Ne_print(const Ne_Object *obj);
void Ne_str(Ne_String *result, const Ne_Number *n);
