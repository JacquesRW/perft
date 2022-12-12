use super::*;
use std::hint::unreachable_unchecked;

macro_rules! msb {($x:expr, $t:ty) => {63 ^ $x.leading_zeros() as $t}}
macro_rules! from {($m:expr) => {(($m >> 6) & 63) as usize}}
macro_rules! to {($m:expr) => {($m & 63) as usize}}
macro_rules! bit {($x:expr) => {1 << $x}}

pub fn batt(idx: usize, occ: u64) -> u64 {
    let mut ne: u64 = NE[idx];
    let mut sq: usize = lsb!((ne & occ) | MSB, usize);
    ne ^= NE[sq];
    let mut nw: u64 = NW[idx];
    sq = lsb!((nw & occ) | MSB, usize);
    nw ^= NW[sq];
    let mut se: u64 = SE[idx];
    sq = msb!((se & occ) | LSB, usize);
    se ^= SE[sq];
    let mut sw: u64 = SW[idx];
    sq = msb!((sw & occ) | LSB, usize);
    sw ^= SW[sq];
    ne | nw | se | sw
}

pub fn ratt(idx: usize, occ: u64) -> u64 {
    let mut n: u64 = NO[idx];
    let mut sq: usize = lsb!((n & occ) | MSB, usize);
    n ^= NO[sq];
    let mut e: u64 = EA[idx];
    sq = lsb!((e & occ )| MSB, usize);
    e ^= EA[sq];
    let mut s: u64 = SO[idx];
    sq = msb!((s & occ) | LSB, usize);
    s ^= SO[sq];
    let mut w: u64 = WE[idx];
    sq = msb!((w & occ) | LSB, usize);
    w ^= WE[sq];
    n | e | s | w
}

#[inline(always)]
pub fn is_sq_att(idx: usize, side: usize, occ: u64) -> bool {
    unsafe {
    let other: usize = side ^ 1;
    let s: u64 = POS.s[other];
    let opp_queen: u64 = POS.pc[Q] & s;
    (NATT[idx] & POS.pc[N] & s > 0)
    || (KATT[idx] & POS.pc[K] & s > 0)
    || (PATT[side][idx] & POS.pc[P] & s > 0)
    || (ratt(idx, occ) & (POS.pc[R] & s | opp_queen) > 0)
    || (batt(idx, occ) & (POS.pc[B] & s | opp_queen) > 0)
    }
}

#[inline(always)]
pub fn in_check() -> bool {
    unsafe {
    let king_idx: usize = lsb!(POS.pc[K] & POS.s[POS.c], usize);
    is_sq_att(king_idx, POS.c, POS.s[0] | POS.s[1])
    }
}

#[inline(always)]
unsafe fn get_pc(bit: u64) -> usize {
    (POS.pc[N] & bit > 0) as usize
    + B * (POS.pc[B] & bit > 0) as usize
    + R * (POS.pc[R] & bit > 0) as usize
    + Q * (POS.pc[Q] & bit > 0) as usize
    + K * (POS.pc[K] & bit > 0) as usize
    + E * (!(POS.s[0] | POS.s[1]) & bit > 0) as usize
} 

pub fn do_move(m: u16) -> bool {
    unsafe {
    let (from, to): (usize, usize) = (from!(m), to!(m));
    let (f, t): (u64, u64) = (bit!(from), bit!(to));
    let (mpc, cpc): (usize, usize) = (get_pc(f), get_pc(t));
    let flag: u16 = m & 0xF000;
    let opp: usize = POS.c ^ 1;

    STACK[STACK_IDX] = MoveState { state: POS.state, m, mpc: mpc as u8, cpc: cpc as u8};
    STACK_IDX += 1;
    let mov: u64 = f | t;
    toggle!(POS.c, mpc, mov);
    POS.state.enp = 0;
    if cpc != E { toggle!(opp, cpc, t); }
    if cpc == R { POS.state.cr &= CR[to]; }
    match mpc {
        P => {
            if flag == ENP {
                let p: u64 = match opp { WH => t << 8, BL => t >> 8, _ => unreachable_unchecked() };
                toggle!(opp, P, p);
            } else if flag == DBL {
                POS.state.enp = match POS.c {WH => to - 8, BL => to + 8, _ => unreachable_unchecked()} as u16;
            } else if flag >= PROMO {
                POS.pc[mpc] ^= t;
                POS.pc[(((flag >> 12) & 3) + 1) as usize] ^= t;
            }
        }
        K => {
            POS.state.cr &= CR[from];
            if flag == KS || flag == QS {
                let c: u64 = CASTLE_MOVES[POS.c][(flag == KS) as usize];
                toggle!(POS.c, R, c);
            }
        }
        R => POS.state.cr &= CR[from],
        _ => {}
    }
    POS.state.hfm = (mpc > P && flag != CAP) as u8 * (POS.state.hfm + 1);
    POS.c ^= 1;

    let king_idx: usize = lsb!(POS.pc[K] & POS.s[opp ^ 1], usize);
    let invalid: bool = is_sq_att(king_idx, opp ^ 1, POS.s[0] | POS.s[1]);
    if invalid { undo_move() }
    invalid
    }
}

pub fn undo_move() {
    unsafe {
    STACK_IDX -= 1;
    let state: MoveState = STACK[STACK_IDX];
    let (mpc, cpc): (usize, usize) = (state.mpc as usize, state.cpc as usize);
    let (from, to): (usize, usize) = (from!(state.m), to!(state.m));
    let (f, t): (u64, u64) = (bit!(from), bit!(to));
    let flag: u16 = state.m & 0xF000;
    let opp: usize = POS.c;

    POS.c ^= 1;
    POS.state = state.state;
    let mov: u64 = f | t;
    toggle!(POS.c, mpc, mov);
    if cpc != E { toggle!(opp, cpc, t); }
    match mpc as usize {
        P =>  {
            if flag == ENP {
                let p: u64 = match opp { WH => t << 8, BL => t >> 8, _ => unreachable_unchecked() };
                toggle!(opp, P, p);
            } else if flag >= PROMO {
                POS.pc[mpc] ^= t;
                POS.pc[(((flag >> 12) & 3) + 1) as usize] ^= t;
            }
        }
        K => {
            if flag == KS || flag == QS {
                let c: u64 = CASTLE_MOVES[POS.c][(flag == KS) as usize];
                toggle!(POS.c, R, c);
            }
        }
        _ => {}
    }}
}
