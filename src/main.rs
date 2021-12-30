/*
A subset of the spec for a pokemon I've already detailed out in my Golang learning
notes:

type BunchOfPokemon struct {
	Name    string    `json:"name"`
	Pokemon []Pokemon `json:"pokemon_entries"`
}

// A Pokemon Struct to map every pokemon to.
type Pokemon struct {
    EntryNo int            `json:"entry_number"`
    Species PokemonSpecies `json:"pokemon_species"`
}

// A struct to map our Pokemon's Species which includes it's name
type PokemonSpecies struct {
    Name string `json:"name"`
}
*/
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct PokemonSpecies {
    name: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct Pokemon {
    entry_number: i32,
    pokemon_species: PokemonSpecies,
}
#[derive(Debug, Serialize, Deserialize)]
struct BunchOfPokemon 
{
    #[serde(rename = "name")]
    region: String,
    pokemon_entries: Vec<Pokemon>,
}

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    
    // The type hint itself is what is instructing serde to deserialize into the right type.
    //  My mind is blown.

    // The alternative would be to provide the type to deserialize into in the json call itself
    //  ...await?.json::<BunchOfPokemon>().await?
    let json_response: BunchOfPokemon =
        reqwest
            ::get("https://pokeapi.co/api/v2/pokedex/kanto/")
            .await?
            .json()
            .await?;
    
    println!("region: {:#?}", json_response.region);
    println!("pokemon_entries: {:#?}", json_response.pokemon_entries.len());
    println!("All Pokemon Names:");
    for pokemon in json_response.pokemon_entries.iter() {
        println!("{} - {}", pokemon.entry_number, pokemon.pokemon_species.name);
    }
    
    Ok(())
}
