#[derive(Clone)]
pub struct State {
    pub mode: Mode,
    pub status: Status,
    pub title: String
}

#[derive(Clone)]
pub enum Status {
    Dirty,
    Clean,
}

#[derive(Clone)]
pub enum Mode {
    Menu,
    SetTime,
    TimerStart,
    TimerStop,
    TimerPause,
    TimerFinish,
}

pub mod run;
