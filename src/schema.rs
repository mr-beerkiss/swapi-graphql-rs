use std::fs::read_to_string;
use std::path::Path;

use std::fs::File;
use std::io::prelude::*;

use juniper::FieldResult;
use juniper::RootNode;
use uuid::Uuid;

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

// Star Wars API

// Luke - https://swapi.dev/api/people/1/
// C3PO - https://swapi.dev/api/people/2/
// R2D2 - https://swapi.dev/api/people/3/
// Vader - https://swapi.dev/api/people/4/
// Leia - https://swapi.dev/api/people/5/
// Obi Wan - https://swapi.dev/api/people/10/
// Chewy - https://swapi.dev/api/people/13/
// Han - https://swapi.dev/api/people/14/

fn read_humans() -> Vec<Human> {
  let humans_path = Path::new("data/humans.json");
  let humans_json_str = read_to_string(humans_path).expect("could not read from file");
  let deserialized_people: Vec<Human> = serde_json::from_str(&humans_json_str).expect("error parsing json");
  deserialized_people
}

fn add_hunman(human: Human) {
  let mut humans = read_humans();
  humans.push(human);

  let json_str = serde_json::to_string(&humans).expect("could not convert humans to json");

  let humans_path = Path::new("data/humans.json");
  let mut file = File::create(humans_path).expect("Unable to create file for writing");
  file.write_all(json_str.as_bytes()).expect("Unable to write humans to disk");
  
}


#[derive(GraphQLEnum, Clone, Serialize, Deserialize)]
enum Episode {
  NewHope,
  Empire,
  Jedi,
}

#[derive(GraphQLObject, Clone, Serialize, Deserialize)]
#[graphql(description = "A humanoid create in the Star Wars universe")]
struct Human {
  id: String,
  name: String,
  appears_in: Vec<Episode>,
  home_planet: String,
}

impl Human {
  pub fn from(human_input: HumanInput) -> Self {
    Human {
      id: Uuid::new_v4().to_string(),
      name: human_input.name,
      appears_in: human_input.appears_in,
      home_planet: human_input.home_planet,
    }
  }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid create in the Star Wars universe")]
struct HumanInput {
  name: String,
  appears_in: Vec<Episode>,
  home_planet: String,
}

pub struct QueryRoot {}

#[juniper::object]
impl QueryRoot {
  fn human(&self, id: String) -> FieldResult<Human> {
    let human = read_humans()
      .into_iter()
      .find(|human| human.id == id)
      .unwrap();
    Ok(human)
  }

  fn humans(&self) -> FieldResult<Vec<Human>> {
    Ok(read_humans())
  }
}

pub struct MutationRoot {}

#[juniper::object]
impl MutationRoot {
  fn createHuman(new_human: HumanInput) -> FieldResult<Human> {
    let human = Human::from(new_human);

    add_hunman(human.clone());

    Ok(human)
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  Schema::new(
    QueryRoot {},
    MutationRoot {},
  )
}
