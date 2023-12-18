// @generated automatically by Diesel CLI.

diesel::table! {
    links (slug) {
        #[max_length = 1024]
        url -> Varchar,
        #[max_length = 255]
        slug -> Varchar,
    }
}
