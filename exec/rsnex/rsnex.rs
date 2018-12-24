use std::env;
use std::fs;
use std::io::Read;

enum Opcode {
    ENTER,      // enter function scope
    LEAVE,      // leave function scope
    PUSHB,      // push boolean immediate
    PUSHN,      // push number immediate
    PUSHS,      // push string immediate
    PUSHT,      // push bytes immediate
    PUSHPG,     // push pointer to global
    PUSHPPG,    // push pointer to predefined global
    PUSHPMG,    // push pointer to module global
    PUSHPL,     // push pointer to local
    PUSHPOL,    // push pointer to outer local
    PUSHI,      // push 32-bit integer immediate
    LOADB,      // load boolean
    LOADN,      // load number
    LOADS,      // load string
    LOADT,      // load bytes
    LOADA,      // load array
    LOADD,      // load dictionary
    LOADP,      // load pointer
    LOADJ,      // load object
    STOREB,     // store boolean
    STOREN,     // store number
    STORES,     // store string
    STORET,     // store bytes
    STOREA,     // store array
    STORED,     // store dictionary
    STOREP,     // store pointer
    STOREJ,     // store object
    NEGN,       // negate number
    ADDN,       // add number
    SUBN,       // subtract number
    MULN,       // multiply number
    DIVN,       // divide number
    MODN,       // modulo number
    EXPN,       // exponentiate number
    EQB,        // compare equal boolean
    NEB,        // compare unequal boolean
    EQN,        // compare equal number
    NEN,        // compare unequal number
    LTN,        // compare less number
    GTN,        // compare greater number
    LEN,        // compare less equal number
    GEN,        // compare greater equal number
    EQS,        // compare equal string
    NES,        // compare unequal string
    LTS,        // compare less string
    GTS,        // compare greater string
    LES,        // compare less equal string
    GES,        // compare greater equal string
    EQT,        // compare equal bytes
    NET,        // compare unequal bytes
    LTT,        // compare less bytes
    GTT,        // compare greater bytes
    LET_,       // compare less equal bytes
    GET,        // compare greater equal bytes
    EQA,        // compare equal array
    NEA,        // compare unequal array
    EQD,        // compare equal dictionary
    NED,        // compare unequal dictionary
    EQP,        // compare equal pointer
    NEP,        // compare unequal pointer
    ANDB,       // and boolean
    ORB,        // or boolean
    NOTB,       // not boolean
    INDEXAR,    // index array for read
    INDEXAW,    // index array for write
    INDEXAV,    // index array value
    INDEXAN,    // index array value, no exception
    INDEXDR,    // index dictionary for read
    INDEXDW,    // index dictionary for write
    INDEXDV,    // index dictionary value
    INA,        // in array
    IND,        // in dictionary
    CALLP,      // call predefined
    CALLF,      // call function
    CALLMF,     // call module function
    CALLI,      // call indirect
    JUMP,       // unconditional jump
    JF,         // jump if false
    JT,         // jump if true
    JFCHAIN,    // jump and drop next if false
    DUP,        // duplicate
    DUPX1,      // duplicate under second value
    DROP,       // drop
    RET,        // return
    CALLE,      // call foreign
    CONSA,      // construct array
    CONSD,      // construct dictionary
    EXCEPT,     // throw exception
    ALLOC,      // allocate record
    PUSHNIL,    // push nil pointer
    JNASSERT,   // jump if assertions disabled
    RESETC,     // reset cell
    PUSHPEG,    // push pointer to external global
    JUMPTBL,    // jump table
    CALLX,      // call extension
    SWAP,       // swap two top stack elements
    DROPN,      // drop element n
    PUSHM,      // push current module
    CALLV,      // call virtual
    PUSHCI,     // push class info
    MAPA,       // map array
    MAPD,       // map dictionary
}

fn get_vint(bytes: &[u8], i: &mut usize) -> usize {
    let mut r: usize = 0;
    loop {
        let x = bytes[*i];
        *i += 1;
        r = (r << 7) | (x & 0x7f) as usize;
        if x & 0x80 == 0 {
            break;
        }
    }
    r
}

#[derive(Clone)]
enum Cell {
    None,
    //Boolean(bool),
    //Number(f64),
    String(String),
    //Array(Vec<Cell>),
    //Dictionary(Vec<Cell>),
}

struct Type {
    _name: usize,
    _descriptor: usize,
}

