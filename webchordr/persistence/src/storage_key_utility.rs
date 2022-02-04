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
    format!(
        "{}{}{}",
        build_combined_key(&context.namespace, &context.key,),
        SEPARATOR,
        id
    )
}
