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
        "\ngreeting = \"hi brett\";\n"
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
        "\ngreeting = \"hi brett\";\n"
    );
}

#[test]
fn chained_define() {
    let tokens = tokenize(
        "#define NAME HEMTT\n#define SALUTATION Mr.\n#define HI \"Hi SALUTATION NAME\"\n\ngreeting = HI;\n",
        "test_chained_define",
    )
    .unwrap();
    let defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &defines).unwrap();
    println!("{:?}", render(preprocessor.output()).export());
    assert_eq!(
        render(preprocessor.output()).export(),
        "\ngreeting = \"Hi Mr. HEMTT\";\n"
    );
}

#[test]
fn undefine() {
    let tokens = tokenize(
        r#"
#define affirmative true
value = affirmative;
#undef affirmative
#ifdef affirmative
defined = true;
#else
defined = false;
#endif
"#,
        "test_undefine",
    )
    .unwrap();
    let defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &defines).unwrap();
    println!("{:?}", render(preprocessor.output()).export());
    assert_eq!(
        render(preprocessor.output()).export(),
        "\nvalue = true;\n\ndefined = false;\n"
    );
}

#[test]
fn define_call() {
    let tokens = tokenize(
        r#"
#define SAY_HI(NAME) Hi NAME

value = "SAY_HI(HEMTT)";
"#,
        "test_undefine",
    )
    .unwrap();
    let defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &defines).unwrap();
    println!("{:?}", render(preprocessor.output()).export());
    assert_eq!(
        render(preprocessor.output()).export(),
        "\n\nvalue = \"Hi HEMTT\";\n"
    );
}