struct Bytecode {
    source_hash: Vec<u8>,
    global_size: usize,
    strtable: Vec<String>,
    types: Vec<Type>,
    code: Vec<u8>,
}

impl Bytecode {
    fn new(bytecode: &Vec<u8>) -> Bytecode {
        let mut r = Bytecode {
            source_hash: Vec::new(),
            global_size: 0,
            strtable: Vec::new(),
            types: Vec::new(),
            code: Vec::new(),
        };
        let mut i = 0;
        r.source_hash = bytecode[i..i+32].to_vec();
        i += 32;
        r.global_size = get_vint(&bytecode, &mut i);

        let strtablesize = get_vint(&bytecode, &mut i);
        r.strtable = Bytecode::get_strtable(&bytecode[i..i+strtablesize]);
        i += strtablesize;

        let typesize = get_vint(&bytecode, &mut i);
        for _ in 1..typesize {
            let t = Type {
                _name: get_vint(&bytecode, &mut i),
                _descriptor: get_vint(&bytecode, &mut i),
            };
            r.types.push(t);
        }

        let constantsize = get_vint(&bytecode, &mut i);
        assert!(constantsize == 0);

        let variablesize = get_vint(&bytecode, &mut i);
        assert!(variablesize == 0);

        let functionsize = get_vint(&bytecode, &mut i);
        assert!(functionsize == 0);

        let exceptionexportsize = get_vint(&bytecode, &mut i);
        assert!(exceptionexportsize == 0);

        let interfaceexportsize = get_vint(&bytecode, &mut i);
        assert!(interfaceexportsize == 0);

        let importsize = get_vint(&bytecode, &mut i);
        assert!(importsize == 0);

        let functionsize = get_vint(&bytecode, &mut i);
        assert!(functionsize == 0);

        let exceptionsize = get_vint(&bytecode, &mut i);
        assert!(exceptionsize == 0);

        let classsize = get_vint(&bytecode, &mut i);
        assert!(classsize == 0);

        r.code = bytecode[i..].to_vec();

        r
    }

    fn get_strtable(bytecode: &[u8]) -> Vec<String> {
        let mut r: Vec<String> = Vec::new();
        let mut i = 0;
        while i < bytecode.len() {
            let strlen = get_vint(bytecode, &mut i);
            r.push(String::from_utf8(bytecode[i..i+strlen].to_vec()).unwrap());
            i += strlen;
        }
        r
    }
}

struct Executor {
    object: Bytecode,
    globals: Vec<Cell>,
    stack: Vec<Cell>,
    callstack: Vec<usize>,
    ip: usize,
}

impl Executor {
    fn new(bytecode: &Vec<u8>) -> Executor {
        let object = Bytecode::new(&bytecode);
        let global_size = object.global_size;
        Executor {
            object: object,
            globals: vec![Cell::None; global_size],
            stack: Vec::new(),
            callstack: Vec::new(),
            ip: 0,
        }
    }

