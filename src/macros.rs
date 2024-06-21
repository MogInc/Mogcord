#[macro_export]
macro_rules! convert_UUID_to_string {
    ($arg:expr) => {
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
macro_rules! map_mongo_collection {
    ($arg:expr) => {
        doc!    
        {
            "owners":
            {
                "$map":
                {
                    "input": "$owners",
                    "in": 
                    {
                    "$mergeObjects": ["$$this", { "uuid" : convert_UUID_to_string!("$$this._id") }]
                    }
                } 
            }
        }
    };
}