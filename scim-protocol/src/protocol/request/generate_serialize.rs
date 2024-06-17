#[macro_export]
macro_rules! generate_request_serialize {
    ($req:ident) => {
       impl serde::Serialize for $req {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                let mut map = serializer.serialize_map(None)?;

                serde::ser::SerializeMap::serialize_entry(&mut map, "externalId", &self.external_id)?;
                serde::Serialize::serialize(
                    &self.resource,
                    $crate::flatmap_serializer::FlatMapSerializer(&mut map),
                )?;
                serde::Serialize::serialize(
                    &self.extensions,
                    $crate::flatmap_serializer::FlatMapSerializer(&mut map),
                )?;

                serde::ser::SerializeMap::end(map)
            }
        }
    };
}
