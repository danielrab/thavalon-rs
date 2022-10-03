#![allow(unused)]

use std::ops::RangeInclusive;
use rand::{Rng, seq::SliceRandom};
use enum_array::enum_array;

const MIN_PLAYERS: usize = 5;
const MAX_PLAYERS: usize = 10;

enum SuccessCard {
    Success,
    Failure,
    Reversal,
}


enum_array! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    enum Role {
        Merlin,
        Percival,
        Tristan,
        Iseult,
        Lancelot,
        Nimue,
        Arthur,
        Titania,
        Mordred,
        Morgana,
        Maelagant,
        Agravaine,
        Colgrevance,
    }
}

impl Role {
    const fn is_evil(&self) -> bool {
        use Role::*;
        matches!(self, Mordred|Morgana|Maelagant|Agravaine|Colgrevance)
    }
    const fn can_see(&self, other: &Self) -> bool {
        use Role::*;
        matches!((self, other), 
            (Merlin, Lancelot|Morgana|Maelagant|Agravaine|Colgrevance)|
            (Percival, Merlin|Morgana)|
            (Tristan|Iseult, Tristan|Iseult)|
            (Mordred|Morgana|Maelagant|Agravaine, Mordred|Morgana|Maelagant|Agravaine|Titania)
        )
    }
    const fn can_play(&self, card: SuccessCard) -> bool {
        use Role::*;
        match (self, card) {
            (Agravaine, SuccessCard::Success) => false,
            (_, SuccessCard::Success) => true,
            (_, SuccessCard::Failure) => self.is_evil(),
            (Lancelot|Maelagant, SuccessCard::Reversal) => true,
            (_, SuccessCard::Reversal) => false,
        }
    }
    const fn available_range(&self) -> RangeInclusive<usize> {
        use Role::*;
        let lower_bound = match self {
            Arthur => 7,
            Titania => 7,
            Agravaine => 8,
            Colgrevance => 10,
            _ => MIN_PLAYERS
        };
        let upper_bound = match self {
            Nimue => 5,
            _ => MAX_PLAYERS
        };
        lower_bound..=upper_bound
    }
}

#[derive(Debug)]
struct Player {
    name: String,
    role: Role,
}

#[derive(Debug)]
pub struct Game {
    players: Vec<Player>,
    good_wins: u8,
    evil_wins: u8,
    mission_attempts: u8,
    current_king: usize,
}

pub enum ShuffleMode {
    KingOnly,
    Full,
}

pub enum Error {
    InvalidPlayerCount(usize)
}

impl Game {
    pub fn new(player_names: Vec<String>, shuffle_mode: ShuffleMode) -> Result<Game, Error> {
        let rng = &mut rand::thread_rng();
        let roles = Self::get_roles(player_names.len())?;

        let mut players: Vec<Player> = player_names.into_iter()
            .zip(roles)
            .map(|(name, role)| Player {name, role})
            .collect();

        let current_king = rng.gen_range(0..players.len());
        if matches!(shuffle_mode, ShuffleMode::Full) {
            players.shuffle(rng);
        }
        
        Ok(Game { players, good_wins: 0, evil_wins: 0, mission_attempts: 0, current_king })
    }

    fn get_counts(player_count: usize) -> Result<(usize, usize), Error> {
        let res = match player_count {
            5 => (3, 2),
            6 => (4, 2),
            7 => (4, 3),
            8 => (5, 3),
            9 => (6, 3),
            10 => (6, 4),
            _ => return Err(Error::InvalidPlayerCount(player_count))
        };
        Ok(res)
    }

    fn get_roles(player_count: usize) -> Result<Vec<Role>, Error> {
        let (good_count, evil_count) = Self::get_counts(player_count)?;
        assert_eq!(good_count + evil_count, player_count, "the amount of evil and good players didn't match");
        
        let (evil_roles, good_roles): (Vec<Role>, Vec<Role>) = Role::ENTRIES.into_iter()
            .filter(|r| r.available_range().contains(&player_count))
            .partition(Role::is_evil);
        
        let rng = &mut rand::thread_rng();
        let mut roles = loop {
            let good_roles = good_roles.choose_multiple(rng, good_count);
            let evil_roles = evil_roles.choose_multiple(rng, evil_count);
            let roles = good_roles.chain(evil_roles).copied().collect::<Vec<_>>();
            if (!(roles.contains(&Role::Tristan) ^ roles.contains(&Role::Iseult))) {
                break roles;
            }
        };
        roles.shuffle(rng);
        
        Ok(roles)
    }
}