use hemtt_arma_config::tokenizer::*;

#[test]
fn word() {
    {
        let tokens = tokenize("a word", "test_word").unwrap();
        let token_names = tokens.iter().map(|t| t.token()).collect::<Vec<_>>();
        assert_eq!(
            token_names,
            vec![
                &Token::Word("a".to_string()),
                &Token::Whitespace(Whitespace::Space),
                &Token::Word("word".to_string()),
                &Token::EOI,
            ]
        );
    }
    {
        let tokens = tokenize("a_word", "test_word").unwrap();
        let token_names = tokens.iter().map(|t| t.token()).collect::<Vec<_>>();
        assert_eq!(
            token_names,
            vec![
                &Token::Word("a".to_string()),
                &Token::Underscore,
                &Token::Word("word".to_string()),
                &Token::EOI,
            ]
        );
    }
    {
        let tokens = tokenize("a2word", "test_word").unwrap();
        let token_names = tokens.iter().map(|t| t.token()).collect::<Vec<_>>();
        assert_eq!(
            token_names,
            vec![&Token::Word("a2word".to_string()), &Token::EOI,]
        );
    }
    {
        let tokens = tokenize("2word", "test_word").unwrap();
        let token_names = tokens.iter().map(|t| t.token()).collect::<Vec<_>>();
        assert_eq!(
            token_names,
            vec![
                &Token::Digit(2),
                &Token::Word("word".to_string()),
                &Token::EOI,
            ]
        );
    }
}
