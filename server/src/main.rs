use revise_badvcomp::elec1601::topic_1_base_encoding::base_encoding_question;
use revise_badvcomp::{
    constants::{ASCII_ART, PORT},
    route::create_router,
};

#[tokio::main]
async fn main() {
    let app = create_router();

    match base_encoding_question() {
        Ok((question, answer)) => {
            println!("Question: {}", question);
            println!("Answer: {}", answer);
        }
        Err(e) => println!("Error: {}", e),
    }

    let address = format!("0.0.0.0:{}", PORT);
    let listener = tokio::net::TcpListener::bind(&address).await.unwrap();
    println!(
        "{ASCII_ART}\n🚀 Server has been started! LISTENING ON {}",
        address
    );
    axum::serve(listener, app).await.unwrap();
}
