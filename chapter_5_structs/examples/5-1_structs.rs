struct user {
	active: bool,
	username: String,
	email: String,
	sign_in_count: u64
}

fn main() {
	let user1 = User {
		email: String::from("someone@example.com"),
		username:  String::from("sourcename123"),
		active: true,
		sign_in_count: 1,
	};

}
