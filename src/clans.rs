
#[derive(Debug)]
pub struct ClanSystem {
    // TODO: add necessary fields
    clan_names : Vec<String>,
    clans : Vec<Vec<String>>,
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        let clan_names : Vec<String> = Vec::new();
        let clans : Vec<Vec<String>> = Vec::new();
        ClanSystem{clan_names, clans}
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        let mut j=0;
        // println!("{:?}",clan_id);
        for i in &self.clan_names{
            if i==clan_id{
                return self.clans[j].clone()
            }
            j+=1;
        }
        // let s: String = clan_id.to_string();
        // self.clan_names.push(s);
        let new_clan : Vec<String> = Vec::new();
        // self.clans.push(new_clan);
        return new_clan
        // unimplemented!();
    }

    pub fn add_member(&mut self, clan_id: &str, crab: &str){
        let mut j=0;
        let mut flag=0;
        for i in &self.clan_names{
            if i == clan_id{
                flag=1;
                self.clans[j].push(crab.to_string());
            }
            j+=1;
        }
        if flag==0{
            self.clan_names.push(clan_id.to_string());
            let mut new_clan: Vec<String> = Vec::new();
            new_clan.push(crab.to_string());
            self.clans.push(new_clan);
        }
        // unimplemented!();
    }

    /**
    * Returns the number of clans currently in existence.
    */
    pub fn get_clan_count(&self) -> usize {
        self.clan_names.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        let mut j=0;
        for i in &self.clan_names{
            if i == clan_id{
                return self.clans[j].len()
            }
            j+=1;
        }
        0usize
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        let mut max=0;
        let mut cnt=1;
        let mut opt: Option<String> = None;
        for i in 0..self.clan_names.len(){
            if self.clans[i].len()==max{
                cnt += 1;
            }
            if self.clans[i].len()>max{
                max = self.clans[i].len();
                opt = Some(self.clan_names[i].clone());
                cnt = 1;
            }
        }
        if cnt==1{
            return opt; //Return Some(clan_name) if the largest clan has a count 1.
        }
        None
    }
}
