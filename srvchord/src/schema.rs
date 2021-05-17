table! {
    /// Representation of the `setlist` table.
    ///
    /// (Automatically generated by Diesel.)
    setlist (id) {
        /// The `id` column of the `setlist` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Integer,
        /// The `name` column of the `setlist` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `sorting` column of the `setlist` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        sorting -> Integer,
        /// The `owner` column of the `setlist` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        owner -> Text,
        /// The `team` column of the `setlist` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        team -> Nullable<Text>,
        /// The `gig_date` column of the `setlist` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        gig_date -> Nullable<Timestamp>,
        /// The `creation_date` column of the `setlist` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        creation_date -> Timestamp,
        /// The `modification_date` column of the `setlist` table.
        ///
        /// Its SQL type is `Timestamp`.
        ///
        /// (Automatically generated by Diesel.)
        modification_date -> Timestamp,
    }
}

table! {
    /// Representation of the `setlist_entry` table.
    ///
    /// (Automatically generated by Diesel.)
    setlist_entry (id) {
        /// The `id` column of the `setlist_entry` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Nullable<Integer>,
        /// The `song_id` column of the `setlist_entry` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        song_id -> Text,
        /// The `file_type` column of the `setlist_entry` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        file_type -> Nullable<Text>,
        /// The `title` column of the `setlist_entry` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        title -> Nullable<Text>,
        /// The `settings` column of the `setlist_entry` table.
        ///
        /// Its SQL type is `Nullable<Text>`.
        ///
        /// (Automatically generated by Diesel.)
        settings -> Nullable<Text>,
        /// The `setlist_db_id` column of the `setlist_entry` table.
        ///
        /// Its SQL type is `Integer`.
        ///
        /// (Automatically generated by Diesel.)
        setlist_db_id -> Integer,
        /// The `modification_date` column of the `setlist_entry` table.
        ///
        /// Its SQL type is `Nullable<Timestamp>`.
        ///
        /// (Automatically generated by Diesel.)
        modification_date -> Nullable<Timestamp>,
    }
}

table! {
    /// Representation of the `tasks` table.
    ///
    /// (Automatically generated by Diesel.)
    tasks (id) {
        /// The `id` column of the `tasks` table.
        ///
        /// Its SQL type is `Nullable<Integer>`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Nullable<Integer>,
        /// The `description` column of the `tasks` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        description -> Text,
        /// The `completed` column of the `tasks` table.
        ///
        /// Its SQL type is `Bool`.
        ///
        /// (Automatically generated by Diesel.)
        completed -> Bool,
    }
}

table! {
    /// Representation of the `team` table.
    ///
    /// (Automatically generated by Diesel.)
    team (id) {
        /// The `id` column of the `team` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        id -> Text,
        /// The `name` column of the `team` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        name -> Text,
        /// The `users` column of the `team` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        users -> Text,
    }
}

table! {
    /// Representation of the `user` table.
    ///
    /// (Automatically generated by Diesel.)
    user (username) {
        /// The `username` column of the `user` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        username -> Text,
        /// The `first_name` column of the `user` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        first_name -> Text,
        /// The `last_name` column of the `user` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        last_name -> Text,
        /// The `password_hash` column of the `user` table.
        ///
        /// Its SQL type is `Text`.
        ///
        /// (Automatically generated by Diesel.)
        password_hash -> Text,
    }
}

joinable!(setlist_entry -> setlist (setlist_db_id));

allow_tables_to_appear_in_same_query!(
    setlist,
    setlist_entry,
    tasks,
    team,
    user,
);
