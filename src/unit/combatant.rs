pub mod buffs;
mod res;
use buffs::{Leader::LeaderBuff, Os::OsBuff, Skill::SkillBuff};
use res::Res;
use rust_decimal::prelude::*;

#[derive(Clone)]
pub struct Chick {
  leader_buff: LeaderBuff,
  os_buff: OsBuff,
}

fn mul_floored(target: Decimal, scale: Decimal) -> Decimal {
  let result = target * scale;
  result.floor()
}

impl Chick {
  pub fn calc_exp(
    &self,
    base: Decimal,
    skill_buff: Decimal,
  ) -> Decimal {
    let first = self.leader_buff.0;
    let second = self.os_buff.0 + skill_buff;
    [first, second].into_iter().fold(base, mul_floored)
  }
  pub fn new(leader_buff: LeaderBuff, os_buff: OsBuff) -> Self {
    Self {
      leader_buff,
      os_buff,
    }
  }
}

pub struct Accompanying {
  pub res: Res,
  pub skill_buff: SkillBuff,
}

pub mod Accompanyings {
  use super::*;
  pub const EMILY_2: Accompanying = Accompanying {
    res: res::EMILY_2,
    skill_buff: buffs::Skill::NONE,
  };

  pub const EMILY_3: Accompanying = Accompanying {
    res: res::EMILY_3,
    skill_buff: buffs::Skill::NONE,
  };

  pub const ALEXANDRA: Accompanying = Accompanying {
    res: res::ALEXANDRA,
    skill_buff: buffs::Skill::ALEXANDRA,
  };

  pub const TOMY_WORKER_BIO: Accompanying = Accompanying {
    res: res::TOMY_WORKER,
    skill_buff: buffs::Skill::TOMY_WORKER_BIO,
  };

  pub const TOMY_WORKER_AGS: Accompanying = Accompanying {
    res: res::TOMY_WORKER,
    skill_buff: buffs::Skill::TOMY_WORKER_AGS,
  };

  pub const MIGHTY_R_S: Accompanying = Accompanying {
    res: res::MIGHTY_R_S,
    skill_buff: buffs::Skill::MIGHTLY_R,
  };

  pub const MIGHTY_R_SS: Accompanying = Accompanying {
    res: res::MIGHTY_R_SS,
    skill_buff: buffs::Skill::MIGHTLY_R,
  };
}

#[cfg(test)]
mod chick {
  use super::*;
  use rust_decimal_macros::dec;
  #[test]
  fn calc_exp() {
    let chick = {
      let leader_buff = buffs::Leader::LEADER;
      let os_buff = buffs::Os::IMPROVED;
      Chick {
        leader_buff,
        os_buff,
      }
    };
    let exp = chick.calc_exp(dec!(100), dec!(0.1));
    // 100*1.2=120, 120*1.375=165
    let expected = dec!(165);
    assert_eq!(exp, expected);
  }
}
