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
    let mut defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &mut defines).unwrap();
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
    let mut defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &mut defines).unwrap();
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
    let mut defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &mut defines).unwrap();
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
    let mut defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &mut defines).unwrap();
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
        "test_define_call",
    )
    .unwrap();
    let mut defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &mut defines).unwrap();
    println!("{:?}", render(preprocessor.output()).export());
    assert_eq!(
        render(preprocessor.output()).export(),
        "\n\nvalue = \"Hi HEMTT\";\n"
    );
}

#[test]
fn recursive() {
    let tokens = tokenize(
        r#"
#define ADD_PERIOD(NAME) NAME.
#define MR(NAME) Mr. ADD_PERIOD(NAME)
#define SAY_HI(NAME) Hi MR(NAME)

value = "SAY_HI(HEMTT)";
"#,
        "test_recursive",
    )
    .unwrap();
    let mut defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &mut defines).unwrap();
    println!("{:?}", render(preprocessor.output()).export());
    assert_eq!(
        render(preprocessor.output()).export(),
        "\n\nvalue = \"Hi Mr. HEMTT.\";\n"
    );
}

#[test]
fn recursive2() {
    let tokens = tokenize(
        r#"
#define ADD_PERIOD(NAME) NAME.
#define MR(NAME) Mr. NAME
#define SAY_HI(NAME) Hi MR(ADD_PERIOD(NAME))

value = "SAY_HI(HEMTT)";
"#,
        "test_recursive2",
    )
    .unwrap();
    let mut defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &mut defines).unwrap();
    println!("{:?}", render(preprocessor.output()).export());
    assert_eq!(
        render(preprocessor.output()).export(),
        "\n\nvalue = \"Hi Mr. HEMTT.\";\n"
    );
}

#[test]
fn quote() {
    let tokens = tokenize(
        r#"
#define QUOTE(s) #s
value = QUOTE(HEMTT);
"#,
        "test_recursive2",
    )
    .unwrap();
    let mut defines = Defines::new();
    let preprocessor = Preprocessor::execute(&tokens, &mut defines).unwrap();
    println!("{:?}", render(preprocessor.output()).export());
    assert_eq!(
        render(preprocessor.output()).export(),
        "\nvalue = \"HEMTT\";\n"
    );
}

// #[test]
// fn very_nested() {
//     let tokens = tokenize(
//     r#"#define QUOTE(var1) #var1
// #define ARR_2(ARG1,ARG2) ARG1, ARG2
// #define DOUBLES(var1,var2) var1##_##var2
// #define TRIPLES(var1,var2,var3) var1##_##var2##_##var3
// #define ADDON test
// #define DFUNC(var1) TRIPLES(ADDON,fnc,var1)
// #define GVAR(var1) DOUBLES(ADDON,var1)
// #define QGVAR(var1) QUOTE(GVAR(var1))
// #define QQGVAR(var1) QUOTE(QGVAR(var1))

// #define GET_NUMBER(config,default) (if (isNumber (config)) then {getNumber (config)} else {default})
// #define GET_NUMBER_GREATER_ZERO(config,default) (if (0 < getNumber (config)) then {getNumber (config)} else {default})
// #define DEFAULT_FUELCARGO \
//     GET_NUMBER(\
//         configFile >> 'CfgVehicles' >> typeOf _this >> QQGVAR(fuelCargo),\
//         GET_NUMBER_GREATER_ZERO(configFile >> 'CfgVehicles' >> typeOf _this >> 'transportFuel',-1)\
//     )

// class CfgPatches {
//     class q {
//         expression = QUOTE(if (_value != DEFAULT_FUELCARGO) then {[ARR_2(_this,_value)] call DFUNC(makeSource)});
//     };
// };"#,
//     "test_recursive2",
// )
// .unwrap();
//     let mut defines = Defines::new();
//     let preprocessor = Preprocessor::execute(&tokens, &mut defines).unwrap();
//     println!("{:?}", render(preprocessor.output()).export());
//     assert_eq!(
//         render(preprocessor.output()).export(),
//         r#"

// class CfgPatches {
//     class q {
//         expression = "if (_value !=
//     (if (isNumber (
//         configFile >> 'CfgVehicles' >> typeOf _this >> ""test_fuelCargo"")) then {getNumber (
//         configFile >> 'CfgVehicles' >> typeOf _this >> ""test_fuelCargo"")} else {
//         (if (0 < getNumber (configFile >> 'CfgVehicles' >> typeOf _this >> 'transportFuel')) then {getNumber (configFile >> 'CfgVehicles' >> typeOf _this >> 'transportFuel')} else {-1})
//     })) then {[_this, _value] call test_fnc_makeSource}";
//     };
// };"#
//     );
// }
