#![allow(dead_code)]
use std::marker::PhantomData;
use std::string::String;

/// Сущности
struct User {
    user_id: u64,
    full_name: String,
    email: String,
}

struct Post<S> {
    post_id: u64,
    user: User,
    title: String,
    body: String,
    state: PhantomData<S>,
}

/// Состояния
struct New;
struct Unmoderated;
struct Published;
struct Deleted;

///Вариант основан на преобразованим From and PhantomData


/// New -- Unmoderated
impl From<Post<New>> for Post<Unmoderated> {
    fn from(_val: Post<New>) -> Post<Unmoderated> {
      Post {
            post_id: _val.post_id,
            user: _val.user,
            title: _val.title,
            body: _val.body,
            state: PhantomData,
        }
    }
}

/// Unmoderated -- Published
impl From<Post<Unmoderated>> for Post<Published> {
    fn from(_val: Post<Unmoderated>) -> Post<Published> {

        Post {
            post_id: _val.post_id,
            user: _val.user,
            title: _val.title,
            body: _val.body,
            state: PhantomData,
        }
    }
}

/// Unmoderated -- Deleted
impl From<Post<Unmoderated>> for Post<Deleted> {
    fn from(_val: Post<Unmoderated>) -> Post<Deleted> {
        Post {
            post_id: _val.post_id,
            user: _val.user,
            title: _val.title,
            body: _val.body,
            state: PhantomData,
        }
    }
}

/// Published -- Deleted
impl From<Post<Published>> for Post<Deleted> {
    fn from(_val: Post<Published>) -> Post<Deleted> {
        Post {
            post_id: _val.post_id,
            user: _val.user,
            title: _val.title,
            body: _val.body,
            state: PhantomData,
        }
    }
}

/// Create new Post
/// state New
fn new(user: User, title: String, body: String) -> Post<New> {
    let post: Post<New> = Post {
        post_id: 1u64,
        user: user,
        title: title,
        body: body,
        state: PhantomData,
    };
    post
}

fn publish(post: Post<New>) -> Post<Unmoderated> {
    println!("New -- \"publish()\" --> Unmoderated");
    post.into()
}
fn allow(post: Post<Unmoderated>) -> Post<Published> {
    println!("Unmoderated -- \"allow()\" --> Published");
    post.into()
}

fn deny(post: Post<Unmoderated>) -> Post<Deleted> {
    println!("Unmoderated -- \"deny()\" --> Deleted");
    post.into()
}

fn delete(post: Post<Published>) -> Post<Deleted> {
    println!("Published -- \"delete()\" --> Deleted");
    post.into()
}

fn main() {
    let user = User {
        user_id: 1u64,
        full_name: String::from("Egor Egorov"),
        email: String::from("email@mail.ru"),
    };

    let post_new = new(user, String::from("title"), String::from("body"));

    let post_unmoderated = publish(post_new);

    let post_published = allow(post_unmoderated);

    let _post_delete = delete(post_published);
}
