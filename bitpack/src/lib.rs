pub mod bitpack;
use crate::bitpack::*;
use rand::Rng;

// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// UNSIGNED
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn _check_laws_u(word: u64, w: u64, lsb: u64, value: u64, w2: u64, lsb2: u64){

    // newu should return a value
    assert!(
        bitpack::newu(word, w, lsb, value).is_some(),
        "newu Returned None\nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}\n",
        word, w, lsb, value, w2, lsb2
    );
    // packing a value into a word and then getting that value from the word should not change the value
    assert!(
        bitpack::getu(bitpack::newu(word, w, lsb, value).unwrap(), w, lsb) == value,
        "getu Returned Incorrect Value\nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
        \nWord after newu: {:b}
        \n{:b} != {:b}\n",
        word, w, lsb, value, w2, lsb2,
        bitpack::newu(word, w, lsb, value).unwrap(), 
        bitpack::getu(bitpack::newu(word, w, lsb, value).unwrap(), w, lsb), value
    );
    // if one field in the word does not overlap with another, then changing one field should 
    // not affect the other
    if lsb2 >= w + lsb {
        assert!(
            bitpack::getu(bitpack::newu(word, w, lsb, value).unwrap(), w2, lsb2) == getu(word, w2, lsb2),
            "getu Returned Incorrect Value for w2 and lsb2\nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
            \nWord after newu: {:b}
            \n{:b} != {:b}\n",
            word, w, lsb, value, w2, lsb2,
            bitpack::newu(word, w, lsb, value).unwrap(), 
            bitpack::getu(bitpack::newu(word, w, lsb, value).unwrap(), w2, lsb2), getu(word, w2, lsb2)
        );
    }
    // if two fields do not overlap eachother, if you change both fields, the order in 
    // which these fields are changed should not matter
    if lsb2 >= w + lsb || lsb >= w2 + lsb2 {
        let value2: u64 = 
        match w2 {
            0 => 0,
            _ => bitpack::safe_left_shift(1,w2-1)
        };
        assert!(
            bitpack::newu(bitpack::newu(word, w, lsb, value).unwrap(), w2, lsb2, value2).unwrap() == 
            bitpack::newu(bitpack::newu(word, w2, lsb2, value2).unwrap(), w, lsb, value).unwrap(), 
            "Order of newu on word on non-overlapping bits should not matter\nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
            \n{:b} != {:b}\n",
            word, w, lsb, value, w2, lsb2,
            bitpack::newu(bitpack::newu(word, w, lsb, value).unwrap(), w2, lsb2, value2).unwrap(),
            bitpack::newu(bitpack::newu(word, w2, lsb2, value2).unwrap(), w, lsb, value).unwrap()
        );
    }
    // if you insert a new field (w, lsb) and another new field (w', lsb'), and if the 
    // field (w, lsb) is entirely contained within the field (w0, lsb0), then the result 
    // is the same is if you had inserted only(w', lsb').
    if lsb2 > lsb && lsb2 + w2 < lsb + w {
        let value2: u64 = 
        match w2 {
            0 => 0,
            _ => bitpack::safe_left_shift(1,w2-1)
        };
        assert!(
            bitpack::newu(bitpack::newu(word,w2,lsb2,value2).unwrap(),w,lsb,value).unwrap() == 
            bitpack::newu(word,w,lsb,value).unwrap(),
            "Adding field that overlaps another new field should be the same as just adding the bigger field
            \nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
            \n{:b} != {:b}\n",
            word, w, lsb, value, w2, lsb2,
            bitpack::newu(bitpack::newu(word,w2,lsb2,value2).unwrap(),w,lsb,value).unwrap(),
            bitpack::newu(word,w,lsb,value).unwrap()
        );
    }

    // if you insert a field f, the bits fh that are above (more significant than) the 
    // inserted field are unchanged.
    assert!(
        bitpack::getu(bitpack::newu(word,w,lsb,value).unwrap(),64 - (lsb + w), lsb + w) == 
        bitpack::getu(word,64 - (lsb + w), lsb + w), 
        "Inserting field should not affect more significant bits
        \nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
        \n{:b} != {:b}\n",
        word, w, lsb, value, w2, lsb2,
        bitpack::getu(bitpack::newu(word,w,lsb,value).unwrap(),64 - (lsb + w), lsb + w), 
        bitpack::getu(word,64 - (lsb + w), lsb + w) 
    );

    // if you insert a field f, the bits fl that are
    // below (less significant than) the inserted field are unchanged.
    assert!(
        bitpack::getu(bitpack::newu(word,w,lsb,value).unwrap(),lsb, 0) == 
        bitpack::getu(word,lsb, 0), 
        "Inserting field should not affect less significant bits
        \nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
        \n{:b} != {:b}\n",
        word, w, lsb, value, w2, lsb2,
        bitpack::getu(bitpack::newu(word,w,lsb,value).unwrap(),lsb, 0), 
        bitpack::getu(word,lsb, 0) 
    );

    // A value left shifted twice should fit in a width that is 2 greater, 
    // and a value right shifted twice should fit in a width that is 2 less
    assert!(
        bitpack::fitsu(value,w) == bitpack::fitsu(bitpack::safe_left_shift(value, 2), w + 2),
        "Value left shifted x times should fit in (width + x) bits\
        \ninitial value: {:b}\ninitial w: {}\nshifted value: {:b}\nnew width: {}\n",
        value, w, bitpack::safe_left_shift(value, 2), w + 2
    );
    // w should never be negative anyways (unsigned int)
    if w as i64 - 2 >= 0 {
        assert!(
            bitpack::fitsu(value,w) == bitpack::fitsu(bitpack::safe_right_shift(value, 2), w - 2),
            "Value right shifted x times should fit in (width - x) bits\
            \ninitial value: {:b}\ninitial w: {}\nshifted value: {:b}\nnew width: {}\n",
            value, w, bitpack::safe_right_shift(value, 2), w - 2
        );
    }

}


