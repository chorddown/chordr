use crate::domain::setlist::db::SetlistDb;
use crate::domain::setlist::UserSetlist;
use crate::DbConn;
use rocket::{get, post};
use rocket_contrib::json::Json;
// use rocket::data::{Transformed, FromData, Outcome, Transform};

#[get("/")]
pub fn setlist_index(conn: DbConn) -> Json<Vec<UserSetlist>> {
    Json(
        SetlistDb::all(&conn.0)
            .into_iter()
            .map(|setlist_db| {
                let entries = setlist_db.entries(&conn.0);

                UserSetlist::from_data(setlist_db, entries)
            })
            .collect(),
    )
}

#[derive(FromForm, Serialize, Deserialize, Debug)]
pub struct SetlistData {
    id: i32,
}

// impl ::rocket::data::FromData for SetlistData{
//     type Error = ();
//     type Owned = ();
//     type Borrowed = ();
//
//     fn transform(request: &Request<'r>, data: Data) -> Transform<Outcome<Self::Owned, Self::Error>, T> {
//         unimplemented!()
//     }
//
//     fn from_data(request: &Request<'r>, outcome: Transformed<'a, Self>) -> Outcome<Self, Self::Error> {
//         unimplemented!()
//     }
// }

#[post("/<username>/<id>", format = "application/json", data = "<setlist>")]
pub fn setlist_put(
    username: String,
    id: i32,
    conn: DbConn,
    setlist: Json<UserSetlist>,
) -> Json<Vec<UserSetlist>> {
    println!("user {} {} {:?}", username, id, setlist);
    Json(
        SetlistDb::all(&conn.0)
            .into_iter()
            .map(|setlist_db| {
                let entries = setlist_db.entries(&conn.0);

                UserSetlist::from_data(setlist_db, entries)
            })
            .collect(),
    )
}

#[cfg(test)]
mod test {
    use crate::domain::setlist::db::SetlistDb;
    use crate::test_helpers::run_test_fn;
    use rocket::http::ContentType;

    #[test]
    fn test_insertion_deletion() {
        // Setlist
        run_test_fn(|client, conn| {
            assert!(
                SetlistDb::delete_all(&conn.0),
                "Failed to delete all data before testing"
            );
            // Get the tasks before making changes.
            let init_tasks = SetlistDb::all(&conn.0);

            // Issue a request to insert a new task.
            client
                .post("/setlist/daniel/1")
                .header(ContentType::JSON)
                .body(r#"{"id":2,"user":2,"user_name":"daniel","entries":[{"song_id":"swing_low_sweet_chariot.chorddown","file_type":"chorddown","title":"Swing Low Sweet Chariot"},{"song_id":"amazing_grace.chorddown","file_type":"chorddown","title":"Amazing Grace"}]}"#)
                .dispatch();

            // Ensure we have one more task in the database.
            let new_tasks = SetlistDb::all(&conn.0);
            assert_eq!(new_tasks.len(), init_tasks.len() + 1);

            // Ensure the task is what we expect.
            // assert_eq!(new_tasks[0].description, "My first task");
            // assert_eq!(new_tasks[0].completed, false);

            // Issue a request to delete the task.
            let id = new_tasks[0].id;
            client.delete(format!("/todo/{}", id)).dispatch();

            // Ensure it's gone.
            let final_tasks = SetlistDb::all(&conn.0);
            assert_eq!(final_tasks.len(), init_tasks.len());
            if final_tasks.len() > 0 {
                // assert_ne!(final_tasks[0].description, "My first task");
            }
        })
    }
}
