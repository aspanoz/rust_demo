use std::fmt::{self, Display, Formatter};

#[derive(Debug, Clone)]
pub enum Category {
   Animation,
   Gif,
   Picture,
   Static,
   Video,
   Error,
}

impl Display for Category {
   fn fmt(&self, f: &mut Formatter) -> fmt::Result {
      let value = match self {
         Category::Gif => "Gif",
         Category::Picture => "Picture",
         Category::Animation => "Animation",
         Category::Static => "Static",
         Category::Video => "Video",
         Category::Error => "Error",
      };

      write!(f, "{}", value)
   }
}

#[derive(Debug, Clone)]
pub struct Media {
   pub path: String,
   pub category: Category,
   pub width: Option<i32>,
   pub height: Option<i32>,
   pub data: Option<String>,
}

impl Media {
   pub fn new(path: String, category: Category) -> Self {
      Self {
         path,
         category,
         data: None,
         width: None,
         height: None,
      }
   }
}
