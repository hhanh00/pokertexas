enum ActionAllowed {
    NoAction, FoldOrCall, Any
}

enum PlayerStatus {
    Playing, Folded, Allin
}

struct PlayerState {
    idx: usize,
    player: Player,
    pos: usize,
    in_pot: u64,
    action_allowed: ActionAllowed,
    status: PlayerStatus,
    score: u64,
    show_hand: bool,
}

fn run_betting_round(phase: u8, bb: u64, mut bet_amount: u64, player_states: &mut [PlayerState]) {
    let N = player_states.len();
    let mut last_raise = bb;

    if phase == 1 {
        bet_amount = bb; // + ante
    }

    // reset actions allowed at the beginning of the betting round
    for ps in player_states {
        if ps.status == PlayerStatus::Playing {
            ps.action_allowed = ActionAllowed::Any;
        }
    }

    if phase == 0 {
        return
    }

    // first to act
    let mut i = if phase == 1 && N == 2 && sb_idx >= 0 { 0 } else { 1 };

    // check if hole cards should be revealed

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
