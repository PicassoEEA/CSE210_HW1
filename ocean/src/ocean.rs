use crate::beach::Beach;
use crate::prey::{Algae, Clam, Minnow, Shrimp};
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Ocean {
    beaches : Vec<Beach>,
    reefs : Vec<Rc<RefCell<Reef>>>
}

impl Ocean {
    pub fn new() -> Ocean {
        Ocean { beaches: Vec::new(), reefs: Vec::new() }
    }

    pub fn add_beach(&mut self, beach: Beach) {
        self.beaches.push(beach);
    }

    pub fn beaches(&self) -> Iter<Beach> {
        self.beaches.iter()
    }

    pub fn reefs(&self) -> Iter<Rc<RefCell<Reef>>> {
        self.reefs.iter()
    }

    /**
     * Generate a reef with the specified number of each concrete type of prey, and then add it to the ocean.
     *   - Minnows should have a speed of 25.
     *   - Shrimp should have an energy of 1.
     *
     * Returns a reference to the newly created reef.
     */
    pub fn generate_reef(
        &mut self,
        n_minnows: u32,
        n_shrimp: u32,
        n_clams: u32,
        n_algae: u32,
    ) -> Rc<RefCell<Reef>> {
        let mut reef = Reef::new();
        for _i in 0..n_minnows{
            let p = Minnow::new(25);
            reef.add_prey(Box::new(p));
        }

        for _i in 0..n_shrimp{
            let p = Shrimp::new(1);
            reef.add_prey(Box::new(p));
        }

        for _i in 0..n_clams{
            let p = Clam::new();
            reef.add_prey(Box::new(p));
        }

        for _i in 0..n_algae{
            let p = Algae::new();
            reef.add_prey(Box::new(p));
        }
        
        let ref_cell_reef = RefCell::new(reef);
        let rc_reef = Rc::new(ref_cell_reef);

        self.reefs.push(rc_reef.clone());
        rc_reef.clone()
    }
}
