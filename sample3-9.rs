let s = String::from("hello");
let slice = &s[0..2];
let slice = &s[0..=2];
let slice = &s[..2];
let slice = &s[3..s.len()];
let slice = &s[3..];
let slice = &s[..];
