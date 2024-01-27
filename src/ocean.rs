use crate::beach::Beach;
use crate::prey::{Algae, Clam, Minnow, Shrimp};
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;
use std::slice::Iter;

#[derive(Debug)]
pub struct Ocean {
    // TODO: Fill in fields here.
    beaches: Vec<Beach>,
    reefs : Vec<Rc<RefCell<Reef>>>,
}

impl Ocean {
    pub fn new() -> Ocean {
        let beaches: Vec<Beach> = Vec::new();
        let reefs: Vec<Rc<RefCell<Reef>>> = Vec::new();
        Ocean{beaches,reefs}
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
        let mut reef: Reef= Reef::new();
        for _i in 0..n_minnows{
            // println!("{:?}",i);
            let new_minnow : Minnow = Minnow::new(25u32);
            let min_box : Box<Minnow> = Box::new(new_minnow);
            reef.add_prey(min_box);
        }
        for _i in 0..n_shrimp{
            // println!("{:?}",i);
            let new_shrimp : Shrimp = Shrimp::new(1u32);
            let shrimp_box: Box<Shrimp> = Box::new(new_shrimp);
            reef.add_prey(shrimp_box);
        }
        for _i in 0..n_algae{

            let new_algae : Algae = Algae::new();
            let algae_box: Box<Algae> = Box::new(new_algae);
            reef.add_prey(algae_box);
        }
        for _i in 0..n_clams{
            let new_clam : Clam = Clam::new();
            let clam_box : Box<Clam> = Box::new(new_clam);
            reef.add_prey(clam_box);
        }
        // let c=;
        let new_reef = Rc::new(RefCell::new(reef));
        let new_reef_clone = Rc::clone(&new_reef);
        self.reefs.push(new_reef);
        new_reef_clone
        
    }
}
