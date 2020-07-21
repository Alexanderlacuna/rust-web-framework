table! {
    contents (postId) {
        postId -> Integer,
        user_id -> Nullable<Integer>,
        title -> Text,
        content -> Text,
    }
}

table! {
    gender (id) {
        id -> Integer,
        title -> Text,
        person_id -> Nullable<Integer>,
    }
}

table! {
    person (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
    }
}

table! {
    posts (postId) {
        postId -> Integer,
        creator -> Nullable<Integer>,
        title -> Text,
        content -> Text,
    }
}

table! {
    students (id) {
        id -> Integer,
        title -> Text,
        usernames_id -> Nullable<Integer>,
    }
}

table! {
    usernames (id) {
        id -> Integer,
        title -> Text,
        email -> Text,
    }
}

table! {
    users (id) {
        id -> Integer,
        username -> Text,
        email -> Text,
    }
}

joinable!(contents -> users (user_id));
joinable!(gender -> person (person_id));
joinable!(posts -> users (creator));
joinable!(students -> usernames (usernames_id));

allow_tables_to_appear_in_same_query!(
    contents,
    gender,
    person,
    posts,
    students,
    usernames,
    users,
);
