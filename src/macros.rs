#[macro_export]
macro_rules! convert_mongo_key_to_string 
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
macro_rules! map_mongo_collection_keys_to_string 
{
    ($input_collection:expr, $renamed_id:expr, $id_type:expr) => 
    {
        doc!    
        {
            "$map":
            {
                "input": $input_collection,
                "in": 
                {
                    "$mergeObjects": ["$$this", { $renamed_id : convert_mongo_key_to_string!("$$this._id", $id_type) }]
                }
            } 
        }
    };
}