pub mod post {
    use std::{
        fmt::{self, Display},
        ops::Add,
    };

    trait Summary {
        fn summarize(&self) -> String;
    }

    struct Post {
        title: String,
        author: String,
        content: String,
    }

    impl Post {
        fn new(title: &str, author: &str, content: &str) -> Post {
            Post {
                title: title.to_string(),
                author: author.to_string(),
                content: content.to_string(),
            }
        }
    }

    impl Summary for Post {
        fn summarize(&self) -> String {
            format!("Post {}, with author{}", self.title, self.author)
        }
    }

    fn show_summary(item: &impl Summary) {
        println!("{}", item.summarize());
    }

    impl Add for Post {
        type Output = Post;
        fn add(self, p: Self) -> Self::Output {
            Post {
                author: format!("{} {}", self.author, p.author),
                content: format!("{} {}", self.content, p.content),
                title: format!("{} {}", self.title, p.title),
            }
        }
    }

    fn add_posts(p1: Post, p2: Post) -> Post {
        p1 + p2
    }
    impl Display for Post {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Post <{}> by '{}'", self.title, self.author)
        }
    }

    pub fn test() {
        let post1 = Post {
            title: "Hello".to_string(),
            content: "CONTENT".to_string(),
            author: "ME".to_string(),
        };
        let post2 = Post::new("World", " Yes", "and HIM");

        let post3 = add_posts(post1, post2);
        let r = post3.summarize();
        println!("{r}");
        show_summary(&post3);
        println!("{post3}");
    }
}
