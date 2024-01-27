use crate::color::Color;
use crate::cookbook::{Cookbook, Recipe};
use crate::diet::Diet;
use crate::prey::Prey;
use crate::reef::Reef;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
pub struct Crab {
    // TODO: Add fields here (some in part 1, some in part 2)
    name: String,
    speed: u32,
    color: Color,
    diet: Diet,
    reefs: Vec<Rc<RefCell<Reef>>>,
    is_part_of_clan: bool,
}

// Do NOT implement Copy for Crab.
impl Crab {
    pub fn new(name: String, speed: u32, color: Color, diet: Diet) -> Crab{
        let reefs : Vec<Rc<RefCell<Reef>>> = Vec::new(); 
        let is_part_of_clan : bool = false;
        Crab{name, speed, color, diet, reefs: reefs, is_part_of_clan}
    }

    pub fn name(&self) -> &str {
        // unimplemented!();
        &self.name
    }

    pub fn speed(&self) -> u32 {
        // unimplemented!();
        self.speed
    }

    pub fn color(&self) -> &Color {
        // unimplemented!();
        &self.color
    }

    pub fn diet(&self) -> Diet {
        // unimplemented!();
        self.diet
    }

    pub fn is_part_of_clan(&self) -> bool{
        self.is_part_of_clan
    }

    pub fn make_true_clan(&mut self){
        self.is_part_of_clan=true;
    }

    pub fn breed(crab1: &Crab, crab2: &Crab, name: String) -> Crab{
        // let s=1;
        let new_color: Color = Color::cross(&crab1.color,&crab2.color);
        let new_diet: Diet = Diet::random_diet();
        let new_crab: Crab = Crab::new(name, 1u32, new_color, new_diet);
        new_crab
    }

    // PART 2 BELOW
    // ------------

    /**
     * Have this crab discover a new reef, adding it to its list of reefs.
     */
    pub fn discover_reef(&mut self, reef: Rc<RefCell<Reef>>) {
        // unimplemented!();
        // println!("{:?}",Rc::clone(&reef));
        let clone_reef=Rc::clone(&reef);
        self.reefs.push(clone_reef);
    }

    /**
     * Returns Some prey from one of the reefs this crab feeds from,
     * and the index of that reef in self.reefs if able to find Some prey
     * using the `take_prey` method of Reef.
     *
     * If `take_prey` returns None, try the next reef. Try each reef only once.
     *
     * If all reefs are empty, or this crab has no reefs, return None.
     */
    // fn catch_prey(&mut self) -> Option<(Box<dyn Prey>, usize)> {
    //     // for x in self.reefs{
    //     // }
    //     // None
    //     unimplemented!();
    // }

    // /**
    //  * Releases the given prey back into the reef at the given index.
    //  */
    // fn release_prey(&mut self, prey: Box<dyn Prey>, reef_index: usize) {
    //     unimplemented!();
    // }

    /**
     * Have this crab go hunting.
     *
     * A crab will keep trying to catch prey until it succeeds,
     * or runs out of remaining prey to try to catch.
     *
     * You should keep track of all escaped prey in a local.
     *
     * Once you have finished hunting, release all escaped prey back into
     * the reefs from whence they came _before_ you return if prey was caught.
     *
     * Your algorithm might look something like this pseudocode. The challenge
     * in this task is not intended to be in figuring out the algorithm, but
     * in figuring out _how to write it in Rust_ using what you've learned so far.
     *
     * ```text
     *     there are no escaped prey yet
     *     prey has not been caught
     *     while prey can be caught
     *       if prey escapes
     *         mark the prey as escaped
     *         try again
     *     
     *       if prey is not edible by this crab
     *         mark the prey as escaped
     *         try again
     *       
     *       prey has been caught
     *       stop trying
     *     
     *     release each escaped prey back to its reef
     *     was prey caught?
     * ```
     *
     * Note: this pseudocode reads like a terrible poem.
     */
    pub fn hunt(&mut self) -> bool {
        // unimplemented!();
        for x in &self.reefs{
            // println!("{:?}",x.borrow_mut().prey().len());
            let mut y=x.borrow_mut().prey().len(); //we'll iterate over this fixed length y so we don't revisit prey that have already been processed and pushed at the back of the queue.
            while y>0{ 
                let Some(mut z)=x.borrow_mut().take_prey() else{ panic!("No prey available") };
                // println!("{:?}",z.try_escape(self));
                if z.try_escape(self){
                    x.borrow_mut().add_prey(z); //add_prey at the back of the reef if it escapes.
                }
                else{
                    if z.diet()==self.diet{
                        return true 
                    }
                    else{
                        x.borrow_mut().add_prey(z); //add prey at the back of the reef if it cannot be consumed.
                    }
                }
                y-=1;
            }
            // reef_no+=1
        }
        false
    }

    /**
     * Returns Some of any recipe from the given cookbook that matches the crab's diet
     * preferences, or None if no such recipe exists.
     *
     * IMPORTANT: you will need to add lifetime parameters to this function. It is
     * up to you to figure out which ones and where. Do not make any other changes
     * to the signature.
     */
    pub fn choose_recipe<'a>(&self, cookbook: &'a Cookbook) -> Option<&'a Recipe> {
        for x in cookbook.recipes(){
            if x.diet()==self.diet{
                return Some(&x);
            }
        }
        None
    }
}
