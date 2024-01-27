use std::{borrow::Borrow, collections::HashMap, mem};

use crate::crab;


#[derive(Debug)]
pub struct ClanSystem {
    pub clans : HashMap<String, Vec<String>>
}

impl ClanSystem {
    pub fn new() -> ClanSystem {
        ClanSystem { clans: HashMap::new()}
    }

    pub fn add_member(&mut self, clan_id: &str, crab_name: &str){
        self.clans.entry(clan_id.to_string()).and_modify(|memebers| {
            memebers.push(crab_name.to_string());
        })
        .or_insert(vec![crab_name.to_string()]);
    }

    /**
     * Returns a list of the names of the clan members for the given clan id.
     */
    pub fn get_clan_member_names(&self, clan_id: &str) -> Vec<String> {
        match self.clans.get(clan_id){
            Some(members) => members.clone(),
            None => Vec::new()
        }
    }

    /**
     * Returns the number of clans currently in existence.
     */
    pub fn get_clan_count(&self) -> usize {
        self.clans.len()
    }

    /**
     * Returns the number of clan members for the given clan id.
     */
    pub fn get_clan_member_count(&self, clan_id: &str) -> usize {
        match self.clans.get(clan_id){
            Some(members) => members.len(),
            None => 0
        }
    }

    /**
     * Returns the id of the clan with the most number of members, or None if such a clan does not exist.
     */
    pub fn get_largest_clan_id(&self) -> Option<String> {
        let mut largest: Option<String> = None;
        let mut largest_num: usize = 0;
        let mut curr_num: usize;

        for clan_id in self.clans.keys() {
            curr_num = self.get_clan_member_count(clan_id);
            if curr_num > largest_num {
                largest_num = curr_num;
                largest = Some(clan_id.clone());
            }
            
        }
        largest
    }
}