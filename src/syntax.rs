#[derive(Debug, Clone)]
pub enum Cmd {
    Inc,
    Dec,
    Fwd,
    Bwd,
    In,
    Out,
    Loop(Program),
}

pub type Program = Vec<Cmd>;
