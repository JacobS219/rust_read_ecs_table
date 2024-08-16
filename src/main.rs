extern crate odbc;

use chrono::NaiveDateTime;
use odbc::*;
use std::error::Error;

/* 
type Result<T> = ...: This is defining a type alias named Result that takes a generic parameter T.
std::result::Result<T, Box<dyn Error>>: The type alias is for the std::result::Result enum, 
which represents either a success (Ok variant) or an error (Err variant).
Box<dyn Error>: This represents a heap-allocated trait object of something that implements the Error trait. 
The dyn keyword indicates a trait object (a type of dynamic dispatch), and Box is a heap-allocated smart pointer.
This is a type alias for a Result with the error type being a trait object. This allows us to return any type that implements the Error trait.
*/ 

type Result<T> = std::result::Result<T, Box<dyn Error>>;  

struct Event {
    /*
        Option<T> is an enum with two variants, Some(T) and None. 
        It's a way of expressing that a value might be absent without resorting to null or special values. 
    */
    eventnumber: i32, // MSSQL Type: PK, int, not null
    event_type: Option<u8>,  // MSSQL Type: tinyint, null - Using `event_type` instead of `type` because `type` is a keyword in Rust
    server: Option<String>, // MSSQL Type: varchar(64), null
    batch: Option<String>, // MSSQL Type: varchar(50), null
    jobnum: Option<String>,  // MSSQL Type: varchar(50), null
    submitted: Option<NaiveDateTime>,  // MSSQL Type: datetime, null
    began: NaiveDateTime,  // MSSQL Type: PK, datetime, not null
    ended: Option<NaiveDateTime>,  // MSSQL Type: datetime, null
    message: Option<String>,  // MSSQL Type: varchar(255), null
    status: Option<u8>,  // MSSQL Type: tinyint, null
    priority: Option<u8>,  // MSSQL Type: tinyint, null
    fixedby: Option<String>,  // MSSQL Type: varchar(48), null
    fixcomment: Option<String>,  // MSSQL Type: varchar(255), null
    color: Option<u8>,  // MSSQL Type: tinyint, null
    bkcolor: Option<u8>,  // MSSQL Type: tinyint, null
    beingworkedon: Option<u8>,  // MSSQL Type: varchar(48), null
    dateclosed: Option<NaiveDateTime>,  // MSSQL Type: datetime, null
    added: Option<NaiveDateTime>,  // MSSQL Type: datetime, null
}

