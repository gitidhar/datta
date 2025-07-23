use std::error::Error;
use anyhow::Result;                       
use async_openai::{types::CreateCompletionRequestArgs, Client};
use dotenv::dotenv;  

#[tokio::main] 
async fn Delegate() -> Result<()> { 

    functions::launch_an_app();

    dotenv().ok();
    let client = Client::new();
    // println!("Yo mama!");
    // single
    let request = CreateCompletionRequestArgs::default()
        .model("gpt-4o-mini")
        .prompt("Tell me a joke about the universe")
        .max_tokens(40_u32)
        .build()?;

    let response = client.completions().create(request).await?;

    println!("\nResponse (single):\n");
    for choice in response.choices {
        println!("{}", choice.text);
    }

    // // multiple
    // let request = CreateCompletionRequestArgs::default()
    //     .model("gpt-3.5-turbo-instruct")
    //     .prompt([
    //         "How old is the human civilization?",
    //         "How far away is the moon in miles?",
    //     ])
    //     .max_tokens(40_u32)
    //     .build()?;

    // let response = client.completions().create(request).await?;

    // println!("\nResponse (multiple):\n");
    // for choice in response.choices {
    //     println!("{}", choice.text);
    // } 

    Ok(())                                
}