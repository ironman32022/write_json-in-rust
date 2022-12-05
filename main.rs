use serde:: {Deserialize,serialize};
#[derive(Serialize,Deserialize)]
struct paragraph{
    name: String
}

#[derive(Serialize,Deserialize)]
struct Article{
    article:string,
    auther:string,
    paragraph: Vec<Paragraph>
}
fn main(){
    let article : Article = Article { article:String::from("how to work with rust")
     auther:String::from("pavan"),
      paragraph: vec![
        paragraph {
            name: string::from("first sentence")
        },
        paragraph{
            name: string::from("second sentence")
        },
        paragragh{
            name: string::from("end of paragraph")

        },
      ] 
    };

    let json=serde_json::to_string(&article).unwarp();
    println!("the json is :{}",json)
}