fn main() -> Result<()> {
    // This is a 64 bit ODBC Connection and will not work on 32 bit systems.
    let conn_str = "DSN=GECS_Testing;";

    /*
    1. `Environment::new()`: This is calling a static method named `new` on the `Environment` struct (or type). 
        This method typically creates and returns a new instance of the `Environment` type. In the context of ODBC, the `Environment` represents 
        the ODBC environment which is a foundational setup needed to work with ODBC in an application. 
    2. `.unwrap()`: This method is called on the `Result` or `Option` that `Environment::new()` returns. 
        If `Environment::new()` returns an `Ok` variant of a `Result`, then `.unwrap()` extracts and returns the value inside the `Ok`. 
        If it returns an `Err` variant (i.e., if there was an error creating the ODBC environment), then `.unwrap()` will panic and terminate the program.
        In short, this line attempts to create a new ODBC environment, and if successful, binds it to the variable `env`. 
        If there's any error in the creation process, the program will panic and terminate because of the `.unwrap()`.
    */
    let env = Environment::new().unwrap();
    let conn = env.connect_with_connection_string(&conn_str)?;

    /*
        `Statement::with_parent(&conn)?` is a method call on the `Statement` type. In the context of ODBC:
    1. `Statement`: In ODBC, a statement is an object that allows you to execute SQL commands and queries against a database. 
        Once you have a connection to a database (represented by the `conn` variable), you can create one or more statements to interact with that database.
    2. `with_parent(&conn)`: The `with_parent` method is used to create a new statement that is associated with a particular connection. 
        The method takes a reference to a connection (`&conn` in this case) as its argument, indicating that the new statement will use that connection to communicate 
        with the database.
    3. `?`: This is the try operator in Rust. If `Statement::with_parent(&conn)` returns an `Ok` variant of a `Result`, the value inside that `Ok` is extracted. 
        If it returns an `Err` variant (indicating an error occurred while creating the statement), then the error is returned early from the current function.
        In essence, `let stmt = Statement::with_parent(&conn)?;` is trying to create a new ODBC statement associated with the given database connection, and if successful, 
        binds it to the variable `stmt`. If there's an error, the current function will return early with that error.
    4.  This means that stmt is an immutable binding to a Statement object.
    */
    let stmt = Statement::with_parent(&conn)?;
    let sql_text = "SELECT * FROM [GECS_Testing].[dbo].[GECSEVENTS];";

    /*
        Function Call: The method 'exec_direct' is being called on the stmt object (which is an instance of Statement). 
        This method attempts to directly execute a given SQL command or query represented by the string &sql_text.
        Why does exec_direct take a string 'reference' &str instead of a string 'value' str?
        The function doesn't need to own the string; it just needs to read it.
        Also after the call to exec_direct, sql_text can still be used or modified in your code if needed.
        By accepting a reference, the function can operate on the data without taking ownership, which can help prevent unnecessary allocations or data movements.
        The Try Operator (?): After exec_direct is called, the ? operator is used. This operator checks the Result returned by exec_direct. 
        If the Result is an Ok variant (indicating the operation was successful), it will extract the value inside the Ok for further use. 
        If the Result is an Err variant (indicating an error occurred during the execution of the SQL statement), it will immediately return that error from the current function.
        Pattern Matching with match: The value extracted from the Ok variant (or, in another way to think about it, the result of the successful execution of the SQL statement) 
        is then passed into a match expression. A match expression in Rust is used for pattern matching: 
        it allows you to check the value against several potential patterns and execute code based on which pattern the value matches.
        In this specific case, it's likely that exec_direct returns a Result where the "successful" type can be one of two variants, 
        probably something like Data (indicating that the SQL statement returned some data) and NoData (indicating that the SQL statement executed successfully 
        but did not return any data, like an UPDATE or DELETE command in SQL might).
        The code that follows the match expression will contain branches for each of these patterns, specifying what to do in each case.
        Data() & NoData()
    */

    match stmt.exec_direct(&sql_text)? {
        /*
            'mut stmt' Here, Data(mut stmt) is a pattern match on the Data variant of the result. 
            The mut stmt inside the pattern means that if the result of exec_direct is the Data variant, 
            then bind the value inside this variant to a new, mutable variable named stmt.
            This effectively "shadows" the original immutable stmt. (let stmt = Statement::with_parent(&conn)?;)
            Shadowing means that within the scope of the Data match arm, 
            the name stmt refers to this new mutable variable, and not the original immutable one. 
            This is a common pattern in Rust to transition from an immutable to a mutable variable without needing to come up with a new name.
         */
        Data(mut stmt) => {
            while let Some(mut cursor) = stmt.fetch()? {
                /*
                    1. **`cursor.get_data(1)?`**: 
                        - The `cursor` object represents a position within a result set from a database query.
                        - The `get_data` method is being called on the `cursor` to retrieve the data from the first column of the current row, since indexing starts at 1 in this context.
                        - The type of data that `get_data` returns is generic and can vary. In this context, it's expected to be an `Option<T>` where `T` is the type like `String`.
                        - The `?` operator is used for error propagation in Rust. If `get_data` returns an error, the function will immediately return that error. 
                          If `get_data` succeeds, it will give back the contained value from the `Ok` variant.
                 */
                let eventnumber_str: Option<String> = cursor.get_data(1)?;
                let eventnumber: i32 = eventnumber_str.unwrap_or_default().parse()?;

                let eventtype_str: Option<String> = cursor.get_data(2)?;
                /*
                    'and_then' method of Option<T>. 
                    This method is useful when you want to transform the inner value of an Option (if there is one) and produce another Option.

                    1. **`eventtype_str`**: An `Option<String>`. It may or may not contain a `String`.
                    2. **`and_then`**: It calls the provided closure (the anonymous function) if there's a `Some(T)` value inside the `Option`. 
                        Otherwise, if it's `None`, it does nothing and just returns `None`.
                    3. **The Closure**: `|s| s.parse::<u8>().ok()`
                        - `|s|`: This is an argument list. It declares a single argument `s` which represents the `String` inside `eventtype_str` (if there is one).
                        - `s.parse::<u8>()`: Tries to parse the `String` as a `u8` value.
                        - `.ok()`: Converts the `Result` returned by `parse` into an `Option`. 
                        If the parsing is successful, it'll produce `Some(u8)`. If there's an error, it'll produce `None`.

                        The anonymous function more verbosely without using a closure, it might look like this:

                        fn parse_u8_from_string(s: String) -> Option<u8> {
                            match s.parse::<u8>() {
                                Ok(value) => Some(value),
                                Err(_) => None
                            }
                        }

                        let event_type = if let Some(inner_string) = eventtype_str {
                            parse_u8_from_string(inner_string)
                        } else {
                            None
                        };
                 */
                let event_type = eventtype_str.and_then(|s| s.parse::<u8>().ok());

                let server: Option<String> = cursor.get_data(3)?;

                let batch: Option<String> = cursor.get_data(4)?;

                let jobnum: Option<String> = cursor.get_data(5)?;

                let submitted_str: Option<String> = cursor.get_data(6)?;
                let submitted = submitted_str
                    .and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok());

                let began_str: String = cursor.get_data(7)?.ok_or("Missing value for 'began'")?;
                let began = NaiveDateTime::parse_from_str(&began_str, "%Y-%m-%d %H:%M:%S%.f")?;

                let ended_str: Option<String> = cursor.get_data(8)?;
                let ended = ended_str
                    .and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok());

                let message: Option<String> = cursor.get_data(9)?;

                let status_str: Option<String> = cursor.get_data(10)?;
                let status = status_str.and_then(|s| s.parse::<u8>().ok());

                let priority_str: Option<String> = cursor.get_data(11)?;
                let priority = priority_str.and_then(|s| s.parse::<u8>().ok());

                let fixedby: Option<String> = cursor.get_data(12)?;

                let fixcomment: Option<String> = cursor.get_data(13)?;

                let color_str: Option<String> = cursor.get_data(14)?;
                let color = color_str.and_then(|s| s.parse::<u8>().ok());

                let bkcolor_str: Option<String> = cursor.get_data(15)?;
                let bkcolor = bkcolor_str.and_then(|s| s.parse::<u8>().ok());

                let beingworkedon_str: Option<String> = cursor.get_data(16)?;
                let beingworkedon = beingworkedon_str.and_then(|s| s.parse::<u8>().ok());

                let dateclosed_str: Option<String> = cursor.get_data(17)?;
                let dateclosed = dateclosed_str
                    .and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok());

                let added_str: Option<String> = cursor.get_data(18)?;
                let added = added_str
                    .and_then(|s| NaiveDateTime::parse_from_str(&s, "%Y-%m-%d %H:%M:%S").ok());

                let event = Event {
                    eventnumber: eventnumber,
                    event_type: event_type,
                    server: server,
                    batch: batch,
                    jobnum: jobnum,
                    message: message,
                    status: status,
                    priority: priority,
                    fixedby: fixedby,
                    fixcomment: fixcomment,
                    color: color,
                    bkcolor: bkcolor,
                    beingworkedon: beingworkedon,
                    submitted: submitted,
                    began: began,
                    ended: ended,
                    dateclosed: dateclosed,
                    added: added,
                    // submitted: cursor.get_data(6)?,
                    // began: cursor.get_data(7)?,
                    // ended: cursor.get_data(8)?,
                    // dateclosed: cursor.get_data(17)?,
                    // added: cursor.get_data(18)?.unwrap_or_default(),
                };

                match event.event_type {
                    Some(t) => println!("Event Type: {}", t),
                    None => println!("Event Type: NULL"),
                }

                println!("Event Number: {}", event.eventnumber);
                println!("Server: {}", event.server.unwrap_or("NULL".to_string()));
                println!("Batch: {}", event.batch.unwrap_or("NULL".to_string()));
                println!("Job Number: {}", event.jobnum.unwrap_or("NULL".to_string()));
                println!(
                    "Submitted: {}",
                    event
                        .submitted
                        .map_or("NULL".to_string(), |d| d.to_string())
                );
                println!("Began: {}", event.began);
                println!(
                    "Ended: {}",
                    event.ended.map_or("NULL".to_string(), |d| d.to_string())
                );
                println!("Message: {}", event.message.unwrap_or("NULL".to_string()));
                println!(
                    "Status: {}",
                    event.status.map_or("NULL".to_string(), |d| d.to_string())
                );
                println!(
                    "Priority: {}",
                    event.priority.map_or("NULL".to_string(), |d| d.to_string())
                );
                println!(
                    "Fixed By: {}",
                    event.fixedby.map_or("NULL".to_string(), |d| d.to_string())
                );
                println!(
                    "Fix Comment: {}",
                    event
                        .fixcomment
                        .map_or("NULL".to_string(), |d| d.to_string())
                );
                println!(
                    "Color: {}",
                    event.color.map_or("NULL".to_string(), |d| d.to_string())
                );
                println!(
                    "BkColor: {}",
                    event.bkcolor.map_or("NULL".to_string(), |d| d.to_string())
                );
                println!(
                    "Being Worked On: {}",
                    event
                        .beingworkedon
                        .map_or("NULL".to_string(), |d| d.to_string())
                );
                println!(
                    "Date Closed: {}",
                    event
                        .dateclosed
                        .map_or("NULL".to_string(), |d| d.to_string())
                );
                println!(
                    "Added: {}",
                    /*
                        map_or: This is a method on the Option type. It takes two arguments:
                        A default value ("NULL".to_string() in this case) to be used if the Option is None.
                        A closure (|d| d.to_string()) to be applied if the Option is Some.
                        If event.added is Some(d), then d.to_string() will be executed, converting the NaiveDateTime to a String.
                        If event.added is None, then "NULL".to_string() will be returned.
                        In simpler terms, this expression is converting the NaiveDateTime to a String if it exists. 
                        If it doesn't exist (i.e., it's None), then the string "NULL" is returned.
                    */
                    /*
                        In Rust, a closure is a way to define an anonymous function. 
                        It's called "closure" because it can "close over" variables from its surrounding scope, capturing them in its environment.
                        Here are some characteristics and examples of closures in Rust:
                        Basic Usage: Closures are often used as arguments to functions, especially for short, "throw-away" functions that you don't want to name.
                            let numbers = vec![1, 2, 3, 4, 5];
                            let squared: Vec<_> = numbers.iter().map(|x| x * x).collect();
                            println!("{:?}", squared); // [1, 4, 9, 16, 25]
                        In the example above, |x| x * x is a closure that takes a value x and returns its square.
                        Environment Capture: Closures can capture values from their surrounding environment.
                            let multiplier = 2;
                            let multiply_by = |x| x * multiplier;
                            println!("{}", multiply_by(10)); // 20
                        In this example, the closure multiply_by captures the multiplier variable from its surrounding environment.
                        Types of Capture: Closures can capture variables in their environment in different ways:
                            By reference: |&x|
                            By mutable reference: |&mut x|
                            By value (moving the value): |x|
                            Type Inference: One advantage of closures in Rust is that they can have inferred input and return types, so you often don't need to annotate them.
                        Flexibility with Parameters and Body: Like functions, closures can take multiple parameters, and their body can have multiple statements.
                                let greeting = |name, time_of_day| {
                                println!("Hello, {}", name);
                                println!("Good {}", time_of_day);
                            };
                            greeting("Alice", "morning");
                        Fn, FnMut, and FnOnce: Rust has three traits to represent how a closure captures variables from its environment: Fn, FnMut, and FnOnce. Each one allows different types of manipulation of the captured environment:
                            Fn: borrows values immutably.
                            FnMut: borrows values mutably.
                            FnOnce: takes ownership of the environment.
                            In essence, closures provide a convenient way to define small, anonymous bits of functionality inline, with the added power of capturing their environment. They are especially useful for higher-order functions, callback-style functions, and any situation where a small, specialized bit of logic is needed.
                    */
                    event.added.map_or("NULL".to_string(), |d| d.to_string())
                );
                println!();
            }
        }
        NoData(_) => eprintln!("Query executed, but no data returned."),
    }

    Ok(())
}

