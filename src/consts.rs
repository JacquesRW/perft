// loop for calculating tables
macro_rules! init {
    ($init:stmt, $idx:expr, $func:expr) => {{
        let mut res = [0; 64];
        $init
        while $idx < 64 {
            res[$idx] = $func;
            $idx += 1;
        }
        res
    }};
}

// pcs / sides
pub const E: usize = 0;
pub const WH: usize = 0;
pub const BL: usize = 1;
pub const P: usize = 2;
pub const N: usize = 3;
pub const B: usize = 4;
pub const R: usize = 5;
pub const Q: usize = 6;
pub const K: usize = 7;

// move flags
pub const QUIET: u8 = 0;
pub const DBL: u8 = 1;
pub const KS: u8 = 2;
pub const QS: u8 = 3;
pub const CAP: u8 = 4;
pub const ENP: u8 = 5;
pub const PROMO: u8 = 8;
pub const BPROMO: u8 = 9;
pub const RPROMO: u8 = 10;
pub const QPROMO: u8 = 11;
pub const PROMO_CAP: u8 = 12;
pub const BPROMO_CAP: u8 = 13;
pub const RPROMO_CAP: u8 = 14;
pub const QPROMO_CAP: u8 = 15;

// castling
pub const WQS: u8 = 8;
pub const WKS: u8 = 4;
pub const BQS: u8 = 2;
pub const BKS: u8 = 1;
pub const SIDES: [u8; 2] = [WKS | WQS, BKS | BQS];
pub const CKM: [u64; 2] = [160, 0xA000000000000000];
pub const CQM: [u64; 2] = [9, 0x0900000000000000];
pub const B1C1D1: u64 = 14;
pub const F1G1: u64 = 96;
pub const B8C8D8: u64 = 0x0E00000000000000;
pub const F8G8: u64 = 0x6000000000000000;
pub static CR: [u8; 64] = [7, 15, 15, 15, 3, 15, 15, 11, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 15, 13, 15, 15, 15, 12, 15, 15, 14];

