#include <tree_sitter/parser.h>
#include <wctype.h>

enum TokenType {
  STRING_CONTENT,
  RAW_STRING_LITERAL,
  FLOAT_LITERAL,
  BLOCK_COMMENT_START,
  BLOCK_DOC_COMMENT_START,
  BLOCK_COMMENT_CONTENT,
};

void *tree_sitter_rust_external_scanner_create() { return NULL; }
void tree_sitter_rust_external_scanner_destroy(void *p) {}
void tree_sitter_rust_external_scanner_reset(void *p) {}
unsigned tree_sitter_rust_external_scanner_serialize(void *p, char *buffer) { return 0; }
void tree_sitter_rust_external_scanner_deserialize(void *p, const char *b, unsigned n) {}

static void advance(TSLexer *lexer) {
  lexer->advance(lexer, false);
}

static bool is_num_char(int32_t c) {
  return c == '_' || iswdigit(c);
}

bool tree_sitter_rust_external_scanner_scan(void *payload, TSLexer *lexer,
                                            const bool *valid_symbols) {
  if (valid_symbols[STRING_CONTENT] && !valid_symbols[FLOAT_LITERAL]) {
    bool has_content = false;
    for (;;) {
      if (lexer->lookahead == '\"' || lexer->lookahead == '\\') {
        break;
      } else if (lexer->lookahead == 0) {
        return false;
      }
      has_content = true;
      advance(lexer);
    }
    lexer->result_symbol = STRING_CONTENT;
    return has_content;
  }

  while (iswspace(lexer->lookahead)) lexer->advance(lexer, true);

  if (
    valid_symbols[RAW_STRING_LITERAL] &&
    (lexer->lookahead == 'r' || lexer->lookahead == 'b')
  ) {
    lexer->result_symbol = RAW_STRING_LITERAL;
    if (lexer->lookahead == 'b') advance(lexer);
    if (lexer->lookahead != 'r') return false;
    advance(lexer);

    unsigned opening_hash_count = 0;
    while (lexer->lookahead == '#') {
      advance(lexer);
      opening_hash_count++;
    }

    if (lexer->lookahead != '"') return false;
    advance(lexer);

    for (;;) {
      if (lexer->lookahead == 0) {
        return false;
      } else if (lexer->lookahead == '"') {
        advance(lexer);
        unsigned hash_count = 0;
        while (lexer->lookahead == '#' && hash_count < opening_hash_count) {
          advance(lexer);
          hash_count++;
        }
        if (hash_count == opening_hash_count) {
          return true;
        }
      } else {
        advance(lexer);
      }
    }
  }

  if (valid_symbols[FLOAT_LITERAL] && iswdigit(lexer->lookahead)) {
    lexer->result_symbol = FLOAT_LITERAL;

    advance(lexer);
    while (is_num_char(lexer->lookahead)) {
      advance(lexer);
    }

    bool has_fraction = false, has_exponent = false;

    if (lexer->lookahead == '.') {
      has_fraction = true;
      advance(lexer);
      if (iswalpha(lexer->lookahead)) {
          // The dot is followed by a letter: 1.max(2) => not a float but an integer
          return false;
      }

      if (lexer->lookahead == '.') {
        return false;
      }
      while (is_num_char(lexer->lookahead)) {
        advance(lexer);
      }
    }

    lexer->mark_end(lexer);

    if (lexer->lookahead == 'e' || lexer->lookahead == 'E') {
      has_exponent = true;
      advance(lexer);
      if (lexer->lookahead == '+' || lexer->lookahead == '-') {
        advance(lexer);
      }
      if (!is_num_char(lexer->lookahead)) {
        return true;
      }
      advance(lexer);
      while (is_num_char(lexer->lookahead)) {
        advance(lexer);
      }

      lexer->mark_end(lexer);
    }

    if (!has_exponent && !has_fraction) return false;

    if (lexer->lookahead != 'u' && lexer->lookahead != 'i' && lexer->lookahead != 'f') {
      return true;
    }
    advance(lexer);
    if (!iswdigit(lexer->lookahead)) {
      return true;
    }

    while (iswdigit(lexer->lookahead)) {
      advance(lexer);
    }

    lexer->mark_end(lexer);
    return true;
  }

  if (!valid_symbols[BLOCK_COMMENT_CONTENT] &&
      (valid_symbols[BLOCK_COMMENT_START] ||
       valid_symbols[BLOCK_DOC_COMMENT_START])) {
    if (lexer->lookahead == '/') {
      advance(lexer);
      if (lexer->lookahead != '*') {
        return false;
      }
      advance(lexer);
      lexer->mark_end(lexer);

      if (lexer->lookahead == '!') {
        advance(lexer);
        lexer->result_symbol = BLOCK_DOC_COMMENT_START;
        lexer->mark_end(lexer);
        return true;
      }

      if (lexer->lookahead == '*') {
        advance(lexer);
        if (lexer->lookahead != '*' && lexer->lookahead != '/') {
          lexer->mark_end(lexer);
          lexer->result_symbol = BLOCK_DOC_COMMENT_START;
          lexer->mark_end(lexer);
          return true;
        }
      }
      lexer->result_symbol = BLOCK_COMMENT_START;
      return true;
    }
  }

  if (valid_symbols[BLOCK_COMMENT_CONTENT]) {
    unsigned depth = 1;
    for (;;) {
      switch (lexer->lookahead) {
        case '/':
          advance(lexer);
          if (lexer->lookahead == '*') {
            advance(lexer);
            depth++;
          }
          break;
        case '*':
          lexer->mark_end(lexer);
          advance(lexer);
          if (lexer->lookahead == '/') {
            advance(lexer);
            if (--depth == 0) {
              lexer->result_symbol = BLOCK_COMMENT_CONTENT;
              return true;
            }
          }
          break;
        default:
          advance(lexer);
      }
    }
  }

  return false;
}
