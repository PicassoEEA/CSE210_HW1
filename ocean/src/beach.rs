use crate::color::Color;
use crate::crab::{self, Crab};
use crate::diet::Diet;
use crate::clans::ClanSystem;
use std::slice::Iter;

#[derive(Debug)]
pub struct Beach {
    crabs: Vec<Crab>,
    clan_system: ClanSystem,
}

impl Beach {
    pub fn new() -> Beach {
        Beach { crabs: Vec::new() , clan_system: ClanSystem::new()}
    }

    /**
     * Returns the number of crabs on the beach.
     */
    pub fn size(&self) -> usize {
        self.crabs.len()
    }

    /**
     * This moves `crab`, taking ownership. Do NOT implement Copy for Crab.
     *
     *   - After `add_crab` returns:
     *     - The Beach should hold the crab in its collection of crabs.
     *     - The newly added crab should be at the END of the collection.
     */
    pub fn add_crab(&mut self, crab: Crab) {
        self.crabs.push(crab);
    }

    pub fn get_crab(&self, index: usize) -> &Crab {
        self.crabs.get(index).expect("should be within range")
    }

    pub fn crabs(&self) -> Iter<Crab> {
        self.crabs.iter()
    }

    /**
     * Returns:
     *   - None if the beach is empty.
     *   - Some of a reference to the Crab with the highest speed.
     */
    pub fn get_fastest_crab(&self) -> Option<&Crab> {
        self.crabs().max_by(|x,y| x.speed().cmp(&y.speed()))
    }

    /**
     * Returns a vector of references to the crabs with a given name.
     */
    pub fn find_crabs_by_name(&self, name: &str) -> Vec<&Crab> {
        self.crabs().filter(|x| x.name() == name).collect()
    }

    /**
     * Breeds the `Crab`s at indices `i` and `j`, adding the new `Crab` to
     * the end of the beach's crab vector. If the indices are out of bounds,
     * the method should panic.
     */
    pub fn breed_crabs(&mut self, i: usize, j: usize, name: String) {
        self.add_crab(Crab::new(name, 1, Color::cross(self.get_crab(i).color(), self.get_crab(j).color()), Diet::random_diet()))
    }

    /**
     * Returns a reference to the clan system associated with the beach.
     */
    pub fn get_clan_system(&self) -> &ClanSystem {
        &self.clan_system
    }

    /**
     * Adds a crab that lives on the beach as a member to the clan system for the given clan id and the crab's name.
     * A crab can only belong to one clan.
     */
    pub fn add_member_to_clan(&mut self, clan_id: &str, crab_name: &str) {
        self.clan_system.add_member(clan_id, crab_name)
    }

    /**
     * Returns the id of the clan that wins the competition given two clan ids. The winner is decided based on the average speed of the clan members.
     * Return `None` if there are no clear winners between two different existing clans. If the inputs are invalid, return an Err string.
     */
    pub fn get_winner_clan(&self, id1: &str, id2: &str) -> Result<Option<String>, String> {
        let clan1: Vec<String> = self.clan_system.get_clan_member_names(id1);
        let clan2: Vec<String> = self.clan_system.get_clan_member_names(id2);

        if clan1.len() == 0 || clan2.len() == 0{
            Err(String::from("clan does not exist"))
        }
        else{
            let mut clan1_total = 0;
            let mut clan2_total = 0;

            for crab_name in &clan1{
                clan1_total += self.find_crabs_by_name(&crab_name)[0].speed();
            }
            for crab_name in &clan2{
                clan2_total += self.find_crabs_by_name(&crab_name)[0].speed();
            }

            let clan1_avg = clan1_total / clan1.len() as u32;
            let clan2_avg = clan2_total / clan2.len() as u32;
            
            if clan1_avg == clan2_avg {
                Ok(None)
            }
            else if clan1_avg > clan2_avg{
                Ok(Some(String::from(id1)))
            }
            else{
                Ok(Some(String::from(id2)))
            }
        }
    }
}
