use aluvm::aluasm;
use aluvm::{isa::Instr, library::Lib, Prog, Vm};

fn main() {
    let code = aluasm! {
            clr     r1024[5]                        ;
            put     a16[8],378                      ;
            putif   r128[5],0xaf67937b5498dc        ;
            swp     a8[1],a8[2]                     ;
            swp     f256[8],f256[7]                 ;
            dup     a256[1],a256[7]                 ;
            mov     a16[1],a16[2]                   ;
            mov     r256[8],r256[7]                 ;
            cpy     a256[1],a256[7]                 ;
            ret                                     ;
            jmp     0                               ;
    };

    let lib = Lib::assemble(&code).unwrap();
    let program = Prog::<Instr>::new(lib);
    let mut vm = Vm::<Instr>::new();
    match vm.run(&program, &()) {
        true => println!("success"),
        false => println!("failure"),
    }
}
