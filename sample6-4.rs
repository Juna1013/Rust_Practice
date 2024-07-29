fn build_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: ture,
        sign_in_count: 1,
    }
}
