#[macro_export]
macro_rules! convert_mongo_key_to_string 
{
    ($arg:expr, $type:expr) => 
    {
        {
            let slice_params = match $type {
                "uuid" => (6, -2),
                _ => (0, 0), 
            };

            doc! {
                "$function": {
                    "body": format!("function(x) {{ return x?.toString().slice({}, {}); ?? \"\" }}", slice_params.0, slice_params.1),
                    "args": [ $arg ],
                    "lang": "js"
                }
            }
        }
    };
}

#[macro_export]
macro_rules! map_mongo_collection 
{
    ($input:expr, $change_to:expr, $type:expr) => 
    {
        doc!    
        {
            "$map":
            {
                "input": $input,
                "in": 
                {
                    "$mergeObjects": ["$$this", { $change_to : convert_mongo_key_to_string!("$$this._id", $type) }]
                }
            } 
        }
    };
}