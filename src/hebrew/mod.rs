pub const ALEPH: char = '\u{05D0}';
pub const BETH: char = '\u{05D1}';
pub const GIMEL: char = '\u{05D2}';
pub const DALET: char = '\u{05D3}';
pub const HE: char = '\u{05D4}';
pub const VAV: char = '\u{05D5}';
pub const ZAYIN: char = '\u{05D6}';
pub const HET: char = '\u{05D7}';
pub const TET: char = '\u{05D8}';
pub const YOD: char = '\u{05D9}';
pub const KAF: char = '\u{05DB}';
pub const KAF_FINAL: char = '\u{05DA}';
pub const LAMED: char = '\u{05DC}';
pub const MEM: char = '\u{05DE}';
pub const MEM_FINAL: char = '\u{05DD}';
pub const NUN: char = '\u{05E0}';
pub const NUN_FINAL: char = '\u{05DF}';
pub const SAMEK: char = '\u{05E1}';
pub const AYIN: char = '\u{05E2}';
pub const PE: char = '\u{05E4}';
pub const PE_FINAL: char = '\u{05E3}';
pub const TSADE: char = '\u{05E6}';
pub const TSADE_FINAL: char = '\u{05E5}';
pub const QOF: char = '\u{05E7}';
pub const RESH: char = '\u{05E8}';
pub const SHIN: char = '\u{05E9}';
pub const TAV: char = '\u{05EA}';

pub const MAQEF: char = '\u{05BE}';
pub const PASEQ: char = '\u{05C0}';
pub const SOF_PASUQ: char = '\u{05C3}';
pub const NUN_HAFUKHA: char = '\u{05C6}';
pub const GERESH: char = '\u{05F3}';
pub const GERSHAYIM: char = '\u{05F4}';

pub const PATAH: char = '\u{05B7}';
pub const QAMETS: char = '\u{05B8}';
pub const TSERE: char = '\u{05B5}';
pub const HIRIQ: char = '\u{05B4}';
pub const SHUREQ: char = '\u{05BC}';
pub const SEGOL: char = '\u{05B6}';
pub const HOLAM_HASER: char = '\u{05BA}';
pub const HOLAM: char = '\u{05B9}';
pub const QIBUTS: char = '\u{05BB}';
pub const SHEWA: char = '\u{05B0}';
pub const HATEF_SEGOL: char = '\u{05B1}';
pub const HATEF_PATAH: char = '\u{05B2}';
pub const HATEF_QAMETS: char = '\u{05B3}';
pub const QAMETS_QATAN: char = '\u{05C7}';

pub const METEG: char = '\u{05BD}';
pub const RAFE: char = '\u{05BF}';
pub const SHIN_DOT: char = '\u{05C1}';
pub const SIN_DOT: char = '\u{05C2}';
pub const UPPER: char = '\u{05C4}';
pub const LOWER: char = '\u{05C5}';

pub const ATHNAH: char = '\u{0591}';
pub const SEGOLTA: char = '\u{0592}';
pub const SHALSHALET: char = '\u{0593}';
pub const ZAQEF_QATON: char = '\u{0594}';
pub const ZAQEF_GADOL: char = '\u{0595}';
pub const TARHA: char = '\u{0596}';
pub const REVIA: char = '\u{0597}';
pub const ZARQA: char = '\u{0598}';
pub const PASHTA: char = '\u{0599}';
pub const YETIV: char = '\u{059A}';
pub const TEBIR: char = '\u{059B}';
pub const TERES: char = '\u{059C}';
pub const GERESH_MUQDAM: char = '\u{059D}';
pub const QARNEY_PARA: char = '\u{059F}';
pub const TELISHA_GEDOLA: char = '\u{05A0}';
pub const PAZER: char = '\u{05A1}';
pub const ATHNAH_HAFUKH: char = '\u{05A2}';
pub const MUNAH: char = '\u{05A3}';
pub const MAHAPAKH: char = '\u{05A4}';
pub const MERKHA: char = '\u{05A5}';
pub const MERKHA_KEFULA: char = '\u{05A6}';
pub const DARGA: char = '\u{05A7}';
pub const QADMA: char = '\u{05A8}';
pub const TELISHA_QETANA: char = '\u{05A9}';
pub const GALGAL: char = '\u{05AA}';
pub const OLE: char = '\u{05AB}';
pub const ILUY: char = '\u{05AC}';
pub const DEHI: char = '\u{05AD}';
pub const ZINOR: char = '\u{05AE}';
pub const MASORA_CIRCLE: char = '\u{05AF}';

pub const ALPHABET: [char; 22] = [
    ALEPH, BETH, GIMEL, DALET, HE, VAV, ZAYIN, HET, TET, YOD, KAF, LAMED, MEM, NUN, SAMEK, AYIN,
    PE, TSADE, QOF, RESH, SHIN, TAV,
];

pub const FINALS: [char; 5] = [KAF_FINAL, MEM_FINAL, NUN_FINAL, PE_FINAL, TSADE_FINAL];

pub const PUNCTUATION: [char; 6] = [MAQEF, PASEQ, SOF_PASUQ, NUN_HAFUKHA, GERESH, GERSHAYIM];

pub const DIACRITICS: [char; 6] = [METEG, RAFE, SHIN_DOT, SIN_DOT, UPPER, LOWER];

pub const VOWELS: [char; 14] = [
    PATAH,
    QAMETS,
    TSERE,
    HIRIQ,
    SHUREQ,
    SEGOL,
    HOLAM_HASER,
    HOLAM,
    QIBUTS,
    SHEWA,
    HATEF_SEGOL,
    HATEF_QAMETS,
    HATEF_PATAH,
    QAMETS_QATAN,
];

// Returns whether or not `c` is a consonant.
pub fn is_consonant(c: char) -> bool {
    [&ALPHABET[..], &FINALS[..]].concat().contains(&c)
}

// Returns whether or not `c` is a final consonant.
pub fn is_final_consonant(c: char) -> bool {
    FINALS[..].contains(&c)
}

// Returns whether or not `c` is a vowel.
pub fn is_vowel(v: char) -> bool {
    VOWELS[..].contains(&v)
}

mod tests {
    #[test]
    fn constants_render() {
        for l in super::ALPHABET {
            println!("{}", l);
        }
    }

    #[test]
    fn test_is_consonant() {
        //
    }
}