#[test]
fn check_fitsu(){
    // Test if width >= 64 always returns true
    assert!(bitpack::fitsu(u64::MAX, 64));
    assert!(bitpack::fitsu(0, 64));
    assert!(bitpack::fitsu(u64::MAX, 100));

    // Test basic cases for different widths
    assert!(bitpack::fitsu(0, 0)); // 0 fits in 0 bits
    assert!(!bitpack::fitsu(1, 0)); // 1 does not fit in 0 bits
    assert!(bitpack::fitsu(1, 1)); // 1 fits in 1 bit
    assert!(bitpack::fitsu(2, 2)); // 2 fits in 2 bits
    assert!(!bitpack::fitsu(4, 2)); // 4 does not fit in 2 bits
    assert!(bitpack::fitsu(4, 3)); // 4 fits in 3 bits

    // Test edge cases
    assert!(bitpack::fitsu(u64::MAX, 64));
    assert!(!bitpack::fitsu(u64::MAX, 63));

    // make sure newu returns None if value does not fit.
    assert!(
        !(bitpack::newu(666_u64, 8, 0, 256).is_some()), "newu should return None if value does not fit in width
        value: 256
        width: 8\n"
    );
    // returns true if it does fit
    assert!(
        bitpack::newu(666_u64, 8, 0, 255).is_some(), "newu should return a word if value fits in width
        value: 255
        width: 8\n"
    );
}



#[test]
fn random_unsigned(){
    let mut rng = rand::thread_rng();
    for w in 0..65 { // upper value is excluded
        for lsb in 0..(65-w) {
            for _trial in 0..1001 {
                // set other parameter values randomly
                let word: u64 = rng.gen::<u64>();
                let value: u64 = match w {
                    0 => 0,
                    64 => rng.gen_range(0..std::u64::MAX),
                    _ => rng.gen_range(0..bitpack::safe_left_shift(1, w)),
                };
                let w2: u64 = rng.gen_range(0..65);
                let lsb2: u64 = rng.gen_range(0..(65 - w2));
                _check_laws_u(word, w, lsb, value, w2, lsb2);
            }
        }
    }
}



// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// SIGNED
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

