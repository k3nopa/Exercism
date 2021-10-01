// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut magazine_dict: HashMap<&str, u32> = HashMap::new();
    let mut note_dict: HashMap<&str, u32> = HashMap::new();
    // Build the HashMap with magazine words
    for x in magazine.iter() {
        let val = magazine_dict.entry(x).or_insert(1);
        *val += 1;
    }
    // Build the HashMap with note words
    for x in note.iter() {
        let val = note_dict.entry(x).or_insert(1);
        *val += 1;
    }
    // Compare both HashMap
    // magazine need to have note's words
    for key in note.iter() {
        let x = *note_dict.get(key).unwrap_or(&0);
        let y = *magazine_dict.get(key).unwrap_or(&0);

        if y < x {
            return false;
        } else {
            continue;
        }
    }
    true
}
