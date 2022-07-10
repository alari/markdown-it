// Process [link](<to> "stuff")
//
use crate::{Node, NodeValue, Renderer};
use crate::parser::MarkdownIt;
use crate::parser::internals::syntax_base::generics::inline::full_link;

#[derive(Debug)]
pub struct Link {
    pub url: String,
    pub title: Option<String>,
}

impl NodeValue for Link {
    fn render(&self, node: &Node, fmt: &mut dyn Renderer) {
        let mut attrs : Vec<(&str, &str)> = Vec::with_capacity(2);
        attrs.push(("href", &self.url));

        if let Some(title) = &self.title {
            attrs.push(("title", &*title));
        }

        fmt.open("a", &attrs);
        fmt.contents(&node.children);
        fmt.close("a");
    }
}

pub fn add(md: &mut MarkdownIt) {
    full_link::add::<false>(md, |href, title| Node::new(Link {
        url: href.unwrap_or_default(),
        title,
    }));
}
