//! The manual cues: function words and morphemes that the language manuals name as diagnostic.
//!
//! Sources: `languages/arc/family.md` §5 (the comparison table and its rules of thumb), `languages/arc/mova.md` §5,
//! `languages/hbo/11-mishnaic.md` (§§5–7, the MH markers), `languages/he/` (Modern Hebrew function words).
//! Each cue carries the varieties the manual expects it to point to; the weights themselves are learned (log-odds
//! per class from the training pool), so a cue the manual over-rates is visible in `langseg cues`.
//!
//! Forms are consonantal skeletons (points and accents removed), so a cue fires on pointed and unpointed text alike.

/// A cue word: skeleton, short name, what the manual says it points to.
pub struct Cue {
    pub form: &'static str,
    pub name: &'static str,
    pub expect: &'static str,
}

const fn c(form: &'static str, name: &'static str, expect: &'static str) -> Cue {
    Cue { form, name, expect }
}

/// Whole-word cues (the skeleton must match exactly, after a single prefixed ו "and" is tried off).
pub const WORDS: &[Cue] = &[
    // the relative and genitive
    c("די", "rel_di", "arc, arc-x-targum (Aramaic relative די)"),
    c("אשר", "rel_asher", "hbo (Hebrew relative אשר)"),
    c("של", "gen_shel", "hbo-x-mishnaic, he (possessive של)"),
    // the object marker
    c("ית", "acc_yat", "arc-x-targum (object marker ית)"),
    c("את", "acc_et", "Hebrew (object marker את; Modern also 'you' f.)"),
    // "now"
    c("כען", "now_kean", "arc, arc-x-targum (כען)"),
    c("כענת", "now_kean", "arc (כענת)"),
    c("כדון", "now_kedon", "jpa (כדון)"),
    c("השתא", "now_hashta", "tmr, arc-x-zohar (השתא)"),
    c("עתה", "now_atta", "hbo (עתה)"),
    c("עכשיו", "now_akhshav", "hbo-x-mishnaic, he (עכשיו)"),
    c("עכשו", "now_akhshav", "hbo-x-mishnaic (עכשו)"),
    // existence
    c("איכא", "exist_ika", "tmr (איכא)"),
    c("ליכא", "exist_ika", "tmr (ליכא)"),
    c("אית", "exist_it", "arc-x-targum, jpa, arc-x-zohar (אית)"),
    c("לית", "exist_it", "arc-x-targum, jpa, arc-x-zohar (לית)"),
    c("איתי", "exist_itay", "arc (איתי)"),
    c("יש", "exist_yesh", "Hebrew (יש)"),
    c("אין", "exist_en", "Hebrew (אין)"),
    c("לאו", "neg_lav", "tmr (לאו)"),
    // durative particle and Babylonian question words
    c("קא", "dur_qa", "tmr (durative קא)"),
    c("מאי", "q_may", "tmr, arc-x-zohar (מאי)"),
    c("אמאי", "q_ammay", "tmr (אמאי)"),
    c("היכי", "q_heikhi", "tmr (היכי)"),
    c("היכא", "q_heikha", "tmr (היכא)"),
    c("טעמא", "q_taama", "tmr (מאי טעמא)"),
    // demonstratives
    c("האי", "dem_hai", "tmr (האי)"),
    c("הני", "dem_hanei", "tmr (הני)"),
    c("הדין", "dem_haden", "jpa, arc-x-targum (הדין)"),
    c("הדא", "dem_hada", "jpa, arc-x-targum (הדא)"),
    c("דין", "dem_den", "arc-x-targum, arc-x-zohar (דין)"),
    c("דא", "dem_da", "arc, arc-x-targum, arc-x-zohar (דא)"),
    c("דנה", "dem_dena", "arc (דנה)"),
    c("דך", "dem_dekh", "arc (דך)"),
    c("אלין", "dem_ilen", "arc, arc-x-targum, arc-x-zohar (אלין)"),
    c("אילין", "dem_ilen", "jpa (אילין)"),
    c("הלין", "dem_ilen", "jpa (הלין)"),
    c("זה", "dem_ze", "Hebrew (זה)"),
    c("זאת", "dem_zot", "hbo, he (זאת)"),
    c("זו", "dem_zo", "hbo-x-mishnaic, he (זו)"),
    c("אלה", "dem_elle", "Hebrew (אלה)"),
    c("אלו", "dem_ellu", "hbo-x-mishnaic (אלו)"),
    c("אותו", "dem_oto", "hbo-x-mishnaic, he (אותו 'that same')"),
    // pronouns and copulas
    c("איהו", "cop_ihu", "arc-x-zohar, tmr (איהו)"),
    c("איהי", "cop_ihu", "arc-x-zohar (איהי)"),
    c("ניהו", "cop_nihu", "tmr (ניהו)"),
    c("אינון", "pron_innun", "arc-x-targum, jpa, arc-x-zohar (אינון)"),
    c("אנון", "pron_innun", "arc (אנון)"),
    c("אינהו", "pron_inhu", "tmr (אינהו)"),
    c("המו", "pron_himmo", "arc (המו)"),
    c("הם", "pron_hem", "Hebrew (הם)"),
    c("הן", "pron_hen", "Hebrew (הן)"),
    c("אנן", "pron_anan", "tmr, jpa, arc-x-zohar (אנן)"),
    c("אנחנא", "pron_anahna", "arc, arc-x-targum (אנחנא)"),
    c("אנחנו", "pron_anahnu", "Hebrew (אנחנו)"),
    c("אנא", "pron_ana", "Aramaic (אנא)"),
    c("אנה", "pron_ana", "arc (אנה)"),
    c("אני", "pron_ani", "Hebrew (אני)"),
    c("אנכי", "pron_anokhi", "hbo (אנכי)"),
    c("אנת", "pron_ant", "Aramaic (אנת)"),
    c("אנתון", "pron_antun", "Aramaic (אנתון)"),
    c("אתם", "pron_attem", "Hebrew (אתם)"),
    // the l-/n-imperfect of Babylonian Aramaic
    c("לימא", "ipf_l", "tmr (לימא)"),
    c("נימא", "ipf_l", "tmr (נימא)"),
    c("ליהוי", "ipf_l", "tmr (ליהוי)"),
    c("ליתי", "ipf_l", "tmr (ליתי)"),
    c("ליעביד", "ipf_l", "tmr (ליעביד)"),
    // "see"
    c("חמי", "see_hmy", "jpa (חמי)"),
    c("חמא", "see_hmy", "jpa, arc-x-zohar (חמא)"),
    c("חזי", "see_hzy", "tmr, arc-x-zohar (חזי)"),
    c("חזא", "see_hzy", "arc-x-targum, tmr (חזא)"),
    c("חזה", "see_hzy", "arc (חזה)"),
    c("ראה", "see_raa", "Hebrew (ראה)"),
    // conjunctions
    c("ארי", "conj_are", "arc-x-targum (ארי 'because')"),
    c("ארום", "conj_arum", "jpa (ארום)"),
    c("בגין", "conj_begin", "jpa, arc-x-zohar (בגין)"),
    c("משום", "conj_mishum", "tmr (משום)"),
    c("דהא", "conj_deha", "tmr, arc-x-zohar (דהא)"),
    c("ברם", "conj_beram", "arc, arc-x-targum (ברם)"),
    c("אלהן", "conj_elahen", "arc-x-targum (אלהן)"),
    c("אלא", "conj_ella", "tmr, hbo-x-mishnaic (אלא)"),
    c("אבל", "conj_aval", "Hebrew, arc-x-zohar (אבל)"),
    c("כי", "conj_ki", "Hebrew (כי)"),
    c("כגוונא", "conj_kegavna", "arc-x-zohar (כגוונא)"),
    c("כמא", "conj_kema", "arc-x-targum (כמא)"),
    c("כמו", "conj_kemo", "he, hbo-x-mishnaic (כמו)"),
    c("כדי", "conj_kedei", "hbo-x-mishnaic, he (כדי)"),
    c("שמא", "conj_shemma", "hbo-x-mishnaic (שמא)"),
    c("כשם", "conj_keshem", "hbo-x-mishnaic (כשם)"),
    // targumic marks
    c("קדם", "tg_qodam", "arc-x-targum (קדם 'before' the LORD)"),
    c("ייי", "tg_yyy", "arc-x-targum (the divine name ייי)"),
    c("יי", "tg_yyy", "arc-x-targum (the divine name יי)"),
    c("מימרא", "tg_memra", "arc-x-targum (מימרא)"),
    c("מימריה", "tg_memra", "arc-x-targum (מימריה)"),
    // Zoharic marks
    c("רזא", "zh_raza", "arc-x-zohar (רזא)"),
    c("סטרא", "zh_sitra", "arc-x-zohar (סטרא)"),
    c("סטרין", "zh_sitra", "arc-x-zohar (סטרין)"),
    c("ת\"ח", "zh_ta_hazei", "arc-x-zohar (ת\"ח = תא חזי)"),
    c("קודשא", "zh_qudsha", "arc-x-zohar, tmr (קודשא בריך הוא)"),
    c("עלאה", "zh_illaa", "arc-x-zohar (עלאה)"),
    c("עלאין", "zh_illaa", "arc-x-zohar (עלאין)"),
    c("שכינתא", "zh_shekhinta", "arc-x-zohar (שכינתא)"),
    c("ספירה", "zh_sefira", "arc-x-zohar (ספירה)"),
    c("מלכות", "zh_sefira", "arc-x-zohar (sefira name; also Hebrew 'kingdom')"),
    c("תפארת", "zh_sefira", "arc-x-zohar (sefira name)"),
    c("בינה", "zh_sefira", "arc-x-zohar (sefira name)"),
    c("יסוד", "zh_sefira", "arc-x-zohar (sefira name)"),
    // Biblical Aramaic chancery words
    c("דת", "ba_persian", "arc (Persian דת 'law')"),
    c("פתגם", "ba_persian", "arc (Persian פתגם)"),
    c("גזבר", "ba_persian", "arc (Persian גזבר)"),
    // Mishnaic Hebrew markers
    c("אימתי", "mh_ematai", "hbo-x-mishnaic (אימתי)"),
    c("לפיכך", "mh_lefikhakh", "hbo-x-mishnaic (לפיכך)"),
    c("הרי", "mh_harei", "hbo-x-mishnaic (הרי)"),
    c("אומר", "mh_omer", "hbo-x-mishnaic (participle אומר)"),
    c("אומרים", "mh_omer", "hbo-x-mishnaic (participle אומרים)"),
    c("פטור", "mh_patur", "hbo-x-mishnaic (פטור)"),
    c("חייב", "mh_hayyav", "hbo-x-mishnaic (חייב)"),
    c("חייבין", "mh_hayyav", "hbo-x-mishnaic (חייבין)"),
    // Modern Hebrew function words
    c("מאוד", "he_meod", "he (plene מאוד)"),
    c("האם", "he_haim", "he (question האם)"),
    c("לגבי", "he_legabei", "he (לגבי)"),
    c("כלומר", "he_klomar", "he (כלומר)"),
    c("בעצם", "he_beetsem", "he (בעצם)"),
    c("כרגע", "he_karega", "he (כרגע)"),
    c("יותר", "he_yoter", "he (יותר)"),
    c("אחרי", "he_aharei", "he (אחרי)"),
    c("הזה", "he_haze", "Hebrew (הזה)"),
    c("הזאת", "he_hazot", "he (הזאת)"),
    c("היה", "he_haya", "Hebrew (היה)"),
    c("גם", "he_gam", "Hebrew (גם)"),
    c("למשל", "he_lemashal", "he (למשל)"),
    c("שנת", "he_shnat", "he (dates: שנת)"),
];

