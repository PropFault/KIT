use crate::libs::pump::pump::Pump;

pub struct Pumper{ // Pumps pumps
    pumps : Vec<Box<dyn Pump>> // Pumps to pump
}

impl Pump for Pumper{
    fn pump(&mut self) { // Pumps the pumps in pumps
        for pump in self.pumps.iter_mut(){
            pump.pump(); // Pump pump
        }
    }
}

impl Pumper{
    pub(crate) fn new() -> Pumper{
        return Pumper{ pumps: Vec::new()}
    }
    pub(crate) fn add(&mut self, pump: Box<dyn Pump>){
        self.pumps.push(pump);
    }
}