    fn run(&mut self) {
        self.callstack.push(self.object.code.len());
        while self.ip < self.object.code.len() {
            match self.object.code[self.ip] {
                x if x == Opcode::ENTER as u8 => self.op_enter(),
                x if x == Opcode::LEAVE as u8 => self.op_leave(),
                x if x == Opcode::PUSHB as u8 => self.op_pushb(),
                x if x == Opcode::PUSHN as u8 => self.op_pushn(),
                x if x == Opcode::PUSHS as u8 => self.op_pushs(),
                x if x == Opcode::PUSHT as u8 => self.op_pusht(),
                x if x == Opcode::PUSHPG as u8 => self.op_pushpg(),
                x if x == Opcode::PUSHPPG as u8 => self.op_pushppg(),
                x if x == Opcode::PUSHPMG as u8 => self.op_pushpmg(),
                x if x == Opcode::PUSHPL as u8 => self.op_pushpl(),
                x if x == Opcode::PUSHPOL as u8 => self.op_pushpol(),
                x if x == Opcode::PUSHI as u8 => self.op_pushi(),
                x if x == Opcode::LOADB as u8 => self.op_loadb(),
                x if x == Opcode::LOADN as u8 => self.op_loadn(),
                x if x == Opcode::LOADS as u8 => self.op_loads(),
                x if x == Opcode::LOADT as u8 => self.op_loadt(),
                x if x == Opcode::LOADA as u8 => self.op_loada(),
                x if x == Opcode::LOADD as u8 => self.op_loadd(),
                x if x == Opcode::LOADP as u8 => self.op_loadp(),
                x if x == Opcode::LOADJ as u8 => self.op_loadj(),
                x if x == Opcode::STOREB as u8 => self.op_storeb(),
                x if x == Opcode::STOREN as u8 => self.op_storen(),
                x if x == Opcode::STORES as u8 => self.op_stores(),
                x if x == Opcode::STORET as u8 => self.op_storet(),
                x if x == Opcode::STOREA as u8 => self.op_storea(),
                x if x == Opcode::STORED as u8 => self.op_stored(),
                x if x == Opcode::STOREP as u8 => self.op_storep(),
                x if x == Opcode::STOREJ as u8 => self.op_storej(),
                x if x == Opcode::NEGN as u8 => self.op_negn(),
                x if x == Opcode::ADDN as u8 => self.op_addn(),
                x if x == Opcode::SUBN as u8 => self.op_subn(),
                x if x == Opcode::MULN as u8 => self.op_muln(),
                x if x == Opcode::DIVN as u8 => self.op_divn(),
                x if x == Opcode::MODN as u8 => self.op_modn(),
                x if x == Opcode::EXPN as u8 => self.op_expn(),
                x if x == Opcode::EQB as u8 => self.op_eqb(),
                x if x == Opcode::NEB as u8 => self.op_neb(),
                x if x == Opcode::EQN as u8 => self.op_eqn(),
                x if x == Opcode::NEN as u8 => self.op_nen(),
                x if x == Opcode::LTN as u8 => self.op_ltn(),
                x if x == Opcode::GTN as u8 => self.op_gtn(),
                x if x == Opcode::LEN as u8 => self.op_len(),
                x if x == Opcode::GEN as u8 => self.op_gen(),
                x if x == Opcode::EQS as u8 => self.op_eqs(),
                x if x == Opcode::NES as u8 => self.op_nes(),
                x if x == Opcode::LTS as u8 => self.op_lts(),
                x if x == Opcode::GTS as u8 => self.op_gts(),
                x if x == Opcode::LES as u8 => self.op_les(),
                x if x == Opcode::GES as u8 => self.op_ges(),
                x if x == Opcode::EQT as u8 => self.op_eqt(),
                x if x == Opcode::NET as u8 => self.op_net(),
                x if x == Opcode::LTT as u8 => self.op_ltt(),
                x if x == Opcode::GTT as u8 => self.op_gtt(),
                x if x == Opcode::LET_ as u8 => self.op_let(),
                x if x == Opcode::GET as u8 => self.op_get(),
                x if x == Opcode::EQA as u8 => self.op_eqa(),
                x if x == Opcode::NEA as u8 => self.op_nea(),
                x if x == Opcode::EQD as u8 => self.op_eqd(),
                x if x == Opcode::NED as u8 => self.op_ned(),
                x if x == Opcode::EQP as u8 => self.op_eqp(),
                x if x == Opcode::NEP as u8 => self.op_nep(),
                x if x == Opcode::ANDB as u8 => self.op_andb(),
                x if x == Opcode::ORB as u8 => self.op_orb(),
                x if x == Opcode::NOTB as u8 => self.op_notb(),
                x if x == Opcode::INDEXAR as u8 => self.op_indexar(),
                x if x == Opcode::INDEXAW as u8 => self.op_indexaw(),
                x if x == Opcode::INDEXAV as u8 => self.op_indexav(),
                x if x == Opcode::INDEXAN as u8 => self.op_indexan(),
                x if x == Opcode::INDEXDR as u8 => self.op_indexdr(),
                x if x == Opcode::INDEXDW as u8 => self.op_indexdw(),
                x if x == Opcode::INDEXDV as u8 => self.op_indexdv(),
                x if x == Opcode::INA as u8 => self.op_ina(),
                x if x == Opcode::IND as u8 => self.op_ind(),
                x if x == Opcode::CALLP as u8 => self.op_callp(),
                x if x == Opcode::CALLF as u8 => self.op_callf(),
                x if x == Opcode::CALLMF as u8 => self.op_callmf(),
                x if x == Opcode::CALLI as u8 => self.op_calli(),
                x if x == Opcode::JUMP as u8 => self.op_jump(),
                x if x == Opcode::JF as u8 => self.op_jf(),
                x if x == Opcode::JT as u8 => self.op_jt(),
                x if x == Opcode::JFCHAIN as u8 => self.op_jfchain(),
                x if x == Opcode::DUP as u8 => self.op_dup(),
                x if x == Opcode::DUPX1 as u8 => self.op_dupx1(),
                x if x == Opcode::DROP as u8 => self.op_drop(),
                x if x == Opcode::RET as u8 => self.op_ret(),
                x if x == Opcode::CALLE as u8 => self.op_calle(),
                x if x == Opcode::CONSA as u8 => self.op_consa(),
                x if x == Opcode::CONSD as u8 => self.op_consd(),
                x if x == Opcode::EXCEPT as u8 => self.op_except(),
                x if x == Opcode::ALLOC as u8 => self.op_alloc(),
                x if x == Opcode::PUSHNIL as u8 => self.op_pushnil(),
                x if x == Opcode::JNASSERT as u8 => self.op_jnassert(),
                x if x == Opcode::RESETC as u8 => self.op_resetc(),
                x if x == Opcode::PUSHPEG as u8 => self.op_pushpeg(),
                x if x == Opcode::JUMPTBL as u8 => self.op_jumptbl(),
                x if x == Opcode::CALLX as u8 => self.op_callx(),
                x if x == Opcode::SWAP as u8 => self.op_swap(),
                x if x == Opcode::DROPN as u8 => self.op_dropn(),
                x if x == Opcode::PUSHM as u8 => self.op_pushm(),
                x if x == Opcode::CALLV as u8 => self.op_callv(),
                x if x == Opcode::PUSHCI as u8 => self.op_pushci(),
                x if x == Opcode::MAPA as u8 => self.op_mapa(),
                x if x == Opcode::MAPD as u8 => self.op_mapd(),
                _ => panic!("invalid opcode")
            }
        }
    }