fn _check_laws_s(word: u64, w: u64, lsb: u64, value: i64, w2: u64, lsb2: u64){

    // news should return a value
    assert!(
        bitpack::news(word, w, lsb, value).is_some(),
        "news Returned None\nword: {:b}\nw: {}\nlsb: {}\nvalue: {}\nw2: {}\nlsb2: {}\n",
        word, w, lsb, value, w2, lsb2
    );
    // packing a value into a word and then getting that value from the word should not change the value
    assert!(
        bitpack::gets(bitpack::news(word, w, lsb, value).unwrap(), w, lsb) == value,
        "gets Returned Incorrect Value\nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
        \nWord after news: {:b}
        \n{:b} != {:b}\n",
        word, w, lsb, value, w2, lsb2,
        bitpack::news(word, w, lsb, value).unwrap(), 
        bitpack::gets(bitpack::news(word, w, lsb, value).unwrap(), w, lsb), value
    );
    // if one field in the word does not overlap with another, then changing one field should 
    // not affect the other
    if lsb2 >= w + lsb {
        assert!(
            bitpack::gets(bitpack::news(word, w, lsb, value).unwrap(), w2, lsb2) == bitpack::gets(word, w2, lsb2),
            "gets Returned Incorrect Value for w2 and lsb2\nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
            \nWord after news: {:b}
            \n{:b} != {:b}\n",
            word, w, lsb, value, w2, lsb2,
            bitpack::news(word, w, lsb, value).unwrap(), 
            bitpack::gets(bitpack::news(word, w, lsb, value).unwrap(), w2, lsb2), bitpack::gets(word, w2, lsb2)
        );
    }
    // if two fields do not overlap eachother, if you change both fields, the order in 
    // which these fields are changed should not matter
    if lsb2 >= w + lsb || lsb >= w2 + lsb2 {
        let value2: i64 = 
        match w2 {
            0 => 0,
            1 => 0,
            _ => bitpack::safe_left_shift(1,w2-2) as i64
        };
        assert!(
            bitpack::news(bitpack::news(word, w, lsb, value).unwrap(), w2, lsb2, value2).unwrap() == 
            bitpack::news(bitpack::news(word, w2, lsb2, value2).unwrap(), w, lsb, value).unwrap(), 
            "Order of news on word on non-overlapping bits should not matter\nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
            \n{:b} != {:b}\n",
            word, w, lsb, value, w2, lsb2,
            bitpack::news(bitpack::news(word, w, lsb, value).unwrap(), w2, lsb2, value2).unwrap(),
            bitpack::news(bitpack::news(word, w2, lsb2, value2).unwrap(), w, lsb, value).unwrap()
        );
    }
    // if you insert a new field (w, lsb) and another new field (w', lsb'), and if the 
    // field (w, lsb) is entirely contained within the field (w0, lsb0), then the result 
    // is the same is if you had inserted only(w', lsb').
    if lsb2 > lsb && lsb2 + w2 < lsb + w {
        let value2: i64 = 
        match w2 {
            0 => 0,
            1 => 0,
            _ => bitpack::safe_left_shift(1,w2-2) as i64
        };
        assert!(
            bitpack::news(bitpack::news(word,w2,lsb2,value2).unwrap(),w,lsb,value).unwrap() == 
            bitpack::news(word,w,lsb,value).unwrap(),
            "Adding field that overlaps another new field should be the same as just adding the bigger field
            \nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
            \n{:b} != {:b}\n",
            word, w, lsb, value, w2, lsb2,
            bitpack::news(bitpack::news(word,w2,lsb2,value2).unwrap(),w,lsb,value).unwrap(),
            bitpack::news(word,w,lsb,value).unwrap()
        );
    }

    // if you insert a field f, the bits fh that are above (more significant than) the 
    // inserted field are unchanged.
    assert!(
        bitpack::gets(bitpack::news(word,w,lsb,value).unwrap(),64 - (lsb + w), lsb + w) == 
        bitpack::gets(word,64 - (lsb + w), lsb + w), 
        "Inserting field should not affect more significant bits
        \nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
        \n{:b} != {:b}\n",
        word, w, lsb, value, w2, lsb2,
        bitpack::gets(bitpack::news(word,w,lsb,value).unwrap(),64 - (lsb + w), lsb + w), 
        bitpack::gets(word,64 - (lsb + w), lsb + w) 
    );

    // if you insert a field f, the bits fl that are
    // below (less significant than) the inserted field are unchanged.
    assert!(
        bitpack::gets(bitpack::news(word,w,lsb,value).unwrap(),lsb, 0) == 
        bitpack::gets(word,lsb, 0), 
        "Inserting field should not affect less significant bits
        \nword: {:b}\nw: {}\nlsb: {}\nvalue: {:b}\nw2: {}\nlsb2: {}
        \n{:b} != {:b}\n",
        word, w, lsb, value, w2, lsb2,
        bitpack::gets(bitpack::news(word,w,lsb,value).unwrap(),lsb, 0), 
        bitpack::gets(word,lsb, 0) 
    );

    // A value left shifted twice should fit in a width that is 2 greater, 
    // and a value right shifted twice should fit in a width that is 2 less
    assert!(
        bitpack::fitss(value,w) == bitpack::fitss(bitpack::safe_left_shift(value as u64, 2) as i64, w + 2),
        "Value left shifted x times should fit in (width + x) bits\
        \ninitial value: {:b}\ninitial w: {}\nshifted value: {:b}\nnew width: {}\n",
        value, w, bitpack::safe_left_shift(value as u64, 2) as i64, w + 2
    );
    // w should never be negative anyways (unsigned int)
    if w as i64 - 2 >= 0 {
        assert!(
            bitpack::fitss(value,w) == bitpack::fitss(bitpack::safe_right_shift_s(value, 2), w - 2),
            "Value right shifted x times should fit in (width - x) bits\
            \ninitial value: {:b}\ninitial w: {}\nshifted value: {:b}\nnew width: {}\n",
            value, w, bitpack::safe_right_shift(value as u64, 2) as i64, w - 2
        );
    }

}


