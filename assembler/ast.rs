use std::rc::Rc;

type Register = u8;
type Label = Rc<str>;

pub struct AST {
    pub instr_or_label: Vec<InstrOrLabel>,
}

pub enum InstrOrLabel {
    Instruction(Instruction),
    Label(Label),
}

pub enum Instruction {
    Assignment(Register, Expression),
    Statement(Statement),
}

pub enum Expression {
    Monadic { op: ExprMonOp, a1: Arg },
    Diadic { op: ExprBinOp, a1: Arg, a2: Arg },
}

pub enum Statement {
    Monadic { op: StmtMonOp, a1: Arg },
    Diadic { op: StmtBinOp, a1: Arg, a2: Arg },
}

pub enum Arg {
    Register(Register),
    Immediate(i16),
}

pub enum ExprBinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    And,
    Or,
    Xor,
    CmpEq,
    CmpNeq,
    CmpLt,
    CmpLeq,
    CmpGt,
    CmpGeq,
}

impl ExprBinOp {
    pub fn code(&self) -> u32 {
        match self {
            ExprBinOp::Add => 0b00_000,
            ExprBinOp::Sub => 0b00_001,
            ExprBinOp::Mul => 0b00_010,
            ExprBinOp::Div => 0b00_011,
            ExprBinOp::Mod => 0b00_100,
            ExprBinOp::And => 0b00_101,
            ExprBinOp::Or => 0b00_110,
            ExprBinOp::Xor => 0b00_111,
            ExprBinOp::CmpEq => 0b11_000,
            ExprBinOp::CmpNeq => 0b11_100,
            ExprBinOp::CmpLt => 0b11_001,
            ExprBinOp::CmpLeq => 0b11_110,
            ExprBinOp::CmpGt => 0b11_010,
            ExprBinOp::CmpGeq => 0b11_101,
        }
    }
}

pub enum ExprMonOp {
    Load,
}

impl ExprMonOp {
    pub fn code(&self) -> u32 {
        match self {
            Self::Load => 0b10_000,
        }
    }
}

pub enum StmtBinOp {
    Jz,
    Jzo,
    Jnz,
    Jnzo,
    Store,
}

impl StmtBinOp {
    pub fn code(&self) -> u32 {
        match self {
            Self::Jz => 0b01_000,
            Self::Jzo => 0b01_001,
            Self::Jnz => 0b01_010,
            Self::Jnzo => 0b01_011,
            Self::Store => 0b10_000,
        }
    }
}

pub enum StmtMonOp {
    Jmp,
    Jo,
}

impl StmtMonOp {
    pub fn code(&self) -> u32 {
        match self {
            Self::Jmp => 0b01_000,
            Self::Jo => 0b01_001,
        }
    }
}