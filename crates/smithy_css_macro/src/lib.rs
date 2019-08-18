extern crate proc_macro;

mod attribute;
mod class;
mod core;
mod id;
mod modifier;
mod selector;

mod parser_types;
mod types;
mod util;

use proc_macro2::TokenStream;

#[proc_macro]
pub fn css(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
  let input: TokenStream = input.into();
  println!("\ninput {:?}", input);
  // this seems like a hack. What I want is an iterator of &TokenTree's,
  // but TokenStream only implements into_iter for some reason
  //
  // (We actually need a slice of TokenTree's)
  // let input = input.into_iter().collect::<TokenTreeVec>();

  // let foo = util::many_0(util::many_0_joint(modifier::parse_modifier))(input);
  // let foo = util::many_0_joint(modifier::parse_modifier)(input);
  let foo = selector::parse_selector(input);
  println!("\nparse result = {:?}", foo);
  match foo {
    Ok((rest, some_vec)) => {
      util::ensure_consumed(rest).expect("outer macro: rest was not empty");
      let foo = format!("{:?}", some_vec);
      quote::quote!({
        #foo
      })
    }
    .into(),
    _ => unimplemented!("outer macro was an Err"),
  }

  // quote::quote!({
  //   #[derive(Debug, Clone)]
  //   struct CssClasses {
  //     my_class: String,
  //   }
  //   #[derive(Debug, Clone)]
  //   struct CssIds {}
  //   #[derive(Debug, Clone)]
  //   struct CssWrapper {
  //     classes: CssClasses,
  //     ids: CssIds,
  //   }
  //   // TODO figure out why this doesn't work
  //   // = help: message: attempt to subtract with overflow
  //   //
  //   // TODO: divide this into smithy_css_core and include this impl only later
  //   // impl CssWrapper {
  //   //   pub fn style_tag<'a>(&self) -> smithy::types::SmithyComponent<'a> {
  //   //     // smithy::smd!(<style>
  //   //     //   foo
  //   //     // </style>)
  //   //     let a = smithy::smd!(<div>a</div>);
  //   //     smithy::smd!()
  //   //   }
  //   // }

  //   impl ToString for CssWrapper {
  //     fn to_string(&self) -> String {
  //       format!(".{} {{ background-color: red; }}", self.classes.my_class)
  //     }
  //   }

  //   let my_class = "foo".into();
  //   CssWrapper {
  //     classes: CssClasses { my_class },
  //     ids: CssIds {},
  //   }
  // })
  // .into()
}
