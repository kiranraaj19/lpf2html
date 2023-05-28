use maud::{html, PreEscaped};
use pulldown_cmark::{html, Parser};

#[derive(Debug, PartialEq)]
pub struct Slide {
    // Example: [0.5|0.5]|[0.3|0.2|0.5]
    //          |--VSI--||----VSI----|
    //          |HSI||HSI|

    /// Slides content is html, which will be rendered to file
    htmlstr: String,
}

// CSS Logic
mod css;
use css::Css;

#[derive(Debug, PartialEq)]
pub struct VStack {
    inner: Vec<VStackItem>
}

// VSI
#[derive(Debug, PartialEq)]
pub struct VStackItem {
    pub inner: Vec<HStackItem>,
}

#[derive(Debug, PartialEq)]
pub struct HStackItem {
    pub div_width: f32,
    pub content: String,
}

impl HStackItem {
    // Render method will create a div with given width and content
    pub fn new(div_width: f32) -> HStackItem {
        HStackItem {
            div_width,
            content: "".to_string(),
        }
    }
}

impl VStackItem {
    // Creates new vstackitem from tokens like [0.2] or [0.2|0.3]
    pub fn new(s: &str) -> VStackItem {
        // Parse logic
        // Have to parse [a1|a2] into Vec { HStackItem::new(a1) , HStackItem::new(a2), ... }

        VStackItem {
            inner: s
                .split("|")
                .map(|s| HStackItem::new(s.parse::<f32>().unwrap()))
                .collect(),
        }
    }
}

impl VStack {
    fn new(s: Vec<&str>) -> VStack {
        VStack {
            inner: s.iter().map(|x| VStackItem::new(x)).collect()
        }
    }

    // Given a vector of tokens we have to fill each div with the content given
    fn hydrate(&mut self, tokens: Vec<&str>) {
        let mut it1 = tokens.into_iter();

        self.inner.iter_mut().for_each(|x| {
            x.inner.iter_mut().for_each(|y| {
                if let Some(token) = it1.next() {
                    y.content = String::from(token);
                } else {
                    return;
                }
            })
        });
    }

    fn render(&mut self, css: String) -> String {
        // Convert the VStack struct into final required html string
        println!("{}",self.inner.len());
        let slide_div = html! {
                html {
                    head { style { (Css::preset(css)) }}
                    body {
                        div style="display: flex; flex-direction: column; height:100vh ;width: 100vw" {
                            @for v_div in &mut self.inner {
                                div style="display: flex; flex-direction: row; justify-content: space-around;" {
                                    @for h_div in &mut v_div.inner {
                                        div width=(format!{"{}vw",(h_div.div_width*100.00)}) style="justify-content:center" { 
                                            (PreEscaped(md_to_html(&h_div.content))) 
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
        };

        slide_div.into_string()
    }
}

fn md_to_html(s: &String) -> String {
    let parser = Parser::new(s);

    let mut html_output = String::new();

    // Going through each line and checking if its a text, Html, or a line break.
    // If its a line, then we check if its a Decorator, if So, we push required html
    html::push_html(&mut html_output, parser);
    html_output
}

fn vstack_stringify(s: &str) -> Vec<&str> {
    s[1..s.len()-1].split("][").collect()
}

impl Slide {
    pub fn new(s: &str, css: String) -> Slide {
        let binding = String::from(s);
        let items: Vec<&str> = binding.split("---").map(|item| item.trim()).collect();
        
        let mut vstack = VStack::new(vstack_stringify(&items[0]));
        println!("{:?}",vstack);
        // Fills the content into VStack
        vstack.hydrate(items.into_iter().skip(1).collect());

        // Render the slide with perfect width and content!
        Slide {
            htmlstr: vstack.render(css),
        }
    }

    pub fn to_html(self) -> String {
        self.htmlstr
    }
}
