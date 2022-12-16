use super::*;

#[derive(Copy, Clone)]
pub struct Pos {
    pub pc: [u64; 6],
    pub s: [u64; 2],
    pub c: usize,
    pub state: State,
}

#[derive(Copy, Clone)]
pub struct State {
    pub enp: u8,
    pub hfm: u8,
    pub cr: u8,
}

#[derive(Copy, Clone, Default)]
pub struct Move {
    pub from: u8,
    pub to: u8,
    pub flag: u8,
    pub mpc: u8,
}

#[inline(always)]
pub fn batt(idx: usize, occ: u64) -> u64 {
    let m: Mask = MASKS[idx];
    let mut f: u64 = occ & m.diag;
    let mut r: u64 = f.swap_bytes();
    f -= m.bitmask;
    r -= m.bitmask.swap_bytes();
    f ^= r.swap_bytes();
    f &= m.diag;
    let mut f2: u64 = occ & m.antidiag;
    r = f2.swap_bytes();
    f2 -= m.bitmask;
    r -= m.bitmask.swap_bytes();
    f2 ^= r.swap_bytes();
    f2 &= m.antidiag;
    f | f2
}

#[inline(always)]
pub fn ratt(idx: usize, occ: u64) -> u64 {
    let m: Mask = MASKS[idx];
    let mut f: u64 = occ & m.file;
    let mut r: u64 = f.swap_bytes();
    f -= m.bitmask;
    r -= m.bitmask.swap_bytes();
    f ^= r.swap_bytes();
    f &= m.file;
    let mut e: u64 = EA[idx];
    let mut sq: usize = ((e & occ) | MSB).trailing_zeros() as usize;
    e ^= EA[sq];
    let mut w: u64 = WE[idx];
    sq = (((w & occ)| LSB).leading_zeros() ^ 63) as usize;
    w ^= WE[sq];
    f | e | w
}

impl Pos {
    #[inline(always)]
    pub fn toggle(&mut self, side: usize, pc: usize, bit: u64) {
        self.pc[pc] ^= bit;
        self.s[side] ^= bit;
    }

    #[inline(always)]
    pub fn is_sq_att(&self, idx: usize, side: usize, occ: u64) -> bool {
        let s: u64 = self.s[side ^ 1];
        let opp_queen: u64 = self.pc[Q] & s;
        (NATT[idx] & self.pc[N] & s > 0) || (KATT[idx] & self.pc[K] & s > 0)
        || (PATT[side][idx] & self.pc[P] & s > 0)
        || (ratt(idx, occ) & (self.pc[R] & s | opp_queen) > 0)
        || (batt(idx, occ) & (self.pc[B] & s | opp_queen) > 0)
    }

    #[inline(always)]
    pub fn get_pc(&self, bit: u64) -> usize {
        ((self.pc[N] | self.pc[R] | self.pc[K]) & bit > 0) as usize
        | (2 * ((self.pc[B] | self.pc[R]) & bit > 0) as usize)
        | (4 * ((self.pc[Q] | self.pc[K]) & bit > 0) as usize)
    }

    pub fn do_move(&mut self, m: Move) -> bool {
        let f: u64 = 1 << m.from;
        let t: u64 = 1 << m.to;
        let mpc: usize = m.mpc as usize;
        let cpc: usize = if m.flag & CAP == 0 || m.flag == ENP {E} else {self.get_pc(t)};
        let side: usize = self.c;
        self.c ^= 1;
        let opp: usize = self.c;
        self.toggle(side, mpc, f | t);
        self.state.enp = 0;
        self.state.hfm = if mpc == P || cpc != E {0} else {self.state.hfm + 1};
        if cpc != E { self.toggle(opp, cpc, t) }
        if cpc == R { self.state.cr &= CR[m.to as usize] }
        if mpc == R || mpc == K { self.state.cr &= CR[m.from as usize] }
        match m.flag {
            DBL => self.state.enp = if opp == BL {m.to - 8} else {m.to + 8},
            KS => self.toggle(side, R, CKM[side]),
            QS => self.toggle(side, R, CQM[side]),
            ENP => self.toggle(opp, P, if opp == WH {t << 8} else {t >> 8}),
            PROMO.. => {
                self.pc[mpc] ^= t;
                self.pc[((m.flag & 3) + 1) as usize] ^= t;
            }
            _ => {}
        }
        let king_idx: usize = (self.pc[K] & self.s[side]).trailing_zeros() as usize;
        self.is_sq_att(king_idx, side, self.s[0] | self.s[1])
    }
}