#[test]
fn check_fitss(){
    // Test if width >= 64 always returns true
    assert!(bitpack::fitss(i64::MAX, 64));
    assert!(bitpack::fitss(0, 64));
    assert!(bitpack::fitss(i64::MAX, 100));

    // Test basic cases for different widths
    assert!(bitpack::fitss(0, 0)); // 0 fits in 0 bits
    assert!(!bitpack::fitss(1, 0)); // 1 does not fit in 0 bits
    assert!(!bitpack::fitss(1, 1)); // 1 does not fit in 1 bit
    assert!(!bitpack::fitss(4, 2)); // 4 does not fit in 2 bits
    assert!(!bitpack::fitss(4, 3)); // 4 does not fit in 3 bits

    assert!(!bitpack::fitss(1, 0)); // 1 does not fit in 0 bits
    assert!(bitpack::fitss(-1, 1)); // -1 fits in 1 bit
    assert!(bitpack::fitss(1, 2)); // 1 fits in 2 bits




    // Test edge cases
    assert!(bitpack::fitss(i64::MAX, 63));
    assert!(!bitpack::fitss(i64::MAX, 62), "i64-max: {:b}",i64::MAX);

    // make sure news returns None if value does not fit.
    assert!(
        !(bitpack::news(666_u64, 8, 0, 128).is_some()), "news should return None if value does not fit in width
        value: 256
        width: 8\n"
    );
    // returns true if it does fit
    assert!(
        bitpack::news(666_u64, 8, 0, 127).is_some(), "news should return a word if value fits in width
        value: 255
        width: 8\n"
    );
}

#[test]
fn random_signed(){
    let mut rng = rand::thread_rng();
    for w in 0..65 { // upper value is excluded
        for lsb in 0..(65-w) {
            for _trial in 0..1001 {
                // set other parameter values randomly
                let word: u64 = rng.gen::<u64>();
                let value: i64 = match w {
                    0 => 0,
                    1 => rng.gen_range(-1..0),
                    64 => rng.gen_range(std::i64::MIN..std::i64::MAX),
                    _ => rng.gen_range(-1*(bitpack::safe_left_shift(1,w-1) as i64)..bitpack::safe_left_shift(1, w-1) as i64),
                };
                let w2: u64 = rng.gen_range(0..65);
                let lsb2: u64 = rng.gen_range(0..(65 - w2));
                _check_laws_s(word, w, lsb, value, w2, lsb2);
            }
        }
    }
}


// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
// SAFE SHIFTS
// TESTS
// ~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~

#[test]
fn check_safe_shifts(){
    // a left and right shift by 0 should do nothing
    assert!(bitpack::safe_left_shift(6_u64, 0) == 6_u64, "left shift by 0 should not change the value");
    assert!(bitpack::safe_right_shift(6_u64, 0) == 6_u64, "right shift by 0 should not change the value");

    // a left shift of 1 should double the value
    assert!(bitpack::safe_left_shift(6_u64,1) == 12_u64, "left shift of 1 should double the value");

    // a right shift of 1 should halve the value
    assert!(bitpack::safe_right_shift(6_u64,1) == 3_u64, "right shift of 1 should double the value");

    // left shift or right shift by 64 or greater should return 0
    assert!(bitpack::safe_left_shift(6_u64, 64) == 0, "left shift of 64 should return 0");
    assert!(bitpack::safe_left_shift(6_u64, 100) == 0, "left shift of greater than 64 should return 0");
    assert!(bitpack::safe_right_shift(6_u64, 64) == 0, "right shift of 64 should return 0");
    assert!(bitpack::safe_right_shift(6_u64, 100) == 0, "right shift of greater than 64 should return 0");
}

