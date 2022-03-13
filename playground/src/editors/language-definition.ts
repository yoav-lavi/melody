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
        /(<whitespace>|<space>|<newline>|<tab>|<return>|<feed>|<null>|<digit>|<word>|<vertical>|<start>|<end>|<char>|<alphabetic>|<alphanumeric>|<boundary>|<backspace>)/,
        'character',
      ],
      [/[A-Za-z]/, 'character'],
      [/\.(?:\w)+/, 'variable'],
      [/\/\*.*\*\//, 'comment'],
      [/\/\/.*/, 'comment'],
    ],
  },
};
