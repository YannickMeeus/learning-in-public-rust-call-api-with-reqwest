#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    
    // Based on https://github.com/seanmonstar/reqwest/blob/master/examples/simple.rs
    //  Simple GET request, no JSON marshalling
    let response = reqwest::get("https://pokeapi.co/api/v2/pokedex/kanto/").await?;
    let body = response.text().await?;
    print!("Body: \n{}", body);
    Ok(())
}
