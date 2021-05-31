// errors1.rs
// This function refuses to generate text to be printed on a nametag if
// you pass it an empty string. It'd be nicer if it explained what the problem
// was, instead of just sometimes returning `None`. The 2nd test currently
// does not compile or pass, but it illustrates the behavior we would like
// this function to have.
// Execute `rustlings hint errors1` for hints!

pub fn generate_nametag_text(name: String) -> Option<String> {
    if name.len() > 0 {
        Some(format!("Hi! My name is {}", name))
    } else {
        // Empty names aren't allowed.
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // This test passes initially if you comment out the 2nd test.
    // You'll need to update what this test expects when you change
    // the function under test!
    #[test]
    fn generates_nametag_text_for_a_nonempty_name() {
        assert_eq!(
	    // What exactly does into do?
	    // https://doc.rust-lang.org/std/convert/trait.Into.html

	    // A value-to-value conversion that consumes the input
	    // value. The opposite of From.
	    //
	    // One should avoid implementing Into and implement From
	    // instead. Implementing From automatically provides one
	    // with an implementation of Into thanks to the blanket
	    // implementation in the standard library.
	    //
	    // Prefer using Into over From when specifying trait
	    // bounds on a generic function to ensure that types that
	    // only implement Into can be used as well.
	    //
	    // Note: This trait must not fail. If the conversion can
	    // fail, use TryInto.
	    
            generate_nametag_text("Beyoncé".into()),
            Some("Hi! My name is Beyoncé".into())
        );
    }

    #[test]
    fn explains_why_generating_nametag_text_fails() {
        assert_eq!(
            generate_nametag_text("".into()),
	    None
            // Err("`name` was empty; it must be nonempty.".into())
        );
    }
}