// attacks
pub const MSB: u64 = 0x80_00_00_00_00_00_00_00;
pub const LSB: u64 = 1;
pub const PENRANK: [u64; 2] = [0x00FF000000000000, 0x000000000000FF00];
pub const DBLRANK: [u64; 2] = [0x00000000FF000000, 0x000000FF00000000];
pub const WEST: [u64; 64] = init!(let mut idx = 0, idx, ((1 << idx) - 1) & (0xFF << (idx & 56)));
pub const EAST: [u64; 64] = init!(let mut idx = 0, idx, (1 << idx) ^ WEST[idx] ^ (0xFF << (idx & 56)));
pub const NATT: [u64; 64] = [132096, 329728, 659712, 1319424, 2638848, 5277696, 10489856, 4202496, 33816580, 84410376, 168886289, 337772578, 675545156, 1351090312, 2685403152, 1075839008, 8657044482, 21609056261, 43234889994, 86469779988, 172939559976, 345879119952, 687463207072, 275414786112, 2216203387392, 5531918402816, 11068131838464, 22136263676928, 44272527353856, 88545054707712, 175990581010432, 70506185244672, 567348067172352, 1416171111120896, 2833441750646784, 5666883501293568, 11333767002587136, 22667534005174272, 45053588738670592, 18049583422636032, 145241105196122112, 362539804446949376, 725361088165576704, 1450722176331153408, 2901444352662306816, 5802888705324613632, 11533718717099671552, 4620693356194824192, 288234782788157440, 576469569871282176, 1224997833292120064, 2449995666584240128, 4899991333168480256, 9799982666336960512, 1152939783987658752, 2305878468463689728, 1128098930098176, 2257297371824128, 4796069720358912, 9592139440717824, 19184278881435648, 38368557762871296, 4679521487814656, 9077567998918656];
pub const KATT: [u64; 64] = [770, 1797, 3594, 7188, 14376, 28752, 57504, 49216, 197123, 460039, 920078, 1840156, 3680312, 7360624, 14721248, 12599488, 50463488, 117769984, 235539968, 471079936, 942159872, 1884319744, 3768639488, 3225468928, 12918652928, 30149115904, 60298231808, 120596463616, 241192927232, 482385854464, 964771708928, 825720045568, 3307175149568, 7718173671424, 15436347342848, 30872694685696, 61745389371392, 123490778742784, 246981557485568, 211384331665408, 846636838289408, 1975852459884544, 3951704919769088, 7903409839538176, 15806819679076352, 31613639358152704, 63227278716305408, 54114388906344448, 216739030602088448, 505818229730443264, 1011636459460886528, 2023272918921773056, 4046545837843546112, 8093091675687092224, 16186183351374184448, 13853283560024178688, 144959613005987840, 362258295026614272, 724516590053228544, 1449033180106457088, 2898066360212914176, 5796132720425828352, 11592265440851656704, 4665729213955833856];
pub const PATT: [[u64; 64];2] = [
    [512, 1280, 2560, 5120, 10240, 20480, 40960, 16384, 131072, 327680, 655360, 1310720, 2621440, 5242880, 10485760, 4194304, 33554432, 83886080, 167772160, 335544320, 671088640, 1342177280, 2684354560, 1073741824, 8589934592, 21474836480, 42949672960, 85899345920, 171798691840, 343597383680, 687194767360, 274877906944, 2199023255552, 5497558138880, 10995116277760, 21990232555520, 43980465111040, 87960930222080, 175921860444160, 70368744177664, 562949953421312, 1407374883553280, 2814749767106560, 5629499534213120, 11258999068426240, 22517998136852480, 45035996273704960, 18014398509481984, 144115188075855872, 360287970189639680, 720575940379279360, 1441151880758558720, 2882303761517117440, 5764607523034234880, 11529215046068469760, 4611686018427387904, 0, 0, 0, 0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0, 0, 0, 0, 2, 5, 10, 20, 40, 80, 160, 64, 512, 1280, 2560, 5120, 10240, 20480, 40960, 16384, 131072, 327680, 655360, 1310720, 2621440, 5242880, 10485760, 4194304, 33554432, 83886080, 167772160, 335544320, 671088640, 1342177280, 2684354560, 1073741824, 8589934592, 21474836480, 42949672960, 85899345920, 171798691840, 343597383680, 687194767360, 274877906944, 2199023255552, 5497558138880, 10995116277760, 21990232555520, 43980465111040, 87960930222080, 175921860444160, 70368744177664, 562949953421312, 1407374883553280, 2814749767106560, 5629499534213120, 11258999068426240, 22517998136852480, 45035996273704960, 18014398509481984]
];

// hyperbola quintessence rook and bishop attacks
#[derive(Clone, Copy)]
pub struct Mask {
    pub bit: u64,
    pub diag: u64,
    pub anti: u64,
    pub file: u64,
}

pub const FILE: u64 = 0x0101_0101_0101_0101;
pub const DIAGS: [u64; 15] = [
    0x0100000000000000, 0x0201000000000000, 0x0402010000000000, 0x0804020100000000, 0x1008040201000000,
    0x2010080402010000, 0x4020100804020100, 0x8040201008040201, 0x0080402010080402, 0x0000804020100804,
    0x0000008040201008, 0x0000000080402010, 0x0000000000804020, 0x0000000000008040, 0x0000000000000080,
];
pub const ANTIS: [u64; 15] = [
    0x0000000000000001, 0x0000000000000102, 0x0000000000010204, 0x0000000001020408, 0x0000000102040810,
    0x0000010204081020, 0x0001020408102040, 0x0102040810204080, 0x0204081020408000, 0x0408102040800000,
    0x0810204080000000, 0x1020408000000000, 0x2040800000000000, 0x4080000000000000, 0x8000000000000000,
];

pub static MASKS: [Mask; 64] = {
    let mut masks: [Mask; 64] = [Mask { bit: 0, diag: 0, anti: 0, file: 0} ; 64];
    let mut idx: usize = 0;
    while idx < 64 {
        masks[idx].bit = 1 << idx;
        masks[idx].diag = DIAGS[(7 + (idx & 7) - (idx >> 3))] ^ (1 << idx);
        masks[idx].anti = ANTIS[((idx & 7) + (idx >> 3))] ^ (1 << idx);
        masks[idx].file = (FILE << (idx & 7)) ^ (1 << idx) ;
        idx += 1;
    }
    masks
};