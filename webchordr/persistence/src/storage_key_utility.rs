use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

use cqrs::prelude::RecordTrait;

use crate::persistence_manager::CommandContext;

pub(crate) const SEPARATOR: char = '.';

pub fn build_combined_key<N: AsRef<str>, K: AsRef<str>>(namespace: &N, key: &K) -> String {
    if namespace.as_ref().is_empty() {
        key.as_ref().to_string()
    } else {
        format!("{}{}{}", namespace.as_ref(), SEPARATOR, key.as_ref())
    }
}

pub fn build_combined_id_key<R: RecordTrait>(context: &CommandContext, id: &R::Id) -> String {
    fn calculate_hash<T: Hash>(t: &T) -> u64 {
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }

    format!(
        "{}{}{}",
        build_combined_key(&context.namespace, &context.key,),
        SEPARATOR,
        calculate_hash(&id)
    )
}
