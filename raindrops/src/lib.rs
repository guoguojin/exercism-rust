pub fn raindrops(n: u32) -> String {
    // closure to check if the value passed is a factor of n
    let is_factor = |f| n % f == 0;

    // we will build the output string by checking for each of the factors that should be interpretted
    let mut output = String::from("");

    if is_factor(3) { output.push_str("Pling"); }
    if is_factor(5) { output.push_str("Plang"); }
    if is_factor(7) { output.push_str("Plong"); }

    // if the number doesn't have any of the factors we are looking for, we should output the number
    if output.len() == 0 {
        n.to_string()
    } else {
        output
    }
}

pub fn raindrops_functional(n: u32) -> String {
    let sounds = [(3, "Pling"), (5, "Plang"), (7, "Plong")];
    let sound = sounds.into_iter()
        .filter(|&(f, _)| n % f == 0)                       // here we use a closure to filter out the sounds that contain a factor
        .map(|&(_, s)| s)                                   // we then use a map to take the sound associated with the factor
        .fold(String::new(), |acc, sound| acc + sound);     // fold allows us to accumulate the results of the map into a single string
    
    if sound.is_empty() {
        n.to_string()
    } else {
        sound
    }
}
