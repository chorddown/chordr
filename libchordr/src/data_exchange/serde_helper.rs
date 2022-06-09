use serde::de;

pub(crate) fn deserialize_i32_fromstr<'de, D>(deserializer: D) -> Result<i32, D::Error>
where
    D: de::Deserializer<'de>,
{
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    s.parse::<i32>().map_err(serde::de::Error::custom)
}

pub(crate) fn deserialize_opt_isize_fromstr<'de, D>(
    deserializer: D,
) -> Result<Option<isize>, D::Error>
where
    D: de::Deserializer<'de>,
{
    let input: &str = de::Deserialize::deserialize(deserializer)?;
    input
        .parse::<isize>()
        .map(|v| Some(v))
        .map_err(de::Error::custom)
}
