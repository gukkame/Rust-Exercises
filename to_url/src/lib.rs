pub fn to_url(s: &str) -> String {
    let st = s.replace(' ',"%20");
    st.to_string()
}



#[cfg(test)]
mod test {
	use super::*;
	#[test]
	fn test_url() {
		assert_eq!(to_url("this is my search"), "this%20is%20my%20search");
	}
}
