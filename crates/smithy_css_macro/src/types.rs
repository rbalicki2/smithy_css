// TODO handle +, >, pseudo selectors, etc.
pub type Element = String;

// Going off of https://www.w3schools.com/css/css_attribute_selectors.asp
#[derive(Debug, Clone)]
pub enum AttributeRelation {
  Equal(String),          // =
  Containing(String),     // ~=
  BeginsWithWord(String), // |=
  BeginsWith(String),     // ^=
  EndsWith(String),       // $=
  Contains(String),       // *=
}

impl AttributeRelation {
  pub fn from_strings(s1: &str, s2: String) -> Option<AttributeRelation> {
    match s1 {
      "=" => Some(AttributeRelation::Equal(s2)),
      "~=" => Some(AttributeRelation::Containing(s2)),
      "|=" => Some(AttributeRelation::BeginsWithWord(s2)),
      "^=" => Some(AttributeRelation::BeginsWith(s2)),
      "$=" => Some(AttributeRelation::EndsWith(s2)),
      "*=" => Some(AttributeRelation::Contains(s2)),
      _ => None,
    }
  }
}

#[derive(Debug, Clone)]
pub struct AttributeModifier {
  pub attribute: String,
  pub relation: Option<AttributeRelation>,
}

#[derive(Debug, Clone)]
pub enum Modifier {
  Class(String),
  Id(String),
  Attribute(AttributeModifier),
}

// TODO is Selector { element: None, modifiers: vec![] } valid?
#[derive(Debug, Clone)]
pub struct Selector {
  pub element: Option<Element>,
  pub modifiers: Vec<Modifier>,
}

pub type NestedSelector = Vec<Selector>;
pub type NestedSelectorList = Vec<NestedSelector>;
