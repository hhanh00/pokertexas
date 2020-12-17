use tonic::{Request};
use api::poker::BetAction;
use api::{ActionAllowed, PlayerStatus, Player};
use futures_core::Stream;
use futures_util::StreamExt;

pub struct PlayerState<'a> {
    idx: usize,
    player: &'a Player,
    pos: usize,
    in_pot: u64,
    action_allowed: ActionAllowed,
    status: PlayerStatus,
    score: u64,
    show_hand: bool,
}

async fn run_betting_round<'a>(phase: u8, sb: u64, bb: u64,
     player_states: &mut [PlayerState<'a>],
     bets: Request<tonic::Streaming<BetAction>>
) {
    let mut bets = bets.into_inner();
    let n = player_states.len();
    let mut last_raise = bb;

    let mut bet_amount = 0;
    let mut i = 0usize;

    if phase == 0 {
        return
    }

    if phase == 1 { // pre-flop
        bet_amount = bb; // + ante
        let mut place_blinds = |sb: u64, bb: u64| {
            if sb > 0 {
                player_states[i].in_pot += sb;
                i += 1;
            }
            if bb > 0 {
                player_states[i].in_pot += bb;
                i += 1;
            }
        };
        if n != 2 { place_blinds(sb, bb); }
        else { place_blinds(bb, sb); }
    }

    // reset actions allowed at the beginning of the betting round
    for ps in player_states.iter_mut() {
        if ps.status == PlayerStatus::Playing {
            ps.action_allowed = ActionAllowed::Any;
        }
    }

    let mut acting_players_count =
        player_states
            .iter()
            .filter(|p|
                p.status == PlayerStatus::Playing &&
                    p.action_allowed != ActionAllowed::NoAction)
            .count();

    let output = async_stream::try_stream! {
        // while acting_players_count > 0 {
        //     let ps = &player_states[i];
        let Some(action) = bets.next().await {
            yield 1;
        };
        //
        //
        //
        // }
    };

}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
