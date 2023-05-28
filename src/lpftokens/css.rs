pub struct Css(String);

impl Css {
    pub fn preset(s: String) -> String {
        match &s[..] {
            "Default" => {
                String::from(
                    r#"
                        body {
                            background-color: #f1f1f1;
                            color: #333;
                            font-family: Arial, sans-serif;
                            padding: 20px;
                        }
                        
                        h1 {
                            color: #555;
                            font-size: 24px;
                        }
                        
                        p {
                            font-size: 16px;
                            line-height: 1.5;
                        }
                    "#
                )
            },
            "Pastel" => {
                String::from(r#"
                body {
                    background-color: #f5e6df;
                    color: #333;
                    font-family: Arial, sans-serif;
                    padding: 20px;
                }
                
                h1 {
                    color: #555;
                    font-size: 24px;
                }
                
                p {
                    font-size: 16px;
                    line-height: 1.5;
                }
                "#)
            }
            _ => String::new()
        }
    }
}