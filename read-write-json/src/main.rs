use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct Paragraph {
    name: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct Article {
    article: String,
    author: String,
    paraph: Vec<Paragraph>,
}
fn main() {
    let json = r#"
  {
    "article": "article",
    "author": "author",
    "paraph": [
      {
        "name": "mulayam"
      },
       {
        "name": "test"
      }
    ]
  }"#;
    let parsed_article: Article = serde_json::from_str(json).unwrap();

    let str_article: Article = Article {
        article: String::from("article ji"),
        author: String::from("author ji"),
        paraph: vec![Paragraph {
            name: String::from("pargraoph ji"),
        }],
    };
    println!("{:#?}", parsed_article);

    let json = serde_json::to_string(&str_article).unwrap();

    println!("{:#?}", json);
}
