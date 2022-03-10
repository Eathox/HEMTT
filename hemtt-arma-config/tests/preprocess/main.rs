use hemtt_arma_config::{tokenizer::*, Preprocessor};

#[test]
fn define() {
    let content = r#"
#define affirmative true
value = affirmative;
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nvalue = true;\n", rendered.export());
}

#[test]
fn nested_define() {
    let content = r#"
#define TEST 123
#define SOMETHING TEST

value = SOMETHING;
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nvalue = 123;\n", rendered.export());
}

#[test]
fn undefine() {
    let content = r#"
#define affirmative true
#undef affirmative
value = affirmative;
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nvalue = affirmative;\n", rendered.export());
}

#[test]
fn ifdef_true() {
    let content = r#"
#define affirmative true
value = affirmative;
#ifdef affirmative
defined = true;
#else
defined = false;
#endif
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nvalue = true;\ndefined = true;\n", rendered.export());
}

#[test]
fn ifdef_false() {
    let content = r#"
#ifdef affirmative
defined = true;
#else
defined = false;
#endif
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\ndefined = false;\n", rendered.export());
}

#[test]
fn ifndef_true() {
    let content = r#"
#ifndef affirmative
undefined = true;
#else
undefined = false;
#endif
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nundefined = true;\n", rendered.export());
}

#[test]
fn ifndef_false() {
    let content = r#"
#define affirmative true
value = affirmative;
#ifndef affirmative
undefined = true;
#else
undefined = false;
#endif
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nvalue = true;\nundefined = false;\n", rendered.export());
}

#[test]
fn define_function_1_word() {
    let content = r#"
#define SAY_HI(NAME) Hi NAME

value = "SAY_HI(John)";
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nvalue = \"Hi John\";\n", rendered.export());
}

#[test]
fn define_function_1_words() {
    let content = r#"
#define SAY_HI(NAME) Hi NAME

value = "SAY_HI(John Smith)";
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nvalue = \"Hi John Smith\";\n", rendered.export());
}

#[test]
fn define_function_2() {
    let content = r#"
#define SAY_HI(FIRST,LAST) Hi FIRST LAST

value = "SAY_HI(John,Smith)";
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nvalue = \"Hi John Smith\";\n", rendered.export());
}

#[test]
fn define_function_recursive_1() {
    let content = r#"
#define MR(NAME) Mr. NAME
#define SAY_HI(NAME) Hi MR(NAME)

value = "SAY_HI(John)";
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nvalue = \"Hi Mr. John\";\n", rendered.export());
}

#[test]
fn define_function_recursive_2() {
    let content = r#"
#define ADD_PERIOD(NAME) NAME.
#define MR(NAME) Mr. NAME
#define SAY_HI(NAME) Hi MR(ADD_PERIOD(NAME))

value = "SAY_HI(John)";
"#;
    let tokens = tokenize(content, "").unwrap();
    let preprocessor = Preprocessor::execute(&tokens).unwrap();
    let rendered = hemtt_arma_config::render(preprocessor.output());
    assert_eq!("\nvalue = \"Hi Mr. John.\";\n", rendered.export());
}
