use crate::root::dtos::RootDto;

struct RootController {
}

impl RootController {
    pub fn new() -> Self {
        RootController {}
    }

    pub fn get(&self, id: u64) -> Vec<RootDto>{
        unimplemented!("Not implemented yet")
    }
}