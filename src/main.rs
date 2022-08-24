struct Post {
	title: String,
	author: String,
	date_time: String,
	content: String,
	tag: Tag,
	reviews: Review	
}

struct Review {
	author: String,
	date_time: String,
	content: String,
	rating: f32
}

enum Tag {
	Holiday,
	Programming,
	Business,
	Projects,
	Other
}

impl Post {
	fn post(self) {
		println!("{}", self.reviews.author);
	}
}

fn main() {
   let my_post_a = Post {
	   	title: "hello".to_string(),
	   	author: "bye".to_string(),
	   	date_time: "sigh".to_string(),
	   	content: "shy".to_string(),
	   	tag: Tag::Holiday,
	   	reviews: Review {
		   		author: "uwu".to_string(),
		   		date_time: "owo".to_string(),
		   		content: "hate it!".to_string(),
		   		rating: 100.0,
	   	},
   	};

   	my_post_a.post();

}
