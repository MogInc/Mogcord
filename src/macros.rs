#[macro_export]
macro_rules! convert_UUID_to_string 
{
    ($arg:expr) => 
    {
        doc! {
            "$function": 
            {
                "body": "function(x) { return x.toString().slice(6, -2); }",
                "args": [ $arg ],
                "lang": "js"
            }
        }
    };
}

#[macro_export]
macro_rules! map_mongo_collection 
{
    ($arg:expr) => 
    {
        doc!    
        {
            "$map":
            {
                "input": $arg,
                "in": 
                {
                    "$mergeObjects": ["$$this", { "uuid" : convert_UUID_to_string!("$$this._id") }]
                }
            } 
        }
    };
}