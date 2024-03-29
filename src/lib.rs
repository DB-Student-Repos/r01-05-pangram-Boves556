/// Determine whether a sentence is a pangram.
pub fn is_pangram(sentence: &str) -> bool {
    let mut seen = [false; 26]; // Array to track if each letter is seen

    for c in sentence.chars() {
        // Convert character to lowercase and subtract 'a' to get index
        if let Some(idx) = c.to_lowercase().next().map(|c| c as usize - 'a' as usize) {
            if idx < 26 {
                seen[idx] = true; // Mark letter as seen
            }
        }
    }

    seen.iter().all(|&b| b) // Check if all letters were seen
}
