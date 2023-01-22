// We haven't used most of the things from struct but we will use them later 

use dotenv::dotenv; // 1. Import dotenv
use hyper::body::Buf; 
use hyper::{Body, Client, Request, header};
use hyper_tls::HttpsConnector; 
use serde_derive::{Deserialize, Serialize}; 
use spinners::{Spinner, Spinners};
use std::env; // 2. Import env
use std::io::{stdin, stdout, Write};

// OAIChoices is the struct for the choices returned by the OpenAI API
#[derive(Deserialize, Debug)]
struct OAIChoices {
    text: String,
    index: u8,
    logProbs: Option<u8>, 
    finishReason: String, 
}

#[derive(Deserialize, Debug)]
struct OAIResponse {
    id: Option<String>,
    object: Option<String>,
    created: Option<u64>,
    model: Option<String>,
    choices: Vec<OAIChoices>, 
}

// OAIRequest is the struct for the request sent to the OpenAI API
#[derive(Serialize, Debug)]
struct OAIRequest {
    prompt: String,
    max_tokens: u16, // this does not mean the API or authentication token, this is the number of words you want to generate
   /*  temperature: f32,
    top_p: f32,
    frequency_penalty: f32,
    presence_penalty: f32,
    stop: Vec<String>, */
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> { // send + Sync are rust traits for threads saafety
    // 3. Load .env file
    dotenv().ok(); // ok() is a method that returns a Result<T, E> and discards the error value

    // create a http connector, hyper
    let https = HttpsConnector::new();

    // create a client, hyper
    let client = Client::builder().build(https);

    // URL for the OpenAI API
    let uri = "https://api.openai.com/v1/engines/text-davinci-001/completions";
    
    // preamble for the prompt to be sent to the OpenAI API(chatgpt)
    let preamble = "Generate a story about a person who is.";
    let oai_token: String = env::var("OAI_TOKEN").unwrap(); // 4. Get the OAI_TOKEN from the .env file
    let auth_header_val = format!("Bearer {}", oai_token); // 5. Create the authorization header value

    // The character code 27 is the ASCII code for the "Escape" character (often abbreviated as "Esc")
    println!("{esc}c", esc = 27 as char); // clear the terminal

    // create the input from the user
    loop {

        // this > will be used to indicate that the user can input text
        print!(">");

        // flush the stdout buffer to the terminal so that the > is displayed
        stdout().flush().unwrap();

        // create a mutable string to hold the user input
        let mut input = String::new();

        // read the user input and store it in the input string
        stdin().read_line(&mut input).expect("Failed to read line");


        println!(""); 

        // create a spinner, wait for the response from the OpenAI API

        // into method changes the type of somthing into another type by consuming the original value
        let spinner = Spinner::new(&Spinners::Dots12, "Loading...\t\t OpenAI is Thinking...".into());

        // Request to CHATGPT for every single user input, loop

        // this is the req object that will be sent to the OpenAI API
        let oai_request = OAIRequest {
            prompt: format!("{} {}", preamble, input),
            max_tokens: 1000, // it is the number of words you want to generate
        };

        // sending the request to the OpenAI API
        let body = Body::from(serde_json::to_vec(&oai_request)?); // serde_json::to_vec() converts the oai_request object to a json string and returns a vector of bytes
        let req = Request::post(uri)
            .header(header::AUTHORIZATION, &auth_header_val)
            .header(header::CONTENT_TYPE, "application/json")
            .body(body)
            .unwrap();

        // get the response from the OpenAI API
        let res = client.request(req).await?; // res is the response from the OpenAI API
        let body = hyper::body::aggregate(res).await?; // body is the body of the response from the OpenAI API , aggregate() is a method that reads the body of the response from the OpenAI API and returns a Buf, which is a trait that represents a buffer of bytes that can be read from and written to in a streaming fashion
        let json:OAIResponse = serde_json::from_reader(body.reader())?; // serde_json::from_reader() converts the body of the response from the OpenAI API to a json string and returns a OAIResponse object
        spinner.stop(); // stop the spinner
        println!(""); // print a new line
        println!("{}", json.choices[0].text); // print the response from the OpenAI API
    }

    Ok(())
}



/* 
// https://web.mit.edu/rust-lang_v1.25/arch/amd64_ubuntu1404/share/doc/rust/html/book/second-edition/ch16-04-extensible-concurrency-sync-and-send.html

Send + Sync are Rust's trait for thread safety. Send trait indicates that the 
type can be safely sent to another thread, 
Sync trait indicates that it is safe to share the value with multiple threads.
*/


/*  Buffered data is data that is temporarily stored in memory before 
it is written to an output device and an Output stream is an abstraction for a 
device or a file where data can be written to. It is implemented as objects or classes that provide 
methods for writing data to the stream, and can also be buffered, which means that data is 
temporarily stored in memory before it is written to the underlying output device. */