// use tiberius::{Client, Config, QueryItem};
// use tokio::net::TcpStream;
// use tokio_util::compat::TokioAsyncReadCompatExt;
// use futures::stream::StreamExt;
// extern crate odbc;
// use odbc::*;

// #[tokio::main]
// async fn main() -> Result<(), Box<dyn std::error::Error>> {
//     // Hardcoded connection string for testing. NEVER do this in production or in shared code.
//     let conn_str = "server=tcp:MINI\\BEESERVER,1433;user id=sa;password=jollyroger1;initial catalog=GECS_Testing;trustServerCertificate=true;";

//     let config = Config::from_ado_string(&conn_str)?;

//     // Create a tokio TcpStream
//     let tokio_stream = TcpStream::connect(config.get_addr()).await?;

//     // Convert the tokio TcpStream to a compatible stream
//     let compat_stream = tokio_stream.compat();

//     // Now use the compat_stream to connect with tiberius
//     let mut client = Client::connect(config, compat_stream).await?;

//     println!("Connected!");

//     let mut query_stream = client.simple_query("SELECT column_name FROM your_table").await?;

//     while let Some(item_result) = query_stream.next().await {
//         match item_result {
//             Ok(QueryItem::Row(row)) => {
//                 let column: &str = row.get(0).unwrap();
//                 println!("{}", column);
//             },
//             Ok(_) => {},  // Handle other types of QueryItem if needed
//             Err(e) => {
//                 eprintln!("Error processing row: {}", e);
//             }
//         }
//     }

//     Ok(())
// }