    fn op_enter(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_leave(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushb(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushs(&mut self) {
        self.ip += 1;
        let val = get_vint(&self.object.code, &mut self.ip);
        self.stack.push(Cell::String(self.object.strtable[val].to_string()));
    }

    fn op_pusht(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushpg(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushppg(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushpmg(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushpl(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushpol(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushi(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_loadb(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_loadn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_loads(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_loadt(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_loada(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_loadd(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_loadp(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_loadj(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_storeb(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_storen(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_stores(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_storet(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_storea(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_stored(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_storep(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_storej(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_negn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_addn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_subn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_muln(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_divn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_modn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_expn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_eqb(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_neb(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_eqn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_nen(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_ltn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_gtn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_len(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_gen(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_eqs(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_nes(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_lts(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_gts(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_les(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_ges(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_eqt(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_net(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_ltt(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_gtt(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_let(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_get(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_eqa(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_nea(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_eqd(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_ned(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_eqp(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_nep(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_andb(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_orb(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_notb(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_indexar(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_indexaw(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_indexav(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_indexan(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_indexdr(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_indexdw(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_indexdv(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_ina(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_ind(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_callp(&mut self) {
        self.ip += 1;
        let val = get_vint(&self.object.code, &mut self.ip);
        if self.object.strtable[val] == "print" {
            if let Some(Cell::String(s)) = self.stack.pop() {
                println!("{}", s);
            } else {
                assert!(false, "type mismatch");
            }
        } else {
            assert!(false, "unimplemented function");
        }
    }

    fn op_callf(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_callmf(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_calli(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_jump(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_jf(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_jt(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_jfchain(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_dup(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_dupx1(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_drop(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_ret(&mut self) {
        if let Some(ip) = self.callstack.pop() {
            self.ip = ip;
        } else {
            assert!(false, "unexpected empty stack");
        }
    }

    fn op_calle(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_consa(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_consd(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_except(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_alloc(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushnil(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_jnassert(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_resetc(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushpeg(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_jumptbl(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_callx(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_swap(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_dropn(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushm(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_callv(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_pushci(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_mapa(&mut self) {
        assert!(false, "unimplemented");
    }

    fn op_mapd(&mut self) {
        assert!(false, "unimplemented");
    }

}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut f = fs::File::open(&args[1]).expect("file not found");
    let mut bytecode: Vec<u8> = Vec::new();
    f.read_to_end(&mut bytecode).expect("read error");
    let mut executor = Executor::new(&bytecode);
    executor.run();
}