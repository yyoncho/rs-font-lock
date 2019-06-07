;;; rust-font-lock.el --- Font Lock                  -*- lexical-binding: t; -*-

;; Copyright (C) 2019  Ivan Yonchovski

;; Author: Ivan Yonchovski <yyoncho@gmail.com>
;; Keywords:

;; This program is free software; you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.

;; You should have received a copy of the GNU General Public License
;; along with this program.  If not, see <https://www.gnu.org/licenses/>.

;;; Commentary:

;;

;;; Code:

comment.block
comment.block.documentation
comment.line
constant.character.escape
constant.language
constant.numeric
constant.numeric.complex
constant.numeric.complex.imaginary
constant.numeric.complex.real
constant.numeric.float
constant.numeric.float.binary
constant.numeric.float.decimal
constant.numeric.float.hexadecimal
constant.numeric.float.octal
constant.numeric.float.other
constant.numeric.integer
constant.numeric.integer.binary
constant.numeric.integer.decimal
constant.numeric.integer.hexadecimal
constant.numeric.integer.octal
constant.numeric.integer.other
constant.other
constant.other.placeholder
entity.name
entity.name.class
entity.name.class.forward-decl
entity.name.constant
entity.name.enum
entity.name.function
entity.name.function.constructor
entity.name.function.destructor
entity.name.impl
entity.name.interface
entity.name.label
entity.name.namespace
entity.name.section
entity.name.struct
entity.name.tag
entity.name.trait
entity.name.type
entity.name.union
entity.other.attribute-name
entity.other.inherited-class
invalid.deprecated
invalid.illegal
keyword.control
keyword.control.conditional
keyword.control.import
keyword.control.php
keyword.control.python
keyword.declaration
keyword.operator
keyword.operator.arithmetic
keyword.operator.assignment
keyword.operator.bitwise
keyword.operator.logical
keyword.operator.word
keyword.other
markup.bold
markup.deleted
markup.heading
markup.inserted
markup.italic
markup.list.numbered
markup.list.unnumbered
markup.other
markup.quote
markup.raw.block
markup.raw.inline
markup.underline
markup.underline.link
meta.annotation
meta.annotation.identifier
meta.annotation.parameters
meta.block
meta.braces
meta.brackets
meta.class
meta.enum
meta.function
meta.function-call
meta.function.parameters
meta.function.php
meta.function.return-type
meta.generic
meta.group
meta.impl
meta.interface
meta.interpolation
meta.namespace
meta.paragraph
meta.parens
meta.path
meta.preprocessor
meta.string
meta.struct
meta.tag
meta.toc-list
meta.trait
meta.type
meta.union
punctuation.accessor
punctuation.definition.annotation
punctuation.definition.comment
punctuation.definition.generic.begin
punctuation.definition.generic.end
punctuation.definition.keyword
punctuation.definition.string.begin
punctuation.definition.string.end
punctuation.definition.variable
punctuation.section.block.begin
punctuation.section.block.end
punctuation.section.braces.begin
punctuation.section.braces.end
punctuation.section.brackets.begin
punctuation.section.brackets.end
punctuation.section.group.begin
punctuation.section.group.end
punctuation.section.interpolation.begin
punctuation.section.interpolation.end
punctuation.section.parens.begin
punctuation.section.parens.end
punctuation.separator
punctuation.separator.continuation
punctuation.terminator
source.<em>language-suffix</em>.embedded
storage.modifier
storage.type
storage.type keyword.declaration.type
storage.type.class keyword.declaration.class
storage.type.enum keyword.declaration.enum
storage.type.function keyword.declaration.function
storage.type.impl keyword.declaration.impl
storage.type.interface keyword.declaration.interface
storage.type.struct keyword.declaration.struct
storage.type.trait keyword.declaration.trait
storage.type.union keyword.declaration.union
string.quoted.double
string.quoted.other
string.quoted.single
string.quoted.triple
string.regexp
string.unquoted
support.class
support.constant
support.function
support.module
support.type
text.html
text.html.markdown
text.xml
variable.annotation
variable.function
variable.language
variable.other
variable.other.constant
variable.other.member
variable.other.readwrite
variable.parameter


(provide 'rust-font-lock)
;;; rust-font-lock.el ends here
