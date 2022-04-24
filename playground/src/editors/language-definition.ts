import { languages } from 'monaco-editor';

export const languageDefinition: languages.IMonarchLanguage = {
  tokenizer: {
    root: [
      [
        /(of|capture|to|some|match|over|option|not|either|any|ahead|behind|lazy|let)/,
        'keyword',
      ],
      [/\d/, 'digit'],
      [/"(\\"|[^"\n])*"/, 'string'],
      [/'(\\'|[^'\n])*'/, 'string'],
      [/"(\\"|[^"\n])*"/, 'string'],
      [/`(\\`|[^`\n])*`/, 'string'],
      [
        /(<whitespace>|<space>|<newline>|<tab>|<return>|<feed>|<null>|<digit>|<word>|<vertical>|<start>|<end>|<char>|<alphabetic>|<alphanumeric>|<boundary>|<backspace>|<category::letter>|<category::lowercase_letter>|<category::uppercase_letter>|<category::titlecase_letter>|<category::cased_letter>|<category::modifier_letter>|<category::other_letter>|<category::mark>|<category::non_spacing_mark>|<category::spacing_combining_mark>|<category::enclosing_mark>|<category::separator>|<category::space_separator>|<category::line_separator>|<category::paragraph_separator>|<category::symbol>|<category::math_symbol>|<category::currency_symbol>|<category::modifier_symbol>|<category::other_symbol>|<category::number>|<category::decimal_digit_number>|<category::letter_number>|<category::other_number>|<category::punctuation>|<category::dash_punctuation>|<category::open_punctuation>|<category::close_punctuation>|<category::initial_punctuation>|<category::final_punctuation>|<category::connector_punctuation>|<category::other_punctuation>|<category::other>|<category::control>|<category::format>|<category::private_use>|<category::surrogate>|<category::unassigned>)/,
        'character',
      ],
      [/[A-Za-z]/, 'character'],
      [/\.(?:\w)+/, 'variable'],
      [/\/\*.*\*\//, 'comment'],
      [/\/\/.*/, 'comment'],
    ],
  },
};
