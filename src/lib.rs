/*
FERRIMOJI by Alexander Abraham.
Licensed under the MIT license.
*/

use std::collections::HashMap;

/// Returns a map that returns a KV pair of
/// curated UTF-8 emojis.
pub fn unicode_pool() -> HashMap<String, String> {
    let mut result: HashMap<String,String> = HashMap::new();
    result.insert(String::from("grinningFace"), String::from("\u{1f600}"));
    result.insert(String::from("laughCry"), String::from("\u{1f602}"));
    result.insert(String::from("upsideDownFace"), String::from("\u{1f643}"));
    result.insert(String::from("heartsInFace"), String::from("\u{1f970}"));
    result.insert(String::from("smilingFace"), String::from("\u{1f60a}"));
    result.insert(String::from("haloFace"), String::from("\u{1f607}"));
    result.insert(String::from("winkingFace"), String::from("\u{1f609}"));
    result.insert(String::from("kissingFace"), String::from("\u{1f61a}"));
    result.insert(String::from("angryFace"), String::from("\u{1f620}"));
    result.insert(String::from("crazyFace"), String::from("\u{1f92a}"));
    result.insert(String::from("thinkingFace"), String::from("\u{1f914}"));
    result.insert(String::from("naughtyface"), String::from("\u{1f608}"));
    result.insert(String::from("seeNoEvil"), String::from("\u{1f648}"));
    result.insert(String::from("animalGorilla"), String::from("\u{1f98d}"));
    result.insert(String::from("windCloud"), String::from("\u{1f4A8}"));
    result.insert(String::from("sweatDroplets"), String::from("\u{1f4A6}"));
    result.insert(String::from("dogFace"), String::from("\u{1f436}"));
    result.insert(String::from("unicornHead"), String::from("\u{1f984}"));
    result.insert(String::from("catFace"), String::from("\u{1f431}"));
    result.insert(String::from("horseHead"), String::from("\u{1f434}"));
    result.insert(String::from("pandaFace"), String::from("\u{1f43c}"));
    result.insert(String::from("bearFace"), String::from("\u{1f43b}"));
    result.insert(String::from("tigerFace"), String::from("\u{1f42f}"));
    result.insert(String::from("redHeart"), String::from("\u{2764}"));
    result.insert(String::from("blackHeart"), String::from("\u{1f5a4}"));
    result.insert(String::from("backArrow"), String::from("\u{1f519}"));
    result.insert(String::from("soonArrow"), String::from("\u{1f51c}"));
    result.insert(String::from("topArrow"), String::from("\u{1f51d}"));
    result.insert(String::from("infinitySign"), String::from("\u{267e}"));
    result.insert(String::from("multiplySign"), String::from("\u{2716}"));
    result.insert(String::from("plusSign"), String::from("\u{2795}"));
    result.insert(String::from("divideSign"), String::from("\u{2797}"));
    result.insert(String::from("minusSign"), String::from("\u{2796}"));
    result.insert(String::from("tickSign"), String::from("\u{2714}"));
    result.insert(String::from("kitchenKnife"), String::from("\u{1f52a}"));
    result.insert(String::from("scissors"), String::from("\u{2702}"));
    result.insert(String::from("bomb"), String::from("\u{1f4a3}"));
    result.insert(String::from("gun"), String::from("\u{1f52b}"));
    result.insert(String::from("hammer"), String::from("\u{1f528}"));
    result.insert(String::from("wrench"), String::from("\u{1f527}"));
    result.insert(String::from("hammerAndWrench"), String::from("\u{1f6e0}"));
    result.insert(String::from("hammerAndPick"), String::from("\u{2692}"));
    result.insert(String::from("pick"), String::from("\u{26cf}"));
    result.insert(String::from("woodSaw"), String::from("\u{1fa9a}"));
    result.insert(String::from("axe"), String::from("\u{1fa93}"));
    result.insert(String::from("prideFlag"), String::from("\u{1f3f3}"));
    result.insert(String::from("transFlag"), String::from("\u{26a7}"));
    result.insert(String::from("rainbow"), String::from("\u{1f308}"));
    result.insert(String::from("peopleKissing"), String::from("\u{1f48f}"));
    result.insert(String::from("party"), String::from("\u{1f389}"));
    result.insert(String::from("coupleHeart"), String::from("\u{1f491}"));
    result.insert(String::from("sparkles"), String::from("\u{2728}"));
    result.insert(String::from("confetti"), String::from("\u{1f38a}"));
    result.insert(String::from("maleBunnies"), String::from("\u{1f46f}"));
    result.insert(String::from("kiss"), String::from("\u{1f48b}"));
    result.insert(String::from("yarn"), String::from("\u{1f9f6}"));
    result.insert(String::from("headPhones"), String::from("\u{1f3a7}"));
    result.insert(String::from("piano"), String::from("\u{1f3b9}"));
    result.insert(String::from("bowAndArrow"), String::from("\u{1f3f9}"));
    result.insert(String::from("paintBrush"), String::from("\u{1f58c}"));
    result.insert(String::from("paintPalette"), String::from("\u{1f3a8}"));
    result.insert(String::from("sewingNeedle"), String::from("\u{1faa1}"));
    result.insert(String::from("threadRoll"), String::from("\u{1f95f}"));
    result.insert(String::from("syringe"), String::from("\u{1f489}"));
    result.insert(String::from("pill"), String::from("\u{1f48a}"));
    return result;
}

/// Retrieves an emoji from the unicode pool
/// and returns an empty string if it cannots
/// be retrieved from the pool.
pub fn get_emoji(name: String) -> String {
    let mut result: String = String::from("");
    let name_clone_one: String = name.clone();
    let name_clone_two: String = name_clone_one.clone();
    if unicode_pool().contains_key(&name_clone_one) {
        result = unicode_pool()[&name_clone_two].clone();
    }
    else {}
    return result;
}

/// Tests all method present.
/// Iterates through the UTF-8 emoji
/// pool and attempts to print them to the
/// console.
pub fn tests(){
    for (key, value) in unicode_pool().clone().into_iter() {
        let msg: String = value.clone();
        if msg.len() == 0 {
            println!("{}", String::from("Emoji 404."))
        }
        else {
            println!("{}", msg);
        }
    }
}
