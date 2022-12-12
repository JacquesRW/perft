use super::{*, position::{in_check, is_sq_att, ratt, batt}};
use std::hint::unreachable_unchecked;


macro_rules! pop_lsb {($idx:expr, $x:expr) => {$idx = $x.trailing_zeros() as u16; $x &= $x - 1}}
macro_rules! push_move {($l:expr, $m:expr) => {$l.list[$l.len] = $m; $l.len += 1;}}

#[inline(always)]
fn encode(moves: &mut MoveList, mut attacks: u64, from: u16) {
    let f: u16 = from << 6;
    let mut aidx: u16;
    while attacks > 0 {
        pop_lsb!(aidx, attacks);
        push_move!(moves, f | aidx);
    }
}

pub fn gen(moves: &mut MoveList) {
    unsafe {
    let occ: u64 = POS.s[0] | POS.s[1];
    let friends: u64 = POS.s[POS.c];
    let pawns: u64 = POS.pc[P] & friends;
    match POS.c {
        0 => pawn_pushes::<WH>(moves, occ, pawns),
        1 => pawn_pushes::<BL>(moves, occ, pawns),
        _ => unreachable_unchecked(),
    }
    if POS.state.cr & SIDES[POS.c] > 0 && !in_check() {castles(moves, occ)}
    pawn_captures(moves, pawns, POS.s[POS.c ^ 1]);
    if POS.state.enp > 0 {en_passants(moves, pawns, POS.state.enp)}
    pc_moves::<N>(moves, occ, friends);
    pc_moves::<B>(moves, occ, friends);
    pc_moves::<R>(moves, occ, friends);
    pc_moves::<Q>(moves, occ, friends);
    pc_moves::<K>(moves, occ, friends);
    }
}

unsafe fn pc_moves<const PC: usize>(moves: &mut MoveList, occ: u64, friends: u64) {
    let mut from: u16;
    let mut idx: usize;
    let mut attacks: u64;
    let mut attackers: u64 = POS.pc[PC] & friends;
    while attackers > 0 {
        pop_lsb!(from, attackers);
        idx = from as usize;
        attacks = match PC {
            N => NATT[idx],
            R => ratt(idx, occ),
            B => batt(idx, occ),
            Q => ratt(idx, occ) | batt(idx, occ),
            K => KATT[idx],
            _ => panic!("Not a valid usize in fn pc_moves_general: {}", PC),
        };
        encode(moves, attacks & !friends, from);
    }
}

#[inline(always)]
unsafe fn pawn_captures(moves: &mut MoveList, mut attackers: u64, opps: u64) {
    let (mut from, mut cidx, mut f): (u16, u16, u16);
    let mut attacks: u64;
    let mut promo_attackers: u64 = attackers & PENRANK[POS.c];
    attackers &= !PENRANK[POS.c];
    while attackers > 0 {
        pop_lsb!(from, attackers);
        attacks = PATT[POS.c][from as usize] & opps;
        encode(moves, attacks, from);
    }
    while promo_attackers > 0 {
        pop_lsb!(from, promo_attackers);
        attacks = PATT[POS.c][from as usize] & opps;
        while attacks > 0 {
            pop_lsb!(cidx, attacks);
            f = from << 6;
            push_move!(moves, QPROMO_CAP | cidx | f);
            push_move!(moves, PROMO_CAP  | cidx | f);
            push_move!(moves, BPROMO_CAP | cidx | f);
            push_move!(moves, RPROMO_CAP | cidx | f);
        }
    }
}

#[inline(always)]
unsafe fn en_passants(moves: &mut MoveList, pawns: u64, sq: u16) {
    let mut attackers: u64 = PATT[POS.c ^ 1][sq as usize] & pawns;
    let mut cidx: u16;
    while attackers > 0 {
        pop_lsb!(cidx, attackers);
        push_move!(moves, ENP | sq | cidx << 6 );
    }
}

#[inline(always)]
fn shift<const SIDE: usize, const AMOUNT: u8>(bb: u64) -> u64 {
    match SIDE {
        WH => bb >> AMOUNT,
        BL => bb << AMOUNT,
        _ => panic!("Invalid side in fn shift!"),
    }
}

#[inline(always)]
fn idx_shift<const SIDE: usize, const AMOUNT: u16>(idx: u16) -> u16 {
    match SIDE {
        WH => idx + AMOUNT,
        BL => idx - AMOUNT,
        _ => panic!("Invalid side in fn shift!"),
    }
}

fn pawn_pushes<const SIDE: usize>(moves: &mut MoveList, occupied: u64, pawns: u64) {
    let empty: u64 = !occupied;
    let mut pushable_pawns: u64 = shift::<SIDE, 8>(empty) & pawns;
    let mut dbl_pushable_pawns: u64 = shift::<SIDE, 8>(shift::<SIDE, 8>(empty & DBLRANK[SIDE]) & empty) & pawns;
    let mut promotable_pawns: u64 = pushable_pawns & PENRANK[SIDE];
    pushable_pawns &= !PENRANK[SIDE];
    let mut idx: u16;
    while pushable_pawns > 0 {
        pop_lsb!(idx, pushable_pawns);
        push_move!(moves, idx_shift::<SIDE, 8>(idx) | idx << 6);
    }
    while promotable_pawns > 0 {
        pop_lsb!(idx, promotable_pawns);
        let to: u16 = idx_shift::<SIDE, 8>(idx);
        let f: u16 = idx << 6;
        push_move!(moves, QPROMO | to | f);
        push_move!(moves, PROMO  | to | f);
        push_move!(moves, BPROMO | to | f);
        push_move!(moves, RPROMO | to | f);
    }
    while dbl_pushable_pawns > 0 {
        pop_lsb!(idx, dbl_pushable_pawns);
        push_move!(moves, DBL | idx_shift::<SIDE, 16>(idx) | idx << 6);
    }
}


#[inline(always)]
unsafe fn castles(moves: &mut MoveList, occ: u64) {
    let r: u8 = POS.state.cr;
    match POS.c {
        WH => {
            if r & WQS > 0 && occ & B1C1D1 == 0 && !is_sq_att(3, WH, occ) {
                push_move!(moves, QS | 2 | 4 << 6);
            }
            if r & WKS > 0 && occ & F1G1 == 0 && !is_sq_att(5, WH, occ) {
                push_move!(moves, KS | 6 | 4 << 6);
            }
        }
        BL => {
            if r & BQS > 0 && occ & B8C8D8 == 0 && !is_sq_att(59, BL, occ) {
                push_move!(moves, QS | 58 | 60 << 6);
            }
            if r & BKS > 0 && occ & F8G8 == 0 && !is_sq_att(61, BL, occ) {
                push_move!(moves, KS | 62 | 60 << 6);
            }
        }
        _ => unreachable_unchecked(),
    }
}
