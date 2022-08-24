pub fn edit_distance(source: &str, target: &str) -> usize {
    let d = edit_distance(source, target);
    d
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
	fn test_distance() {
		assert_eq!(edit_distance("alignment", "assignment"), 2);
		assert_eq!(edit_distance("gumbo", "gambol"), 2);
		assert_eq!(edit_distance("kitten", "sitting"), 3);
		assert_eq!(edit_distance("rosettacode", "raisethysword"), 8);
	}
}