/// Morpheme cues: a prefix or suffix of the skeleton, with a minimum word length so that short roots do not fire.
pub struct Affix {
    pub pre: bool,
    pub form: &'static str,
    pub min_len: usize,
    pub name: &'static str,
    pub expect: &'static str,
}

const fn p(form: &'static str, min_len: usize, name: &'static str, expect: &'static str) -> Affix {
    Affix { pre: true, form, min_len, name, expect }
}
const fn s(form: &'static str, min_len: usize, name: &'static str, expect: &'static str) -> Affix {
    Affix { pre: false, form, min_len, name, expect }
}

pub const AFFIXES: &[Affix] = &[
    s("א", 3, "emph_a", "Aramaic (determined state ־א)"),
    s("יא", 4, "emph_ya", "Aramaic (determined ־יא, pl. ־ייא)"),
    s("ייא", 5, "emph_ayya", "arc, arc-x-targum, jpa (m.pl. determined ־ייא)"),
    s("תא", 4, "emph_ta", "Aramaic (f. determined ־תא)"),
    s("ין", 4, "pl_in", "Aramaic, hbo-x-mishnaic (m.pl. ־ין)"),
    s("ים", 4, "pl_im", "Hebrew (m.pl. ־ים)"),
    s("ות", 4, "pl_ot", "Hebrew (f.pl. ־ות)"),
    s("הון", 5, "sfx_hon", "Aramaic (their ־הון)"),
    s("הם", 4, "sfx_hem", "Hebrew (their ־הם)"),
    s("כון", 5, "sfx_khon", "Aramaic (your ־כון)"),
    s("כם", 4, "sfx_khem", "Hebrew (your ־כם)"),
    s("יה", 4, "sfx_eh", "Aramaic (his ־יה), Hebrew (-yah)"),
    s("ון", 4, "pf_un", "jpa (3mpl perfect ־ון), Aramaic"),
    s("ית", 4, "pf_it", "Aramaic (1sg perfect ־ית), Hebrew f."),
    s("תי", 4, "pf_ti", "Hebrew (1sg perfect ־תי)"),
    s("ה", 3, "end_ah", "Hebrew (f. ־ה), arc (־ה spelling)"),
    p("ד", 4, "pre_de", "Aramaic (relative/genitive ד־)"),
    p("וד", 5, "pre_de", "Aramaic (ו + ד־)"),
    p("ה", 4, "pre_ha", "Hebrew (article ה־), arc (haphel)"),
    p("וה", 5, "pre_ha", "Hebrew (ו + article)"),
    p("ש", 4, "pre_she", "hbo-x-mishnaic, he (relative ש־)"),
    p("וש", 5, "pre_she", "hbo-x-mishnaic, he (ו + ש־)"),
    p("כש", 5, "pre_kshe", "hbo-x-mishnaic, he (כש־ 'when')"),
    p("מש", 5, "pre_mishe", "hbo-x-mishnaic (מש־)"),
    p("את", 5, "pre_it", "Aramaic (ithpeel/ithpaal את־)"),
    p("אית", 6, "pre_it", "Aramaic (אית־)"),
    p("הת", 5, "pre_hit", "Hebrew (hithpael הת־), arc"),
    p("נת", 5, "pre_nit", "hbo-x-mishnaic (nithpael נת־)"),
    p("לי", 4, "pre_li", "tmr (l-imperfect לי־)"),
    p("וי", 4, "pre_wy", "hbo (wayyiqtol ויּ־)"),
    p("ות", 5, "pre_wt", "hbo (wayyiqtol ותּ־)"),
];

/// The cue table looked up by skeleton. Built once; a `BTreeMap` so iteration order is fixed.
pub fn word_cues() -> std::collections::BTreeMap<&'static str, &'static Cue> {
    let mut m = std::collections::BTreeMap::new();
    for q in WORDS {
        m.entry(q.form).or_insert(q);
    }
    m
}

/// The gloss of a feature name for explanations: cue features carry what the manual expects.
pub fn gloss(feature: &str) -> Option<&'static str> {
    if let Some(name) = feature.strip_prefix("C:") {
        return WORDS.iter().find(|q| q.name == name).map(|q| q.expect);
    }
    if let Some(name) = feature.strip_prefix("A:") {
        return AFFIXES.iter().find(|a| a.name == name).map(|a| a.expect);
    }
    match feature {
        "P:niqqud" => Some("vowel points"),
        "P:accent" => Some("cantillation accents (Biblical text)"),
        "P:wayyiqtol" => Some("pointed wayyiqtol וַ + dagesh (Biblical narrative)"),
        "P:abbr" => Some("abbreviation with gershayim (rabbinic, Modern)"),
        "P:geresh" => Some("geresh (abbreviation, rabbinic)"),
        _ => None,
    }
}
