pub const FOUR: [&str; 33] = [
    "AQUA", "BANK", "CASH", "DOCK", "ERGO", "FOWL", "GOLF", "HACK", "IBEX", "JAZZ", "KNOW", "LAZY",
    "MACE", "NAAN", "OAKS", "PACK", "QUIZ", "RACE", "SAFE", "TORT", "UGLY", "VAIN", "WAVE", "XYST",
    "YELP", "ZEAL", "TREE", "BATH", "DUCK", "KICK", "RISE", "RAZE", "OVER",
];

pub const FIVE: [&str; 60] = [
    "ABOVE", "ADOBE", "ARISE", "BREAK", "BRICK", "CATCH", "COUGH", "DOUGH", "DIVER", "EAGER",
    "ELITE", "FREAK", "FOUND", "GRAPE", "GREAT", "HELLO", "HELPS", "IGLOO", "ICILY", "JELLY",
    "JIVES", "KELLY", "KHAKI", "LOCKS", "LOAMY", "MARCH", "MAIZE", "MAPLE", "NOBLY", "NUKES",
    "OZONE", "OXBOW", "PROXY", "PLUCK", "QUARK", "QUEST", "RIGHT", "RAZOR", "SOUND", "SCUFF",
    "TONIC", "TOWER", "TAWNY", "UNDER", "UDDER", "VIXEN", "VOMIT", "WATER", "WINCH", "XYLEM",
    "XERIC", "ZEBRA", "YAWNS", "YIKES", "ZILCH", "ZESTS", "GREEN", "RAISE", "DOVER", "ROVER",
];

pub const SIX: [&str; 55] = [
    "ANYWAY", "APATHY", "BLAZED", "BREEZE", "CACTUS", "CHEQUE", "DONKEY", "DOZENS", "EXPEND",
    "EXPAND", "EXCESS", "FIZZED", "FUZZED", "FLEXES", "EQUINE", "EVOKED", "EXPORT", "GIZMOS",
    "GRAPES", "GRADES", "HAZILY", "HAZARD", "HUMBLY", "JINKED", "JOKING", "INJURY", "IMPACT",
    "LIZARD", "WIZARD", "LIQUID", "LANKLY", "LACKED", "MIXING", "MYTHIC", "NICELY", "NOBODY",
    "OXFORD", "OFFEND", "RAZING", "REFLEX", "SUFFIX", "SHABBY", "PREFIX", "TEAZED", "TWEEZE",
    "UNPACK", "UNJUST", "VAMPED", "VERIFY", "WHEEZY", "XYLOID", "YOWLED", "YELLOW", "ZYGOTE",
    "ZIGZAG",
];

pub const SEVEN: [&str; 11] = [
    "CATCHER", "FRANKLY", "OUTCOME", "ZIPPERS", "ADJUNCT", "BUZZARD", "BUZZING", "DRIZZLY",
    "EXACTLY", "HAMMOCK", "HICCUPS",
];

#[cfg(test)]
mod test {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn validate_words() {
        check_words(&FOUR, 4);
        check_words(&FIVE, 5);
        check_words(&SIX, 6);
        check_words(&SEVEN, 7);
    }

    fn check_words(words: &[&str], expected: usize) {
        check_len(words, expected);
        check_unique(words);
        check_caps(words);
    }

    fn check_caps(words: &[&str]) {
        let mut invalid = vec![];
        for word in words {
            if word.chars().any(|c| !c.is_ascii_uppercase()) {
                invalid.push(word.to_string());
            }
        }
        if !invalid.is_empty() {
            panic!("Invalid cap words {invalid:?}")
        }
    }

    fn check_len(words: &[&str], expected: usize) {
        let mut invalid = vec![];
        for word in words {
            if word.len() != expected {
                invalid.push(word);
            }
        }
        if !invalid.is_empty() {
            panic!("Invalid {expected:?} len words {invalid:?}")
        }
    }

    fn check_unique(words: &[&str]) {
        let mut set = HashSet::new();
        for word in words {
            set.insert(word);
        }
        assert_eq!(set.len(), words.len());
    }
}
