#ifndef _OPCODE_H
#define _OPCODE_H

#define FOREACH_OPCODE(OPCODE) \
    OPCODE(ENTER)\
    OPCODE(LEAVE)\
    OPCODE(PUSHB)\
    OPCODE(PUSHN)\
    OPCODE(PUSHS)\
    OPCODE(PUSHT)\
    OPCODE(PUSHPG)\
    OPCODE(PUSHPPG)\
    OPCODE(PUSHPMG)\
    OPCODE(PUSHPL)\
    OPCODE(PUSHPOL)\
    OPCODE(PUSHI)\
    OPCODE(LOADB)\
    OPCODE(LOADN)\
    OPCODE(LOADS)\
    OPCODE(LOADT)\
    OPCODE(LOADA)\
    OPCODE(LOADD)\
    OPCODE(LOADP)\
    OPCODE(LOADJ)\
    OPCODE(STOREB)\
    OPCODE(STOREN)\
    OPCODE(STORES)\
    OPCODE(STORET)\
    OPCODE(STOREA)\
    OPCODE(STORED)\
    OPCODE(STOREP)\
    OPCODE(STOREJ)\
    OPCODE(NEGN)\
    OPCODE(ADDN)\
    OPCODE(SUBN)\
    OPCODE(MULN)\
    OPCODE(DIVN)\
    OPCODE(MODN)\
    OPCODE(EXPN)\
    OPCODE(EQB)\
    OPCODE(NEB)\
    OPCODE(EQN)\
    OPCODE(NEN)\
    OPCODE(LTN)\
    OPCODE(GTN)\
    OPCODE(LEN)\
    OPCODE(GEN)\
    OPCODE(EQS)\
    OPCODE(NES)\
    OPCODE(LTS)\
    OPCODE(GTS)\
    OPCODE(LES)\
    OPCODE(GES)\
    OPCODE(EQT)\
    OPCODE(NET)\
    OPCODE(LTT)\
    OPCODE(GTT)\
    OPCODE(LET)\
    OPCODE(GET)\
    OPCODE(EQA)\
    OPCODE(NEA)\
    OPCODE(EQD)\
    OPCODE(NED)\
    OPCODE(EQP)\
    OPCODE(NEP)\
    OPCODE(ANDB)\
    OPCODE(ORB)\
    OPCODE(NOTB)\
    OPCODE(INDEXAR)\
    OPCODE(INDEXAW)\
    OPCODE(INDEXAV)\
    OPCODE(INDEXAN)\
    OPCODE(INDEXDR)\
    OPCODE(INDEXDW)\
    OPCODE(INDEXDV)\
    OPCODE(INA)\
    OPCODE(IND)\
    OPCODE(CALLP)\
    OPCODE(CALLF)\
    OPCODE(CALLMF)\
    OPCODE(CALLI)\
    OPCODE(JUMP)\
    OPCODE(JF)\
    OPCODE(JT)\
    OPCODE(JFCHAIN)\
    OPCODE(DUP)\
    OPCODE(DUPX1)\
    OPCODE(DROP)\
    OPCODE(RET)\
    OPCODE(CALLE)\
    OPCODE(CONSA)\
    OPCODE(CONSD)\
    OPCODE(EXCEPT)\
    OPCODE(ALLOC)\
    OPCODE(PUSHNIL)\
    OPCODE(JNASSERT)\
    OPCODE(RESETC)\
    OPCODE(PUSHPEG)\
    OPCODE(JUMPTBL)\
    OPCODE(CALLX)\
    OPCODE(SWAP)\
    OPCODE(DROPN)\
    OPCODE(PUSHM)\
    OPCODE(CALLV)\
    OPCODE(PUSHCI)\
    OPCODE(CONVJN)\
    OPCODE(CONVJS)

#define GENERATE_ENUM(ENUM)     ENUM,
#define GENERATE_NAME(STRING)   #STRING,

enum Opcode {
    FOREACH_OPCODE(GENERATE_ENUM)
};

static const char *sOpcode[] = {
    FOREACH_OPCODE(GENERATE_NAME)
};

#endif
