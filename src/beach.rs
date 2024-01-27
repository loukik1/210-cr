use crate::color::Color;
use crate::crab::Crab;
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    // TODO: Declare the fields of the Beach struct here.
    size: usize,
    crabs : Vec<Crab>,
    sys : ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        // let d=0;
        // unimplemented!();
        let sys : ClanSystem = ClanSystem::new();
        let crabs : Vec<Crab>  = Vec::new();
        let d=Beach{size: 0usize, crabs, sys};
        d
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.size
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.size += 1;
        self.crabs.push(crab);
        // unimplemented!();
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        &self.crabs[index]
    }

    pub fn crabs(&self) -> Iter<Crab> {
        let tmp=self.crabs.iter();
        tmp
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */

     //What if 2 crabs have the highest speed
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        let mut fastest=0;
        let mut cnt=1;
        let mut opt: Option<&Crab> = None;
        for x in &self.crabs{
            if x.speed()==fastest{
                cnt+=1; //increases the freq if another crab has same current highest speed.
            }
            if x.speed()>fastest{
                fastest = x.speed();
                opt = Some(x);
                cnt=1;  //Stores freq of the fastest crabs. Freq gets reset to 1 when we find a new highest speed.
            }
        }
        if cnt==1{
            return opt;
        }
        None
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        let mut crab_name: Vec<&Crab> = Vec::new();
        for x in &self.crabs{
            if x.name() == name{
                crab_name.push(&x);
            }
        }
        crab_name
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        if i>=self.crabs.len() || j>=self.crabs.len() {
            panic!("Index Out of Bounds");
        }
        let new_crab: Crab = Crab::breed(&self.crabs[i],&self.crabs[j],name);
        self.crabs.push(new_crab);
        self.size+=1;
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.sys
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        // ClanSystem::add_member(clan_id,crab_name)
        for x in &mut self.crabs{
            if x.name()==crab_name{
                if x.is_part_of_clan()==false{ // checks if the crab is already a member of another clan, ignores the crab and does not add it in the current clan if it is already in another clan
                    self.sys.add_member(clan_id,crab_name);
                    x.make_true_clan(); //marks the crab as part of a clan
                }
            }
        }
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let mut len1=0;
        let mut len2=0;
        let mut avg1=0;
        let mut avg2=0;
        
        for i in self.sys.get_clan_member_names(id1){
            // println!("{:?}",i);
            for j in self.find_crabs_by_name(&i){
                // println!("{:?}",j.speed());
                avg1+=j.speed();
            }
            len1+=1;
        }
        for i in self.sys.get_clan_member_names(id2){
            // println!("{:?}",i);
            for j in self.find_crabs_by_name(&i){
                // println!("{:?}",j.speed());
                avg2+=j.speed();
            }
            len2+=1;
        }
        if len1==0 || len2==0{
            return Err("invalid".to_string())
        }
        // avg1=avg1/len1;
        // avg2=avg2/len2;
        if avg1*len2>avg2*len1{ //avoided comparison of floats for an accurate answer for hidden test cases.
            return Ok(Some(id1.to_string()))
        }
        else if avg2*len1>avg1*len2 {
            return Ok(Some(id2.to_string()))
        }
        Ok(None)
    }
}
