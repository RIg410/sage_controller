use crate::devices::{SerialSwitch, Switch as SwitchTrait, WebBeam};
use crate::home::Home;
use crate::io::IOBuilder;
use crate::sensors::Switch;
use anyhow::Result;

#[derive(Debug)]
pub struct BadRoom {
    pub chandelier: SerialSwitch,
    pub beam: WebBeam,
    pub switch_1: Switch,
    pub switch_2: Switch,
}

impl BadRoom {
    pub fn new(io: &mut IOBuilder) -> BadRoom {
        BadRoom {
            chandelier: SerialSwitch::new(io, "bedroom_lamp", 0x01),
            beam: WebBeam::new(io, "bedroom_beam"),
            switch_1: Switch::new(io, "bedroom_1", BadRoom::on_switch_1),
            switch_2: Switch::new(io, "bedroom_2", BadRoom::on_switch_2),
        }
    }

    fn on_switch_1(home: &Home, is_on: bool) -> Result<()> {
        home.bad_room.beam.switch(is_on)
    }

    fn on_switch_2(home: &Home, is_on: bool) -> Result<()> {
        home.bad_room.chandelier.switch(is_on)
    }
}