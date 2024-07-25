#[macro_export]
/// Maps a mongodb key value (uuid, etc.) to string
///
///signature(`$id_name`, `mongo_id_type`),
///  
/// `$` prefix means its a mongo field
/// # Examples
/// ```
/// doc!
/// {
///     "$addFields":
///     {
///         "id": map_mongo_key_to_string!("$_id", "uuid"),
///         "user.id": map_mongo_key_to_string!("$user._id", "uuid"),
///     },
/// }
/// ```
macro_rules! map_mongo_key_to_string 
{
    ($id_field:expr, $id_type:expr) => 
    {
        {
            let slice_params = match $id_type 
            {
                "uuid" => (6, -2),
                _ => (0, 0), 
            };

            doc! 
            {
                "$function": 
                {
                    "body": format!("function(x) {{ return x?.toString().slice({}, {}) ?? \"\"; }}", slice_params.0, slice_params.1),
                    "args": [ $id_field ],
                    "lang": "js"
                }
            }
        }
    };
}

#[macro_export]
/// maps over a mongodb collection and maps the key value (uuid, etc.) to string
/// 
/// uses [`map_mongo_key_to_string`] to map the individual keys
/// 
/// signature(`$collection_name`, `current_id_name`, `rename_id_to`, `mongo_id_type`)
/// 
/// `$` prefix means its a mongo field
/// # Examples
/// ```
/// doc!
/// {
///     "$addFields":
///     {
///         "users": map_mongo_collection_keys_to_string!("$users", "_id", "id", "uuid"),
///         "chat.owners": map_mongo_collection_keys_to_string!("$chat.owners", "_id", "id", "uuid"),
///     }
/// }
/// ```
macro_rules! map_mongo_collection_keys_to_string 
{
    
    ($collection_name:expr, $current_id_name:expr, $rename_id_to:expr, $id_type:expr) => 
    {
        doc!
        {
            "$map":
            {
                "input": $collection_name,
                "in": 
                {
                    "$mergeObjects": ["$$this", { $rename_id_to : map_mongo_key_to_string!(format!("$$this.{}", $current_id_name), $id_type) }]
                }
            } 
        }
    };
}

#[macro_export]
/// maps over a mongodb collection and transforms it into a hashmap
/// 
/// signature(`$collection_name`, `key_name`)
/// 
/// `$` prefix means its a mongo field
/// # Examples
/// ```
/// doc!
/// {
///     "$addFields":
///     {
///         "users": map_mongo_collection_to_hashmap!("$users", "id"),
///     },
/// }
/// ```
/// # Note 
/// if you have transformed the collection you want to map it needs to go in a seperate addFields
/// ```
/// doc!
/// {
///     "$addFields":
///     {
///         "users": map_mongo_collection_keys_to_string!("$users", "_id", "id", "uuid"),
///     },
/// },
/// doc!
/// {
///     "$addFields":
///     {
///         "users": map_mongo_collection_to_hashmap!("$users", "id"),
///     },
/// }
/// ```
macro_rules! map_mongo_collection_to_hashmap 
{
    ($collection_name:expr, $key_name:expr) => 
    {
        doc! 
        {
            "$arrayToObject":
            {
                "$map": 
                {
                    "input": $collection_name,
                    "as": "item",
                    "in":
                    {
                        "k": format!("$$item.{}", $key_name),
                        "v": "$$item"
                    }
                }
            }
        }
    };
}