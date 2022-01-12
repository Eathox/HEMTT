use hemtt_arma_config::{
    preprocessor::{Defines, Preprocessor},
    render::render,
    tokenizer::tokenize,
};

#[test]
fn simple_define() {
    let tokens = tokenize(
        "#define brett_greeting \"hi brett\"\n\ngreeting = brett_greeting;\n",
        "test_simple_define",
    )
    .unwrap();
    let defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &defines).unwrap();
    println!("{:?}", render(preprocessor.output()).export());
    assert_eq!(
        render(preprocessor.output()).export(),
        "greeting = \"hi brett\";"
    );
}

#[test]
fn nested_define() {
    let tokens = tokenize(
        "#define NAME brett\n#define HI \"hi NAME\"\n\ngreeting = HI;\n",
        "test_nested_define",
    )
    .unwrap();
    let defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &defines).unwrap();
    println!("{:?}", render(preprocessor.output()).export());
    assert_eq!(
        render(preprocessor.output()).export(),
        "greeting = \"hi brett\";"
    );
}

#[test]
fn chained_define() {
    let tokens = tokenize(
        "#define NAME Brett\n#define SALUTATION Mr.\n#define HI \"Hi SALUTATION NAME\"\n\ngreeting = HI;\n",
        "test_chained_define",
    )
    .unwrap();
    let defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &defines).unwrap();
    println!("{:?}", render(preprocessor.output()).export());
    assert_eq!(
        render(preprocessor.output()).export(),
        "greeting = \"Hi Mr. Brett\";"
    );
}
