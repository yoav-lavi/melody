" Vim syntax file
" Language: Melody

" Usage Instructions
" Put this file in .vim/syntax/mdy.vim
" and add in your .vimrc file the next line:
" autocmd BufRead,BufNewFile *.mdy set filetype=mdy

if exists("b:current_syntax")
	finish
endif

" keywords
syntax keyword melodyKeywords of to over some any option not lazy capture match either let ahead behind

" escape characters
syntax match melodyEscapes display contained "\\[tnvfr\"']"

" strings, with double quotes and with single quotes
syntax region melodyStringDouble start=/"/ skip=/\\./ end=/"/ contains=melodyEscapes
syntax region melodyStringSingle start=/'/ skip=/\\./ end=/'/ contains=melodyEscapes
syntax region melodyStringRaw start=/`/ skip=/\\./ end=/`/ contains=melodyEscapes

" symbols
syntax region melodySymbols start=/</ end=/>/

" number literals
syntax region melodyNumber start=/\d/ end=/\s/

" comments
syntax region melodyCommentSingle start=/\/\// end=/\n/
syntax region melodyCommentMulti start=/\/\*/ end=/\*\//

highlight default link melodyKeywords Keyword
highlight default link melodyStringDouble String
highlight default link melodyStringSingle String
highlight default link melodyStringRaw String
highlight default link melodySymbols String
highlight default link melodyEscapes SpecialChar
highlight default link melodyNumber Number
highlight default link melodyCommentSingle Comment
highlight default link melodyCommentMulti Comment

let b:current_syntax = "melody"