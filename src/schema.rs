use juniper::FieldResult;
use juniper::RootNode;
use std::sync::Arc;
use uuid::Uuid;

// Star Wars API

// Luke - http://swapi.dev/api/people/1/
// C3PO - http://swapi.dev/api/people/2/
// R2D2 - http://swapi.dev/api/people/3/
// Vader - http://swapi.dev/api/people/4/
// Leia - http://swapi.dev/api/people/5/
// Obi Wan - http://swapi.dev/api/people/10/
// Chewy - http://swapi.dev/api/people/13/
// Han - http://swapi.dev/api/people/14/

#[derive(GraphQLEnum, Clone)]
enum Episode {
  NewHope,
  Empire,
  Jedi,
}

use juniper::{GraphQLEnum, GraphQLInputObject, GraphQLObject};

#[derive(GraphQLObject, Clone)]
#[graphql(description = "A humanoid create in the Star Wars universe")]
struct Human {
  id: String,
  name: String,
  appears_in: Vec<Episode>,
  home_planet: String,
}

impl Human {
  pub fn new(id: &str, name: &str, appears_in: Vec<Episode>, home_planet: &str) -> Self {
    Human {
      id: String::from(id),
      name: String::from(name),
      appears_in,
      home_planet: String::from(home_planet),
    }
  }
}

#[derive(GraphQLInputObject)]
#[graphql(description = "A humanoid create in the Star Wars universe")]
struct NewHuman {
  name: String,
  appears_in: Vec<Episode>,
  home_planet: String,
}

pub struct QueryRoot {
  storage: Arc<Vec<Human>>,
}

#[juniper::object]
impl QueryRoot {
  fn human(&self, id: String) -> FieldResult<Human> {
    let human = self
      .storage
      .to_vec()
      .into_iter()
      .find(|human| human.id == id)
      .unwrap();
    Ok(human)
  }

  fn humans(&self) -> FieldResult<Vec<Human>> {
    Ok(self.storage.to_vec())
  }
}

pub struct MutationRoot {
  // storage: Arc<Vec<Human>>,
}

#[juniper::object]
impl MutationRoot {
  fn createHuman(new_human: NewHuman) -> FieldResult<Human> {
    Ok(Human {
      id: Uuid::new_v4().to_string(),
      name: new_human.name,
      appears_in: new_human.appears_in,
      home_planet: new_human.home_planet,
    })
  }
}

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_schema() -> Schema {
  // let storage = Arc::new(Vec::new());
  let original_trilogy = vec![Episode::NewHope, Episode::Empire, Episode::Jedi];
  let luke_skywalker = Human::new("1", "Luke Skywalker", original_trilogy.to_vec(), "Tatooine");
  let han_solo = Human::new("14", "Han Solo", original_trilogy.to_vec(), "Corellia");
  let humans = vec![luke_skywalker, han_solo];
  let storage = Arc::new(humans);
  Schema::new(
    QueryRoot {
      storage: Arc::clone(&storage),
    },
    MutationRoot { /*storage: Arc::clone(&storage)*/ },
  )
}
