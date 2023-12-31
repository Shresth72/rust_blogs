use aws_sdk_dynamodb::Client;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_config::Config;
use classes::model::{post::Post, blog::{Blog, NewBlog}};
use log::error;
use std::collections::HashMap;

pub struct DDBRepository {
    client: Client,
    table_name: String,
}

#[derive(Debug)]
pub struct DDBError;

fn required_item_value(key: &str, item: &HashMap<String, AttributeValue>) -> Result<String, DDBError> {
    match item_value(key, item) {
        Ok(Some(value)) => Ok(value),
        Ok(None) => {
            error!("Missing required item value: {}", key);
            Err(DDBError)
        },
        Err(DDBError) => Err(DDBError),
    }
}

fn item_value(key: &str, item: &HashMap<String, AttributeValue>) -> Result<Option<String>, DDBError> {
    match item.get(key) {
        Some(value) => match value.as_s() {
            Some(value) => Ok(Some(value.to_string())),
            None => {
                error!("Item value is not a string: {}", key);
                Err(DDBError)
            },
        },
        None => Ok(None),
    }
}

fn items_to_blog(items: Vec<HashMap<String, AttributeValue>>) -> Result<Blog, DDBError> {
    let mut blog = Blog {
        title: None,
        about: None,
        subtitle: None,
        posts: vec![],
    };

    for item in items {
        let sK = required_item_value("sK", &item)?;
        match sK.as_str() {
            "meta" => {
                blog.title = item_value("title", &item)?;
                blog.about = item_value("about", &item)?;
                blog.subtitle = item_value("subtitle", &item)?;
            },
            _ => {
                let title = item_value("title", &item)?;
                blog.posts.push(Post {
                    blog_id: required_item_value("pK", &item)?,
                    post_id: sK,
                    author: required_item_value("author", &item)?,
                    title,
                    content: required_item_value("content", &item)?,
                });
            }
        }
    };
    Ok(blog)
}

impl DDBRepository {
    pub fn init(table_name: String, config: Config) -> DDBRepository {
        let client = Client::new(&config);
        DDBRepository {
            client,
            table_name, 
        }
    }

    pub async fn put_post(&self, post: Post) -> Result<(), DDBError> {
        let mut request = self.client.put_item();
            .table_name(self.table_name.clone())
            .item("pK", AttributeValue::S(String::from(&post.blog_id)))
            .item("sK", AttributeValue::S(String::from(&post.post_id)))
            .item("author", AttributeValue::S(String::from(&post.author)))
            .item("author", AttributeValue::S(String::from(&post.author)));

        if let Some(title) = post.title {
            request = request.item("title", AttributeValue::S(title));
        }

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => {
                error!("Failed to put post: {:?}", post);
                Err(DDBError)
            },
        }
    }

    pub async fn put_blog(&self, blog: NewBlog) -> Result<(), DDBError> {
        let request = self.client.put_item()
            .table_name(self.table_name.clone())
            .item("pK", AttributeValue::S(String::from(&blog.blog_id)))
            .item("sK", AttributeValue::S(String::from("meta")))
            .item("title", AttributeValue::S(String::from(&blog.title.clone().unwrap())))
            .item("about", AttributeValue::S(String::from(&blog.about.clone().unwrap())));

        match request.send().await {
            Ok(_) => Ok(()),
            Err(_) => {
                error!("Failed to put blog: {:?}", blog);
                Err(DDBError)
            },
        }
    }

    pub async fn get_blog(
        &self,
        blog_id: String,
        oldest: Option<String>,
        newest: Option<String>,
    ) -> Result<(), DDBError> {
        let mut res = self.client
            .query()
            .table_name(self.table_name.clone())
            .expresion_attribute_names("#blog_id", "pK")
            .expresion_attribute_values(":blog_id", AttributeValue::S(blog_id))
            .key_condition_expression("#blog_id = :blog_id");

        if oldesr.is_some() || newest.is_some() {
            res = res.expresion_attribute_names("#post_id", "sK")
        }

        if let Some(oldest) = oldest {
            res = res.expresion_attribute_values(":oldest", AttributeValue::S(oldest));
                .key_condition_expression("#post_id > :oldest");
        }

        if let Some(newest) = newest {
            res = res.expresion_attribute_values(":newest", AttributeValue::S(newest));
                .key_condition_expression("#post_id < :newest");
        }

        match res.send().await {
            Ok(output) => {
                let items = res.items.ok_or(DDBError)?;
                let blog = items_to_blog(items)?;
                Ok(blog)
            },
            Err(_) => {
                error!("Failed to get blog: {}", blog_id);
                Err(DDBError)
            },
        }
    }
}

